use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use std::fs::File;
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
    fn serialize(&self) -> String {
        let formatted = format!("\
\n\n
# {} :: {}
Date: {}
{}", self.category, self.title, self.date, self.description);
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

    pub fn load(&self) {}

    pub fn persist(&self) -> Result<(), String> {
        let mut file = match File::create(&self.config.storage_file) {
            Ok(file) => file,
            Err(err) => return Err(err.to_string()),
        };

        for memo in self.memos.iter() {
            if let Err(err) = writeln!(file, "{}", memo.serialize().clone()) {
                return Err(err.to_string());
            }
        }
        Ok(())
    }
}
