use std::error::Error;
use std::fs;
use std::fs::{DirEntry, File};
use std::path::PathBuf;

use chrono::Utc;

use crate::config::Config;
use crate::shell::{ShellError, ShellHelper};
use std::io::Write;

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

    pub fn apply_command(&mut self, command: Command) -> Result<(), CommandError> {
        self.ensure_note_repo_exists();

        match command {
            Command::List => self.list_notes(),
            Command::NewNote { title } => self.new_note(title),
            Command::EditNote { id } => self.edit_note(id),
            Command::Search { pattern } => self.search(pattern),
        }
    }

    fn list_notes(&self)-> Result<(), CommandError>  {
        // TODO: implement
        Ok(())
    }

    fn new_note(&self, title: String) -> Result<(), CommandError> {
        let now = Utc::now().format("%Y-%m-%d_%H-%M-%S");
        let note_name = format!("note_{}_{}.md", now, title);
        let mut note_path = self.config.storage_directory.clone();
        note_path.push(note_name);
        self.edit_file(note_path);
        Ok(())
    }

    fn edit_note(&self, id: usize) -> Result<(), CommandError> {
        let files: Vec<DirEntry> = fs::read_dir(&self.config.storage_directory).unwrap()
            .filter(|entry| entry.is_ok())
            .map(|entry| entry.unwrap())
            .into_iter().collect();
        let to_edit = files.get(id).unwrap();
        self.edit_file(to_edit.path());
        Ok(())
    }

    fn search(&mut self, _pattern: String) -> Result<(), CommandError> {
        // TODO: implement
        Ok(())
    }

    fn edit_file(&self, file_path: PathBuf) -> Result<(), CommandError> {
        ShellHelper::execute(format!("vim {}", file_path.to_str().unwrap()))?;
        Ok(())
    }

    fn ensure_note_repo_exists(&self) -> Result<(), CommandError> {
        fs::create_dir_all(&self.config.storage_directory)?;
        let mut template = self.config.storage_directory.clone();
        template.push(".template");
        if !template.exists() {
            let mut file = File::create(&template)?;
            file.write_all(b" # Title \n\n\n Here we go !\n\n")?;
        }
        Ok(())
    }

}

#[derive(Debug, PartialEq)]
pub enum CommandError {
    GenericError(String), // TODO: improve
}

impl From<std::io::Error> for CommandError {
    fn from(error: std::io::Error) -> CommandError {
        CommandError::GenericError(String::from(error.description()))
    }
}

impl From<ShellError> for CommandError {
    fn from(error: ShellError) -> CommandError {
        let ShellError::GenericError { message } = error;
        CommandError::GenericError(message)
    }
}
