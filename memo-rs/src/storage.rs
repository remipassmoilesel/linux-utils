use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::fs;

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
        println!("--{}", block);
        Memo {
            title: String::from(""),
            category: String::from(""),
            description: String::from(""),
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

    pub fn load(&mut self) -> Result<(), io::Error>{
        let file_content: String = fs::read_to_string(&self.config.storage_file)?;
        let blocks = file_content.split("\n\n").filter(|b| !String::is_empty(&b.to_string()));
        println!("{:#?}", blocks);
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
