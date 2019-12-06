use colored::*;

pub struct Formatter;

impl Formatter {
    pub fn note_id(id: &usize) -> String {
        format!("@{}", id.to_string()).green().to_string()
    }

    pub fn note_title(title: &String) -> String {
        format!("{}", title.cyan())
    }

    pub fn note_matches(line: &String, matched: &str) -> String {
        let highlighted = matched.yellow();
        line.replace(matched, &highlighted.to_string())
    }

    pub fn note_score(score: usize) -> String {
        format!("(Score: {})", score.to_string())
            .dimmed()
            .to_string()
    }
}
