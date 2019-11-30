use ::std::process;

use argument_parser::parse_arguments;
use logger::Logger;

use crate::commands::CommandHandler;
use crate::config::Config;
use crate::storage::ShortMemoStorage;

mod argument_parser;
mod commands;
mod config;
mod logger;
mod short_memo;
mod storage;

fn main() {
    let config = get_config();
    let storage = ShortMemoStorage::new(config.clone());
    let mut command_handler = CommandHandler::new(config.clone(), storage);
    let parse_result = parse_arguments();
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
