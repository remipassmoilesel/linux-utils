use std::error::Error;
use std::process::Command;

use crate::logger::Logger;

pub struct ShellHelper;

impl ShellHelper {
    pub fn execute(command: String) -> Result<(), ShellError> {
        let mut shell_command = Command::new("sh");
        shell_command.args(&["-c", command.as_str()]);
        Logger::dimmed(format!(" $ {}", command));

        let status_code = shell_command.status()?;
        match status_code.success() {
            true => Ok(()),
            false => Err(ShellError::GenericError {
                message: format!("Exited with code {}", status_code.code().unwrap_or(-1))
            }),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ShellError {
    GenericError { message: String },
}

impl From<std::io::Error> for ShellError {
    fn from(error: std::io::Error) -> ShellError {
        ShellError::GenericError {
            message: String::from(error.description())
        }
    }
}
