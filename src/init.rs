use std::path::PathBuf;
use std::env;
use crate::EXPATH;

pub(crate) fn init() {
    EXPATH.set(get_expath()).unwrap();
}

fn get_expath() -> PathBuf {
    let expath = match env::var("EXPATH") {
        Ok(path) => PathBuf::from(path),
        Err(_) => {
            let mut home = dirs::home_dir().expect("Home directory not found");
            home.push(".expack");
            home
        },
    };
    expath
}
