use once_cell::sync::OnceCell;
use std::path::PathBuf;
mod init;

fn main() {
    init::init();
    println!("Hello, world!");
}
