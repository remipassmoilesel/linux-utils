use docopt::{Docopt, Error};
use serde::Deserialize;

use crate::argument_parser::MemoCommand::AddMemo;

const USAGE: &'static str = "
Memo 🚀 🚀 🚀

Usage:
  memo add <command> <description>
  memo search <pattern>
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
}

#[derive(Debug)]
pub enum MemoCommand {
    AddMemo {
        command: String,
        description: String,
    },
    Search {
        pattern: String,
    },
    Edit,
}

pub fn parse_arguments() -> Result<MemoCommand, String> {
    let args: Result<CommandLineArgs, Error> = Docopt::new(USAGE).and_then(|d| d.deserialize());

    match args {
        Ok(args) => build_command(args),
        Err(error) => Err(error.to_string()),
    }
}

fn build_command(args: CommandLineArgs) -> Result<MemoCommand, String> {
    if (args.cmd_add) {
        return Ok(MemoCommand::AddMemo {
            command: args.arg_command.clone(),
            description: args.arg_description.clone(),
        });
    }
    if (args.cmd_search) {
        return Ok(MemoCommand::Search {
            pattern: args.arg_pattern,
        });
    }
    if (args.cmd_edit) {
        return Ok(MemoCommand::Edit);
    }
    Err(String::from("Bad command"))
}
