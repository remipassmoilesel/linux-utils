#![feature(backtrace)]

use ::std::process;

use crate::argument_parser::ArgumentParser;
use crate::commands::CommandHandler;
use crate::config::Config;
use crate::helpers::default_error::DefaultError;
use crate::helpers::logger::Logger;

mod helpers;
mod argument_parser;
mod commands;
mod config;
mod note;

fn main() {
    let result = parse_and_apply_command();
    if result.is_err() {
        terminate(result.unwrap_err())
    }
}

fn parse_and_apply_command() -> Result<(), DefaultError> {
    let config = Config::new();
    let command_handler = CommandHandler::new(config.clone());
    let command = ArgumentParser::new().parse_arguments(std::env::args())?;
    let _command_result = command_handler.apply_command(command)?;
    Ok(())
}

fn terminate(error: DefaultError) {
    Logger::error(format!("{}", error));
    Logger::error(format!("{}", error.backtrace.unwrap_or("".to_string())));
    process::exit(1);
}
