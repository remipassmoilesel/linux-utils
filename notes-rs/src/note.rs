extern crate regex;


use std::io::prelude::*;
use std::path::PathBuf;

const DATE_FORMAT: &'static str = "%Y-%m-%d %H:%M";

// TODO: add date
#[derive(Debug, PartialEq)]
pub struct Note {
    pub id: usize,
    pub path: PathBuf,
    pub title: String,
    pub content: Vec<String>,
}

pub enum NoteParseError {
    InvalidFormat(String),
}

impl Note {
    pub fn from(id: usize, path: PathBuf, raw_content: String) -> Result<Note, NoteParseError> {
        let text_lines: Vec<&str> = raw_content
            .split("\n")
            .filter(|l| !String::is_empty(&l.to_string()))
            .collect();

        if text_lines.len() < 1 {
            return Err(NoteParseError::InvalidFormat(
                "Not enough lines".to_string(),
            ));
        }

        let title = text_lines.get(0).unwrap().to_string();
        let content = text_lines
            .into_iter()
            .skip(1)
            .map(|s| s.to_string())
            .collect();

        Ok(Note {
            id,
            path,
            title,
            content,
        })
    }

    pub fn contains(&self, needle: &String) -> bool {
        true
    }

    pub fn search_and_print(&self, needle: &String) -> () {
        println!("{}", self.title);
        println!("{}", self.content.join("\ns"));
        ()
    }

    pub fn print_as_list(&self) -> () {
        println!("{}: {}", self.id, self.title);
        ()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

// TODO: test
}
