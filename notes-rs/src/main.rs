use ::std::process;

use argument_parser::parse_arguments;
use logger::Logger;

use crate::commands::CommandHandler;
use crate::config::Config;

mod argument_parser;
mod commands;
mod config;
mod logger;
mod note;
mod shell;

fn main() {
    let config = get_config();
    let mut command_handler = CommandHandler::new(config.clone());
    let parse_result = parse_arguments(std::env::args());
    match parse_result {
        Ok(command) => match command_handler.apply_command(command) {
            Err(err) => Logger::error(format!("{:?}", err)),
            Ok(()) => (),
        },
        Err(error) => {
            Logger::error(format!("{}", error));
            process::exit(1);
        }
    }
}

fn get_config() -> Config {
    let config = Config::new();
    match config {
        Ok(config) => return config,
        _ => process::exit(1),
    }
}
