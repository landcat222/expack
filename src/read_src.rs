use std::path::PathBuf;
use std::fs::read_to_string;

fn read_file(src: PathBuf) -> String {
    read_to_string(src.display().to_string()).expect("File not found")
}
