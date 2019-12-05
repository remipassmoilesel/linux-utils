use std::fs;
use std::fs::{DirEntry, File};
use std::io::Write;
use std::path::PathBuf;

use chrono::Utc;

use crate::config::Config;
use crate::helpers::default_error::DefaultError;

use crate::helpers::shell::ShellHelper;
use crate::note::Note;

#[derive(Debug)]
pub enum Command {
    List,
    NewNote { title: String },
    EditNote { id: usize },
    Search { needle: String },
}

pub struct CommandHandler {
    config: Config,
}

impl CommandHandler {
    pub fn new(config: Config) -> CommandHandler {
        CommandHandler { config }
    }

    pub fn apply_command(&self, command: Command) -> Result<(), DefaultError> {
        self.ensure_note_repo_exists()?;
        self.ensure_note_template_exists()?;

        match command {
            Command::NewNote { title } => self.new_note(title),
            Command::Search { needle } => self.search(needle),
            Command::EditNote { id } => self.edit_note(id),
            Command::List => self.list_notes(),
        }
    }

    fn new_note(&self, title: String) -> Result<(), DefaultError> {
        let now = Utc::now().format("%Y-%m-%d_%H-%M-%S");
        let note_name = format!("note_{}_{}.md", now, title);
        let mut note_path = self.config.storage_directory.clone();
        note_path.push(note_name);
        fs::copy(&self.config.template_path, &note_path)?;
        self.edit_file(&note_path)?;
        Ok(())
    }

    fn search(&self, needle: String) -> Result<(), DefaultError> {
        let notes: Vec<Note> = self.get_note_list();

        for note in notes {
            if note.contains(&needle) {
                note.search_and_print(&needle);
            }
        }
        Ok(())
    }

    fn edit_note(&self, id: usize) -> Result<(), DefaultError> {
        let notes: Vec<Note> = self.get_note_list();
        let to_edit = notes.get(id).unwrap();
        self.edit_file(&to_edit.path)?;
        Ok(())
    }

    fn list_notes(&self) -> Result<(), DefaultError> {
        let files = self.get_note_list();
        for file in files {
            file.print_as_list();
        }
        Ok(())
    }

    fn get_note_list(&self) -> Vec<Note> {
        let dir_entries: Vec<DirEntry> = fs::read_dir(&self.config.storage_directory).unwrap().filter_map(Result::ok).collect();

        let res = dir_entries.iter()
            .map(|file| {
                let id = dir_entries.iter().position(|x| x.path() == file.path()).unwrap();
                let path: PathBuf = file.path();
                let content = fs::read_to_string(file.path()).unwrap_or(format!("Error while reading file: {}", path.to_str().unwrap()));
                (id, path, content)
            })
            .map(|(id, path, content)| Note::from(id, path, content))
            .filter_map(Result::ok)
            .collect();
        res
    }

    fn edit_file(&self, file_path: &PathBuf) -> Result<(), DefaultError> {
        ShellHelper::execute(format!("vim {}", file_path.to_str().unwrap()))
    }

    fn ensure_note_repo_exists(&self) -> Result<(), DefaultError> {
        fs::create_dir_all(&self.config.storage_directory)?;
        Ok(())
    }

    fn ensure_note_template_exists(&self) -> Result<(), DefaultError> {
        if !self.config.template_path.exists() {
            let mut file = File::create(&self.config.template_path)?;
            file.write_all(b" # Title\n\n\nHere we go !\n\n")?;
        }
        Ok(())
    }
}
