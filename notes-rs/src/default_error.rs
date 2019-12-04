use std::fmt::Display;
use core::fmt;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct DefaultError {
    pub message: String,
}

impl Display for DefaultError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<std::io::Error> for DefaultError {
    fn from(error: std::io::Error) -> DefaultError {
        DefaultError { message: String::from(error.description()) }
    }
}
