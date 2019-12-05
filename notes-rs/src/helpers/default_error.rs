use core::fmt;

use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct DefaultError {
    pub message: String,
    pub backtrace: Option<String>,
}

impl DefaultError {
    pub fn new(message: String) -> DefaultError {
        DefaultError {
            message,
            backtrace: None,
        }
    }
}

impl Display for DefaultError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<std::io::Error> for DefaultError {
    fn from(error: std::io::Error) -> DefaultError {
        DefaultError {
            message: String::from(error.description()),
            backtrace: error.backtrace().map(|bt| format!("{:?}", bt)),
        }
    }
}
