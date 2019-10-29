use ::std::process;

use argument_parser::parse_arguments;
use logger::Logger;

mod logger;
mod argument_parser;

fn main() {
    let parse_result = parse_arguments();
    match parse_result {
        Ok(command) => Logger::info(format!("{:?}", command)),
        Err(error) => {
            Logger::error(format!("{}", error));
            process::exit(1);
        }
    }
}
