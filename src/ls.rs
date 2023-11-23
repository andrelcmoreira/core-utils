use std::env::args;
use std::fs::read_dir;
use std::io::ErrorKind::{NotFound, PermissionDenied};

fn ls(path: &str) {
    let files = match read_dir(path) {
        Ok(f) => f,
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
    
    for entry in files {
        let name = entry
            .unwrap()
            .file_name()
            .into_string()
            .unwrap();

        if ! name.starts_with(".") {
            print!("{}  ", name)
        }
    }
    
    println!("")
}

fn main() {
    let args: Vec<String> = args().collect();

    ls(args[1].as_str())
}