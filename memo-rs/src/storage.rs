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
use crate::short_memo::short_memo::ShortMemo;

pub struct ShortMemoStorage {
    config: Config,
    memos: Vec<ShortMemo>,
}

impl ShortMemoStorage {
    pub fn new(config: Config) -> ShortMemoStorage {
        ShortMemoStorage {
            config,
            memos: vec![],
        }
    }

    pub fn add(&mut self, memo: ShortMemo) {
        self.memos.push(memo);
    }

    pub fn load(&mut self) -> Result<&Vec<ShortMemo>, io::Error> {
        let file_content: String = fs::read_to_string(&self.config.storage_file)?;
        let blocks = file_content
            .split("\n\n")
            .filter(|b| !String::is_empty(&b.to_string()));
        for block in blocks {
            match ShortMemo::from(block.to_string()) {
                Ok(memo) => self.memos.push(memo),
                Err(e) => (), // TODO: handle error
            };
        }
        Ok(&self.memos)
    }

    pub fn persist(&self) -> Result<(), io::Error> {
        let mut file = File::create(&self.config.storage_file)?;
        for memo in self.memos.iter() {
            writeln!(file, "{}", memo.serialize())?;
        }
        Ok(())
    }
}
