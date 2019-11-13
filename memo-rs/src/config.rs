extern crate dirs;

use std::env;
use std::env::VarError;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub storage_file: PathBuf,
}

impl Config {
    pub fn new() -> Result<Config, String> {
        match Config::get_storage_path() {
            Ok(path) => Ok(Config { storage_file: path }),
            Err(e) => Err(e),
        }
    }

    fn get_storage_path() -> Result<PathBuf, String> {
        let env_path = env::var("MEMO_STORAGE_PATH")
            .map(|path_str| PathBuf::from(path_str));
        let mut home_path = dirs::home_dir().unwrap_or(PathBuf::from("/tmp"));

        match env_path {
            Ok(path) => return Ok(path),
            _ => {
                home_path.push(".memo-storage.txt");
                Ok(home_path)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_path_from_env_var() {
        env::set_var("MEMO_STORAGE_PATH", "/path/to/file.txt");
        let config = Config::new();
        assert_eq!(config.unwrap().storage_file, PathBuf::from("/path/to/file.txt"))
    }

    #[test]
    fn should_return_path_from_home() {
        env::remove_var("MEMO_STORAGE_PATH");
        let config = Config::new();
        let path_str: String = config.unwrap().storage_file.to_str().unwrap().to_string();
        assert!(path_str.starts_with("/home"));
        assert!(path_str.ends_with(".memo-storage.txt"));
    }
}
