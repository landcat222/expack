use std::path::PathBuf;
use std::env;
mod init;

fn main() {
    println!("Hello, world!");
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
