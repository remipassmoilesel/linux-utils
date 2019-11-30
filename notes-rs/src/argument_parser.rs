use docopt::{Docopt, Error};
use serde::Deserialize;

use crate::commands::Command;

const USAGE: &'static str = "
Memo 🚀 🚀 🚀

Usage:
  memo add <command> <description> [--category=<cat>]
  memo search <pattern> [--category=<cat>]
  memo edit
  memo help

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, Deserialize)]
struct CommandLineArgs {
    cmd_add: bool,
    arg_command: String,
    arg_description: String,
    arg_pattern: String,
    cmd_edit: bool,
    cmd_search: bool,
    cmd_help: bool,
    flag_category: String,
}

pub fn parse_arguments() -> Result<Command, String> {
    let args: Result<CommandLineArgs, Error> = Docopt::new(USAGE).and_then(|d| d.deserialize());

    match args {
        Ok(args) => build_command(args),
        Err(error) => Err(error.to_string()),
    }
}

fn build_command(args: CommandLineArgs) -> Result<Command, String> {
    let category = match !String::is_empty(&args.flag_category) {
        true => Some(args.flag_category),
        false => None,
    };

    if args.cmd_add {
        return Ok(Command::AddMemo {
            title: args.arg_command.clone(),
            description: args.arg_description.clone(),
            category,
        });
    }

    if args.cmd_search {
        return Ok(Command::Search {
            pattern: args.arg_pattern,
            category,
        });
    }

    if args.cmd_edit {
        return Ok(Command::Edit);
    }
    Err(String::from("Bad command"))
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_command() {
        env::set_var("MEMO_STORAGE_PATH", "/path/to/file.txt");
        let config = Config::new();
        assert_eq!(config.unwrap().storage_file, PathBuf::from("/path/to/file.txt"))
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
