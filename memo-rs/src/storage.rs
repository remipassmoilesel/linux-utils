use crate::config::Config;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};

#[derive(Debug)]
pub struct Memo {
    pub title: String,
    pub category: String,
    pub description: String,
    pub date: DateTime<Utc>,
}

impl Memo {
    fn serialize(self) -> String {
        let formatted = format!("\
# {} :: {}
Date: {}
{}.", self.category, self.title, self.date, self.description);
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

    pub fn load(&self) {}

    pub fn add(&mut self, memo: Memo) {
        self.memos.push(memo);
    }

    pub fn persist(&self) {
        for memo in self.memos.iter() {
            println!("{:?}", memo)
        }
    }
}
