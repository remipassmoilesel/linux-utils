use crate::config::Config;
use crate::helpers::default_error::DefaultError;
use crate::helpers::shell::Shell;

pub struct Git {
    config: Config,
    shell: Shell,
}

impl Git {
    pub fn new(config: Config, shell: Shell) -> Git {
        Git { config, shell }
    }

    pub fn init(&self) -> Result<(), DefaultError> {
        self.shell.execute_silent(format!("git init"))
    }

    pub fn commit(&self, message: &String) -> Result<(), DefaultError> {
        // TODO: add only note path
        self.shell
            .execute(format!("git add -A && commit -m '{}'", message))
    }

    pub fn push(&self) -> Result<(), DefaultError> {
        self.shell.execute(format!("git push"))
    }

    pub fn pull(&self) -> Result<(), DefaultError> {
        self.shell.execute(format!("git pull"))
    }
}
