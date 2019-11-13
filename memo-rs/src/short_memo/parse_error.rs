extern crate regex;

use core::option;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};
use regex::Regex;

use crate::config::Config;
use crate::logger::Logger;

#[derive(Debug, PartialEq)]
pub enum MemoParseError {
    InvalidFormat(String),
}

impl From<regex::Error> for MemoParseError {
    fn from(error: regex::Error) -> MemoParseError {
        MemoParseError::InvalidFormat(String::from(error.description()))
    }
}

impl From<chrono::format::ParseError> for MemoParseError {
    fn from(error: chrono::format::ParseError) -> MemoParseError {
        MemoParseError::InvalidFormat(String::from(error.description()))
    }
}
