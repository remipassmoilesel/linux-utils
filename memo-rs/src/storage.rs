extern crate regex;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use crate::config::Config;


#[derive(Debug)]
pub struct Memo {
    pub title: String,
    pub category: String,
    pub description: String,
    pub date: DateTime<Utc>,
}

impl Memo {
    fn from(block: String) -> Memo {
        let block_lines: Vec<&str> = block.split("\n").filter(|l| !String::is_empty(&l.to_string())).collect();
        let first_line = Regex::new(r" *# *(\S+) *:: *(\S+) *").unwrap();
        let second_line = Regex::new(r" *Date: *(\S+)").unwrap();

        let captures_l0 = first_line.captures(block_lines.get(0).unwrap_or(&"")).unwrap();
        let captures_l1 = second_line.captures(block_lines.get(1).unwrap_or(&"")).unwrap();

        Memo {
            category: captures_l0.get(1).map_or(String::from("default"), |m| m.as_str().to_string()),
            title: captures_l0.get(2).map_or(String::from(""), |m| m.as_str().to_string()),
            description: block_lines[2..].join("\n"),
            date: Utc::now(),
        }
    }

    fn serialize(&self) -> String {
        let formatted = format!("\n# {} :: {}\nDate: {}\n{}", self.category, self.title, self.date, self.description);
        return String::from(formatted);
    }
}

pub struct MemoStorage {
    config: Config,
    memos: Vec<Memo>,
}

impl MemoStorage {
    pub fn new(config: Config) -> MemoStorage {
        MemoStorage { config, memos: vec!() }
    }

    pub fn add(&mut self, memo: Memo) {
        self.memos.push(memo);
    }

    pub fn load(&mut self) -> Result<(), io::Error> {
        let file_content: String = fs::read_to_string(&self.config.storage_file)?;
        let blocks = file_content.split("\n\n").filter(|b| !String::is_empty(&b.to_string()));
        for block in blocks {
            self.memos.push(Memo::from(block.to_string()));
        }
        Ok(())
    }

    pub fn persist(&self) -> Result<(), io::Error> {
        let mut file = File::create(&self.config.storage_file)?;
        for memo in self.memos.iter() {
            writeln!(file, "{}", memo.serialize().clone())?;
        }
        Ok(())
    }
}
