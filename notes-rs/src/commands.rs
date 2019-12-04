use core::fmt;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::fs::{DirEntry, File};
use std::io::Write;
use std::path::PathBuf;

use chrono::Utc;

use crate::config::Config;
use crate::default_error::DefaultError;
use crate::shell::ShellHelper;

#[derive(Debug)]
pub enum Command {
    List,
    NewNote { title: String },
    EditNote { id: usize },
    Search { pattern: String },
}

pub struct CommandHandler {
    config: Config,
}

impl CommandHandler {
    pub fn new(config: Config) -> CommandHandler {
        CommandHandler { config }
    }

    pub fn apply_command(&self, command: Command) -> Result<(), DefaultError> {
        self.ensure_note_repo_exists();
        self.ensure_note_template_exists();

        match command {
            Command::List => self.list_notes(),
            Command::NewNote { title } => self.new_note(title),
            Command::EditNote { id } => self.edit_note(id),
            Command::Search { pattern } => self.search(pattern),
        }
    }

    fn list_notes(&self) -> Result<(), DefaultError> {
        let files = self.get_note_list();
        for (index, file) in files.iter().enumerate() {
            println!("{} {}", index, file.file_name().to_str().unwrap())
        }
        Ok(())
    }

    fn new_note(&self, title: String) -> Result<(), DefaultError> {
        let now = Utc::now().format("%Y-%m-%d_%H-%M-%S");
        let note_name = format!("note_{}_{}.md", now, title);
        let mut note_path = self.config.storage_directory.clone();
        note_path.push(note_name);
        fs::copy(&self.config.template_path, &note_path)?;
        self.edit_file(note_path);
        Ok(())
    }

    fn edit_note(&self, id: usize) -> Result<(), DefaultError> {
        let files: Vec<DirEntry> = self.get_note_list();
        let to_edit = files.get(id).unwrap();
        self.edit_file(to_edit.path());
        Ok(())
    }

    fn search(&self, _pattern: String) -> Result<(), DefaultError> {
        // TODO: implement
        Ok(())
    }

    fn get_note_list(&self) -> Vec<DirEntry> {
        let res: Vec<DirEntry> = fs::read_dir(&self.config.storage_directory).unwrap()
            .filter(|entry| entry.is_ok())
            .map(|entry| entry.unwrap())
            .into_iter().collect();
        res
    }

    fn edit_file(&self, file_path: PathBuf) -> Result<(), DefaultError> {
        ShellHelper::execute(format!("vim {}", file_path.to_str().unwrap()))?;
        Ok(())
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
