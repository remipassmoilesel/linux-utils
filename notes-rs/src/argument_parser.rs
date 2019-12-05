use std::env::Args;

use docopt::{Docopt, Error};
use serde::Deserialize;

use crate::commands::Command;

use crate::helpers::default_error::DefaultError;

const USAGE: &'static str = "
Notes 🚀 🚀 🚀

Usage:
  notes list
  notes new <title>
  notes edit <id>
  notes search <needle>
  notes help

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, Deserialize)]
struct CommandLineArgs {
    cmd_list: bool,
    cmd_new: bool,
    cmd_edit: bool,
    cmd_search: bool,
    cmd_help: bool,
    arg_title: String,
    arg_needle: String,
    arg_id: usize,
}

pub struct ArgumentParser;

impl ArgumentParser {
    pub fn new() -> ArgumentParser {
        ArgumentParser
    }

    pub fn parse_arguments(&self, args: Args) -> Result<Command, DefaultError> {
        let parser = Docopt::new(USAGE).unwrap().argv(args);
        let args: Result<CommandLineArgs, Error> = parser.deserialize();

        match args {
            Ok(args) => self.build_command(args),
            Err(error) => Err(DefaultError::new(error.to_string())),
        }
    }

    fn build_command(&self, args: CommandLineArgs) -> Result<Command, DefaultError> {
        if args.cmd_list {
            return Ok(Command::List);
        }

        if args.cmd_new {
            return Ok(Command::NewNote {
                title: args.arg_title.clone(),
            });
        }

        if args.cmd_edit {
            return Ok(Command::EditNote { id: args.arg_id });
        }

        if args.cmd_search {
            return Ok(Command::Search {
                needle: args.arg_needle,
            });
        }

        Err(DefaultError::new(String::from("Bad command")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_command() {
        assert_eq!(
            config.unwrap().storage_file,
            PathBuf::from("/path/to/file.txt")
        )
    }

    #[test]
    fn should_return_path_from_home() {
        env::remove_var("MEMO_STORAGE_PATH");
        let config = Config::new();
        let path_str: String = config.unwrap().storage_file.to_str().unwrap().to_string();
        assert!(path_str.starts_with("/home"));
        assert!(path_str.ends_with(".memo-storage.txt"));
    }
}
