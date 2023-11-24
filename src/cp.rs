use std::env::args;
use std::fs::copy;

fn cp(from: &str, to: &str) {
    match copy(from, to) {
        Ok(_) => {},
        Err(e) => match e.kind() {
            // TODO(andrelcmoreira) handle errors
            _ => println!("cp: unknown failure")
        }
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let from = args[1].as_str();
    let to = args[2].as_str();

    cp(from, to);
}
