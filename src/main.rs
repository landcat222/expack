use std::env;
mod init;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = init::init(args[1].to_string());
    println!("Hello, world!");
}
