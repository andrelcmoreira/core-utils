use std::env::args;
use std::fs::read_dir;
use std::io::ErrorKind::{NotFound, PermissionDenied};

fn ls(path: &str) {
    match read_dir(path) {
        Ok(list) => {
            for entry in list {
                print!("{}\t", entry.unwrap().file_name().to_str().unwrap())
            }
            println!("")
        },
        Err(e) => {
            match e.kind() {
                PermissionDenied =>
                    panic!("ls: couldn't access '{}': {}", path, e.to_string()),
                NotFound =>
                    panic!("ls: couldn't open the directory '{}': {}", path,
                           e.to_string()),
                _ => panic!("ls: unknown error")
            }
        }
    };
}

fn main() {
    let args: Vec<String> = args().collect();

    ls(args[1].as_str())
}
