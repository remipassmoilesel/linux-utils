
use std::path::{PathBuf};
use std::process::{Command};

use crate::config::Config;
use crate::helpers::default_error::DefaultError;
use crate::helpers::log::Log;

#[derive(Clone)]
pub struct Shell {
    config: Config,
}

impl Shell {
    pub fn new(config: Config) -> Shell {
        Shell { config }
    }

    pub fn execute(&self, command: String) -> Result<(), DefaultError> {
        self.execute_internal(command, &self.config.storage_directory, true)
    }

    pub fn execute_silent(&self, command: String) -> Result<(), DefaultError> {
        self.execute_internal(format!("{}  > /dev/null", command), &self.config.storage_directory, false)
    }

    fn execute_internal(&self, command: String, current_dir: &PathBuf, log_command: bool) -> Result<(), DefaultError> {
        let mut shell_command = Command::new("sh");
        shell_command.args(&["-c", command.as_str()]);

        if log_command {
            Log::dimmed(format!(" $ {}", command));
        }

        shell_command.current_dir(current_dir);

        let status_code = shell_command.status()?;
        match status_code.success() {
            true => Ok(()),
            false => Err(DefaultError::new(format!(
                "Exited with code {}",
                status_code.code().unwrap_or(-1)
            ))),
        }
    }
}
