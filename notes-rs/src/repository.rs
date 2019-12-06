use std::fs;
use std::fs::{DirEntry, File};
use std::io::Write;
use std::path::PathBuf;

use crate::config::Config;
use crate::helpers::default_error::DefaultError;
use crate::helpers::git::Git;
use crate::helpers::shell::Shell;
use crate::note::Note;

pub struct Repository {
    config: Config,
    shell: Shell,
    git: Git,
}

impl Repository {
    pub fn new(config: Config, shell: Shell, git: Git) -> Repository {
        Repository { config, shell, git }
    }

    pub fn init_repo(&self) -> Result<(), DefaultError> {
        self.ensure_note_repo_exists()?;
        self.ensure_note_template_exists()?;
        self.git.init()
    }

    pub fn get_note_list(&self) -> Vec<Note> {
        let dir_entries: Vec<DirEntry> = fs::read_dir(&self.config.storage_directory)
            .unwrap()
            .filter_map(Result::ok)
            .collect();

        let res = dir_entries
            .iter()
            .map(|file| {
                let id = dir_entries
                    .iter()
                    .position(|x| x.path() == file.path())
                    .unwrap();
                let path: PathBuf = file.path();
                let content = fs::read_to_string(file.path()).unwrap_or(format!(
                    "Error while reading file: {}",
                    path.to_str().unwrap()
                ));
                (id, path, content)
            })
            .map(|(id, path, content)| Note::from(id, path, content))
            .filter_map(Result::ok)
            .collect();
        res
    }

    pub fn edit_file(&self, file_path: &PathBuf) -> Result<(), DefaultError> {
        self.shell
            .execute(format!("$EDITOR {}", file_path.to_str().unwrap()))
    }

    pub fn push_repo(&self) -> Result<(), DefaultError> {
        self.git.push()
    }

    pub fn pull_repo(&self) -> Result<(), DefaultError> {
        self.git.pull()
    }

    fn ensure_note_repo_exists(&self) -> Result<(), DefaultError> {
        fs::create_dir_all(&self.config.storage_directory)?;
        Ok(())
    }

    fn ensure_note_template_exists(&self) -> Result<(), DefaultError> {
        if !self.config.template_path.exists() {
            let mut file = File::create(&self.config.template_path)?;
            file.write_all(b"# Note template\n\nHere we go !\n\n")?;
        }
        Ok(())
    }
}
