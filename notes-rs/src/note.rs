extern crate regex;

use std::path::PathBuf;

use regex::{Regex, RegexBuilder};

use crate::helpers::formatter::Formatter;

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
        let needle_regex = self.build_needle_regex(needle);
        let match_in_title = match needle_regex.is_match(&self.title) {
            true => 4,
            false => 0,
        };
        let match_in_body: usize = self
            .content
            .iter()
            .map(|line| match needle_regex.is_match(line) {
                true => 1,
                false => 0,
            })
            .sum();
        match_in_title + match_in_body
    }

    pub fn format_for_search(&self, needle: &String, score: usize) -> String {
        let needle_regex = self.build_needle_regex(needle);
        let id = Formatter::note_id(&self.id);
        let title = Formatter::note_title(&self.title);
        let formatted_score = Formatter::note_score(score);

        let mut matching_lines: Vec<String> = self
            .content
            .iter()
            .map(|line| match needle_regex.captures(line) {
                Some(captures) => {
                    let matched = captures.get(1).map_or("", |m| m.as_str());
                    Formatter::note_matches(line, matched)
                }
                None => "".to_string(),
            })
            .filter(|line| !line.is_empty())
            .collect();

        if score > 0 && matching_lines.len() < 1 {
            let len = match self.content.len() < 4 {
                true => self.content.len(),
                false => 4,
            };
            matching_lines = self.content[0..len].to_vec();
        }

        format!(
            "\n{} {} {} \n\n{}",
            id,
            title,
            formatted_score,
            matching_lines.join("\n")
        )
    }

    pub fn format_for_list(&self) -> String {
        format!(
            " - {} - {}",
            Formatter::note_id(&self.id),
            Formatter::note_title(&self.title)
        )
    }

    fn build_needle_regex(&self, needle: &String) -> Regex {
        RegexBuilder::new(&format!("({})", needle))
            .case_insensitive(true)
            .build()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn score() -> () {
        let note = Note::from(
            0,
            PathBuf::from("/tmp/note-1.txt"),
            "# SSH\nA note about SSH\n".to_string(),
        )
        .ok()
        .unwrap();
        assert_eq!(note.score(&"ssh".to_string()), 5);
    }
}
