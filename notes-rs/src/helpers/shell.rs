
use std::process::Command;

use crate::helpers::default_error::DefaultError;
use crate::helpers::logger::Logger;

pub struct ShellHelper;

impl ShellHelper {
    pub fn execute(command: String) -> Result<(), DefaultError> {
        let mut shell_command = Command::new("sh");
        shell_command.args(&["-c", command.as_str()]);
        Logger::dimmed(format!(" $ {}", command));

        let status_code = shell_command.status()?;
        match status_code.success() {
            true => Ok(()),
            false => Err(DefaultError::new(format!("Exited with code {}", status_code.code().unwrap_or(-1)))),
        }
    }
}
