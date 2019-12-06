extern crate colored;

use colored::*;

pub struct Log;

impl Log {
    pub fn small_banner() {
        println!("{}", "\nNotes 🚀 🚀 🚀\n".green());
    }

    pub fn log(message: String) {
        println!("{}", message);
    }

    pub fn info(message: String) {
        print!("{}\n", String::from(message).blue());
    }

    pub fn warn(message: String) {
        print!("{}\n", String::from(message).yellow());
    }

    pub fn dimmed(message: String) {
        print!("{}\n", String::from(message).dimmed());
    }

    pub fn error(message: String) {
        eprint!("{}\n", String::from(message).red());
    }
}
