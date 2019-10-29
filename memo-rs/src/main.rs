
mod argument_parser;
use argument_parser::parse_arguments;

fn main() {
    println!("Hello, world!");
    let parse_result = parse_arguments();
    match parse_result {
        Ok(command) => println!("{:?}", command),
        Err(e) => eprintln!("{:?}", e)
    }
}
