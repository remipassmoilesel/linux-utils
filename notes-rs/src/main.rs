use ::std::process;

use argument_parser::parse_arguments;
use logger::Logger;

use crate::commands::CommandHandler;
use crate::config::Config;
use crate::default_error::DefaultError;

mod argument_parser;
mod commands;
mod config;
mod logger;
mod note;
mod shell;
mod default_error;

fn main() {
    let result = parse_and_apply_command();
    if result.is_err() {
        terminate(result.unwrap_err().message)
    }
}

fn parse_and_apply_command() -> Result<(), DefaultError> {
    let config = Config::new();
    let command_handler = CommandHandler::new(config.clone());
    let command = parse_arguments(std::env::args())?;
    let command_result = command_handler.apply_command(command)?;
    Ok(())
}

fn terminate(message: String) {
    Logger::error(format!("{}", message));
    process::exit(1);
}
