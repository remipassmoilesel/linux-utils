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
        let env_path = env::var("MEMO_STORAGE_PATH").map(|path_str| PathBuf::from(path_str));
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
