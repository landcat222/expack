use once_cell::sync::OnceCell;
use std::path::PathBuf;
mod init;

pub(crate) static EXPATH: OnceCell<PathBuf> = OnceCell::new();

fn main() {
    init::init();
    println!("Hello, world!");
}
