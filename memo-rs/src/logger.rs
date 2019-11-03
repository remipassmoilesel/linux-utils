extern crate colored;

use colored::*;

pub struct Logger;

impl Logger {

    pub fn info(message: String) {
        print!("{}\n", String::from(message).blue());
    }

    pub fn error(message: String) {
        eprint!("{}\n", String::from(message).red());
    }
}
