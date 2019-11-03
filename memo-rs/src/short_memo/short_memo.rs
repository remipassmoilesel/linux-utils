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
use crate::short_memo::parse_error::MemoParseError;

const DATE_FORMAT: &'static str = "%Y-%m-%d %H:%M";

#[derive(Debug, PartialEq)]
pub struct ShortMemo {
    pub title: String,
    pub category: String,
    pub description: String,
    pub date: DateTime<Utc>,
}

impl ShortMemo {
    pub fn from(block: String) -> Result<ShortMemo, MemoParseError> {
        let block_lines: Vec<&str> = block
            .split("\n")
            .filter(|l| !String::is_empty(&l.to_string()))
            .collect();
        if block_lines.len() < 2 {
            return Err(MemoParseError::InvalidFormat(
                "Not enough lines".to_string(),
            ));
        }

        let first_line = Regex::new(r" *# *(\S+) *:: *(\S+) *")?;
        let second_line = Regex::new(r" *Date: *(.+)")?;

        let captures_l0 = first_line.captures(block_lines.get(0).unwrap());
        let captures_l1 = second_line.captures(block_lines.get(1).unwrap());

        if captures_l0.is_none() || captures_l1.is_none() {
            return Err(MemoParseError::InvalidFormat(
                "First two lines are invalid".to_string(),
            ));
        }

        let captures_l0 = captures_l0.unwrap();
        let captures_l1 = captures_l1.unwrap();

        let category = captures_l0
            .get(1)
            .map_or(String::from("default"), |m| m.as_str().to_string());
        let title = captures_l0
            .get(2)
            .map_or(String::from("No title"), |m| m.as_str().to_string());

        let date = captures_l1.get(1).map(|m| m.as_str()).unwrap();
        let date = Utc.datetime_from_str(date, DATE_FORMAT)?;

        let description = block_lines[2..].join("\n");

        Ok(ShortMemo {
            category,
            title,
            date,
            description,
        })
    }

    pub fn serialize(&self) -> String {
        let date = self.date.format(DATE_FORMAT);
        let formatted = format!(
            "# {} :: {}\nDate: {}\n{}\n\n",
            self.category, self.title, date, self.description
        );
        return String::from(formatted);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod data {
        pub const SAMPLE_MEMO: &'static str = "\
# category :: title
Date: 2017-09-03 01:06
Description line 1
Description line 2

";
    }

    #[test]
    fn should_serialize() {
        let memo = ShortMemo {
            title: String::from("title"),
            category: String::from("category"),
            description: String::from("Description line 1\nDescription line 2"),
            date: DateTime::parse_from_rfc3339("2017-09-03T01:06:00-00:00")
                .unwrap()
                .with_timezone(&Utc),
        };
        assert_eq!(memo.serialize(), data::SAMPLE_MEMO);
    }

    #[test]
    fn should_parse() {
        let expected = ShortMemo {
            title: String::from("title"),
            category: String::from("category"),
            description: String::from("Description line 1\nDescription line 2"),
            date: DateTime::parse_from_rfc3339("2017-09-03T01:06:00-00:00")
                .unwrap()
                .with_timezone(&Utc),
        };
        assert_eq!(
            ShortMemo::from(data::SAMPLE_MEMO.to_string()).unwrap(),
            expected
        );
    }
}
