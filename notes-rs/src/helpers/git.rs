use crate::config::Config;
use crate::helpers::default_error::DefaultError;
use crate::helpers::shell::Shell;
use std::path::PathBuf;

pub struct Git {
    shell: Shell,
}

impl Git {
    pub fn new(shell: Shell) -> Git {
        Git { shell }
    }

    pub fn init(&self) -> Result<(), DefaultError> {
        self.shell.execute_silent(format!("git init"))
    }

    pub fn add(&self, path: &PathBuf) -> Result<(), DefaultError> {
        self.shell
            .execute(format!("git add '{}'", path.to_str().unwrap()))
    }

    pub fn commit(&self, message: String) -> Result<(), DefaultError> {
        self.shell
            .execute(format!("git commit -m '{}'", message))
    }

    pub fn push(&self) -> Result<(), DefaultError> {
        self.shell.execute(format!("git push"))
    }

    pub fn pull(&self) -> Result<(), DefaultError> {
        self.shell.execute(format!("git pull"))
    }
}
