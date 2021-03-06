use std::path::PathBuf;
use std::env;
use crate::EXPATH;

pub(crate) fn init() {
}

pub(crate) fn get_expath() -> PathBuf {
    let expath = match env::var("EXPATH") {
        Ok(path) => PathBuf::from(path),
        Err(_) => {
            let mut home = env::home_dir().expect("Home directory not found");
            home.push(".expack");
            home
        },
    };
    expath
}
