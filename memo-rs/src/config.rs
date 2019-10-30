extern crate dirs;

use std::env;
use std::env::VarError;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    storage_root: PathBuf
}

impl Config {
    pub fn new() -> Result<Config, String> {
        let env_path = env::var("MEMO_STORAGE_PATH").map(|path_str| PathBuf::from(path_str));
        let mut home_path = dirs::home_dir().unwrap_or(PathBuf::from("/tmp"));

        match env_path {
            Ok(path) => return Ok(Config { storage_root: path }),
            _ => {
                home_path.push(".memo-storage.txt");
                Ok(Config { storage_root: home_path })
            }
        }
    }
}
