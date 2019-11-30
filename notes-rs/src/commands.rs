use std::error::Error;
use std::fs;
use std::path::PathBuf;

use chrono::Utc;

use crate::config::Config;
use std::fs::ReadDir;

#[derive(Debug)]
pub enum Command {
    NewNote { title: String },
    EditNote { id: i32 },
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
        match command {
            Command::NewNote { title } => self.new_note(title),
            Command::EditNote { id } => self.edit_note(id),
            Command::Search { pattern } => self.search(pattern),
        }
    }

    fn new_note(&self, title: String) -> Result<(), CommandError> {
        let now = Utc::now().format("%Y-%m-%d_%H-%M-%S");
        let note_name = format!("note_{}_{}.md", now, title);
        let mut note_path = self.config.storage_directory.clone();
        note_path.push(note_name);
        self.edit_file(note_path);
        Ok(())
    }

    fn edit_note(&self, _id: i32) -> Result<(), CommandError> {
        // TODO: implement
        let _files: Vec<ReadDir> = fs::read_dir(&self.config.storage_directory).into_iter().collect();

        Ok(())
    }

    fn search(&mut self, _pattern: String) -> Result<(), CommandError> {
        // TODO: implement
        Ok(())
    }

    fn edit_file(&self, file_path: PathBuf) -> Result<(), CommandError> {
        let exit_status = std::process::Command::new("vim")
            .args(file_path.to_str())
            .status()?;
        match exit_status.success() {
            true => Ok(()),
            false => Err(CommandError::GenericError(format!(
                "Exited with code {}",
                exit_status.code().unwrap_or(-1)
            ))),
        }
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
