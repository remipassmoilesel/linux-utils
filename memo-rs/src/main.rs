use ::std::process;

use argument_parser::parse_arguments;
use logger::Logger;

use crate::commands::CommandHandler;
use crate::config::Config;
use crate::storage::MemoStorage;

mod argument_parser;
mod commands;
mod logger;
mod config;
mod storage;

fn main() {
    let config = get_config();
    let mut storage = MemoStorage::new(config.clone());
    let mut command_handler = CommandHandler::new(config.clone(), storage);
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
