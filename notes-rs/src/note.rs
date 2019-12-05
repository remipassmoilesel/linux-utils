extern crate regex;

use std::path::PathBuf;

use crate::helpers::log::Log;
use regex::Regex;

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

    pub fn score(&self, needle: &String) -> usize {
        let re = Regex::new(needle).unwrap();
        let match_in_title = match re.is_match(&self.title) {
            true => 4,
            false => 0,
        };
        let match_in_body: usize = self
            .content
            .iter()
            .map(|line| match re.is_match(line) {
                true => 1,
                false => 0,
            })
            .sum();
        match_in_title + match_in_body
    }

    pub fn search_and_print(&self, needle: &String, score: &usize) -> () {
        use colored::*;
        Log::log(format!("{}", self.title.blue()));
        Log::log(format!("{}", self.content.join("\n")));
        Log::log(format!("Score: {}", score.to_string().dimmed()));
        ()
    }

    pub fn print_as_list(&self) -> () {
        use colored::*;
        Log::log(format!(
            " - {} - {}",
            self.id.to_string().green(),
            self.title
        ));
        ()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: test
}
