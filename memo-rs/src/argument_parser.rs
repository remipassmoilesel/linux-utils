use docopt::{Docopt, Error};
use serde::Deserialize;

use crate::commands::MemoCommand;

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

pub fn parse_arguments() -> Result<MemoCommand, String> {
    let args: Result<CommandLineArgs, Error> = Docopt::new(USAGE).and_then(|d| d.deserialize());

    match args {
        Ok(args) => build_command(args),
        Err(error) => Err(error.to_string()),
    }
}

fn build_command(args: CommandLineArgs) -> Result<MemoCommand, String> {
    let category =  Option::from(args.flag_category);

    if args.cmd_add {
        return Ok(MemoCommand::AddMemo {
            title: args.arg_command.clone(),
            description: args.arg_description.clone(),
            category,
        });
    }

    if args.cmd_search {
        return Ok(MemoCommand::Search {
            pattern: args.arg_pattern,
            category,
        });
    }

    if args.cmd_edit {
        return Ok(MemoCommand::Edit);
    }
    Err(String::from("Bad command"))
}
