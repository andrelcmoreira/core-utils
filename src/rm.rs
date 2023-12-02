use std::env::args;
use std::fs::remove_file;
use std::io::ErrorKind::{PermissionDenied, NotFound};

fn rm(path: &str) {
    match remove_file(path) {
        Ok(_) => {},
        Err(e) => match e.kind() {
            NotFound =>
                println!("rm: fail to remove '{path}': {}", e.to_string()),
            PermissionDenied =>
                println!("rm: imposible to remove '{path}': {}", e.to_string()),
            _ => match e.raw_os_error().unwrap() {
                21 => // NotADirectory
                    println!("rm: fail to remove '{path}', {}", e.to_string()),
                _ =>
                    println!("rm: unknown failure to remove '{path}'", )
            }
        }
    };
}

fn main() {
    let args: Vec<String> = args().collect();

    rm(args[1].as_str());
}
