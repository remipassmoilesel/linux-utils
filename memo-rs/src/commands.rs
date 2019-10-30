use crate::config::Config;
use crate::storage::{Memo, MemoStorage};
use chrono::{Utc};

#[derive(Debug)]
pub enum MemoCommand {
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

pub struct CommandHandler {
    config: Config,
    storage: MemoStorage,
}

impl CommandHandler {
    pub fn new(config: Config, storage: MemoStorage) -> CommandHandler {
        CommandHandler { config, storage }
    }

    pub fn apply_command(&mut self, command: MemoCommand) {
        match command {
            MemoCommand::AddMemo { title, description, category } => self.add(title, description, category),
            MemoCommand::Search { pattern, category } => self.search(pattern, category),
            MemoCommand::Edit => self.edit_storage(),
        }
    }

    fn add(&mut self, title: String, description: String, category: Option<String>) {
        let new_memo = Memo {
            title,
            description,
            category: category.unwrap_or(String::from("default")),
            date: Utc::now()
        };
        self.storage.add(new_memo);
        self.storage.persist();
    }

    fn search(&self, pattern: String, category: Option<String>) {}

    fn edit_storage(&self) {}
}

