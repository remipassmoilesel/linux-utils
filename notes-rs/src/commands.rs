use std::fs;



use chrono::Utc;

use crate::config::Config;
use crate::helpers::default_error::DefaultError;
use crate::helpers::log::Log;
use crate::note::Note;
use crate::repository::Repository;
use crate::usage::USAGE;

#[derive(Debug)]
pub enum Command {
    List,
    NewNote { title: String },
    EditNote { id: usize },
    Search { needle: String },
    Help,
}

pub struct CommandHandler {
    config: Config,
    repository: Repository,
}

impl CommandHandler {
    pub fn new(config: Config, repository: Repository) -> CommandHandler {
        CommandHandler { config, repository }
    }

    pub fn apply_command(&self, command: Command) -> Result<(), DefaultError> {
        match command {
            Command::NewNote { title } => self.new_note(title),
            Command::Search { needle } => self.search(needle),
            Command::EditNote { id } => self.edit_note(id),
            Command::List => self.list_notes(),
            Command::Help => self.help(),
        }
    }

    fn new_note(&self, title: String) -> Result<(), DefaultError> {
        let now = Utc::now().format("%Y-%m-%d_%H-%M-%S");
        let note_name = format!("note_{}_{}.md", now, title);
        let mut note_path = self.config.storage_directory.clone();
        note_path.push(note_name);
        fs::copy(&self.config.template_path, &note_path)?;
        self.repository.edit_file(&note_path)
    }

    fn search(&self, needle: String) -> Result<(), DefaultError> {
        let notes: Vec<Note> = self.repository.get_note_list();
        let mut scored: Vec<(usize, &Note)> = notes
            .iter()
            .map(|note| (note.score(&needle), note))
            .filter(|(score, _)| score.ne(&0))
            .collect();
        scored.sort_by(|(score_a, _), (score_b, _)| score_b.cmp(&score_a));
        scored
            .iter()
            .map(|(score, note)| note.format_for_search(&needle, *score))
            .for_each(|search_result| Log::log(format!("{}", search_result)));

        if scored.is_empty() {
            Log::info(format!("Nothing found for: {}", needle));
        }
        Ok(())
    }

    fn edit_note(&self, id: usize) -> Result<(), DefaultError> {
        let notes: Vec<Note> = self.repository.get_note_list();
        let to_edit = notes.get(id).unwrap();
        self.repository.edit_file(&to_edit.path)
    }

    fn list_notes(&self) -> Result<(), DefaultError> {
        let files = self.repository.get_note_list();
        for file in files {
            Log::log(format!("{}", file.format_for_list()));
        }
        Ok(())
    }

    fn help(&self) -> Result<(), DefaultError> {
        Log::banner();
        Log::log(format!("{}", USAGE.to_string()));
        Ok(())
    }
}
