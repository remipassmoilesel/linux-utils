use std::error::Error;

use chrono::Utc;

use crate::config::Config;
use crate::short_memo::short_memo::ShortMemo;
use crate::storage::ShortMemoStorage;

#[derive(Debug)]
pub enum Command {
    AddMemo {
        title: String,
        description: String,
        category: Option<String>,
    },
    Search {
        pattern: String,
        category: Option<String>,
    },
    Edit,
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

pub struct CommandHandler {
    config: Config,
    storage: ShortMemoStorage,
}

impl CommandHandler {
    pub fn new(config: Config, storage: ShortMemoStorage) -> CommandHandler {
        CommandHandler { config, storage }
    }

    pub fn apply_command(&mut self, command: Command) -> Result<(), CommandError> {
        match command {
            Command::AddMemo {
                title,
                description,
                category,
            } => self.add(title, description, category),

            Command::Search { pattern, category } => self.search(pattern, category),

            Command::Edit => self.edit_storage(),
        }
    }

    fn add(
        &mut self,
        title: String,
        description: String,
        category: Option<String>,
    ) -> Result<(), CommandError> {
        let new_memo = ShortMemo {
            title,
            description,
            category: category.unwrap_or(String::from("default")),
            date: Utc::now(),
        };
        self.storage.load()?;
        self.storage.add(new_memo);
        self.storage.persist();

        Ok(())
    }

    fn search(&mut self, pattern: String, category: Option<String>) -> Result<(), CommandError> {
        let memos = self.storage.load()?;
        // TODO: implement
        Ok(())
    }

    fn edit_storage(&self) -> Result<(), CommandError> {
        let exit_status = std::process::Command::new("vim")
            .args(&[self.config.storage_file.to_str().unwrap()])
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
