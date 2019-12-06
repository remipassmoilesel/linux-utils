#![feature(backtrace)]

use ::std::process;

use crate::argument_parser::ArgumentParser;
use crate::commands::CommandHandler;
use crate::config::Config;
use crate::helpers::default_error::DefaultError;
use crate::helpers::log::Log;
use crate::repository::Repository;
use crate::helpers::shell::Shell;
use crate::helpers::git::Git;


mod argument_parser;
mod commands;
mod config;
mod helpers;
mod note;
mod repository;
mod usage;

fn main() {
    Log::small_banner();
    let result = parse_and_apply_command();
    if result.is_err() {
        terminate(result.unwrap_err())
    }
}

fn parse_and_apply_command() -> Result<(), DefaultError> {
    let config = Config::new();

    let shell = Shell::new(config.clone());
    let git = Git::new(config.clone(), shell.clone());
    let repository = Repository::new(config.clone(), shell.clone(), git);
    repository.init_repo()?;

    let command_handler = CommandHandler::new(config.clone(), repository);
    let command = ArgumentParser::new().parse_arguments(std::env::args())?;
    command_handler.apply_command(command)?;

    Ok(())
}

fn terminate(error: DefaultError) {
    Log::error(format!("{}", error));
    Log::error(format!("{}", error.backtrace.unwrap_or("".to_string())));
    process::exit(1);
}
