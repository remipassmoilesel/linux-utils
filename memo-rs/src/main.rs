use ::std::process;

use argument_parser::parse_arguments;
use logger::Logger;

use crate::commands::CommandHandler;
use crate::config::Config;

mod argument_parser;
mod commands;
mod logger;
mod config;

fn main() {
    let config = get_config();
    let command_handler = CommandHandler::new(config);
    let parse_result = parse_arguments();
    match parse_result {
        Ok(command) => command_handler.apply_command(command),
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
        _ => process::exit(1)
    }
}
