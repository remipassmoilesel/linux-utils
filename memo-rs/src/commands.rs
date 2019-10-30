use crate::config::Config;

#[derive(Debug)]
pub enum MemoCommand {
    AddMemo {
        command: String,
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
}

impl CommandHandler {
    pub fn new(config: Config) -> CommandHandler {
        CommandHandler { config }
    }

    pub fn apply_command(self, command: MemoCommand) {
        match command {
            MemoCommand::AddMemo { command, description, category } => self.add(command, description, category),
            MemoCommand::Search { pattern, category } => self.search(pattern, category),
            MemoCommand::Edit => self.edit_storage(),
        }
    }

    fn add(self, command: String, description: String, category: Option<String>) {}
    fn search(self, pattern: String, category: Option<String>) {}
    fn edit_storage(self) {}
}

