use std::path::PathBuf;
use std::fs::read_to_string;

fn read_file(src: PathBuf) -> String {
    read_to_string(src.display().to_string()).expect("File not found")
}

fn format(src: String) -> String {
    let pieces = src.split("\n");
    let mut result = String::new();
    for i in pieces {
        result = format!("{}\n{}",result,i.trim());
    }
    result
}
