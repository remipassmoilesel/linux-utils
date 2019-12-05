extern crate dirs;

use std::env;
use std::path::PathBuf;

pub const NOTES_STORAGE_DIRECTORY: &str = "NOTES_STORAGE_DIRECTORY";

#[derive(Debug, Clone)]
pub struct Config {
    pub storage_directory: PathBuf,
    pub template_path: PathBuf,
}

impl Config {
    pub fn new() -> Config {
        let storage_directory = Config::get_storage_path();
        let mut template_path = storage_directory.clone();
        template_path.push(".template");

        Config {
            storage_directory,
            template_path,
        }
    }

    fn get_storage_path() -> PathBuf {
        let env_path = env::var(NOTES_STORAGE_DIRECTORY).map(|path_str| PathBuf::from(path_str));
        let mut alternative = dirs::home_dir().unwrap_or(PathBuf::from("/tmp"));
        alternative.push(".notes");

        match env_path {
            Ok(repository_path) => repository_path,
            _ => alternative
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_path_from_env_var() {
        env::set_var(NOTES_STORAGE_DIRECTORY, "/path/to/dir");
        let config = Config::new();
        assert_eq!(
            config.unwrap().storage_directory,
            PathBuf::from("/path/to/dir")
        )
    }

    #[test]
    fn should_return_path_from_home() {
        env::remove_var(NOTES_STORAGE_DIRECTORY);
        let config = Config::new();
        let path_str: String = config
            .unwrap()
            .storage_directory
            .to_str()
            .unwrap()
            .to_string();
        assert!(path_str.starts_with("/home"));
        assert!(path_str.ends_with(".notes"));
    }
}
