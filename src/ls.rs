use std::env::args;
use std::fs::read_dir;
use std::io::ErrorKind::{NotFound, PermissionDenied};
use std::process::exit;

fn ls(path: &str) {
    let files = match read_dir(path) {
        Ok(f) => f,
        Err(e) => {
            match e.kind() {
                PermissionDenied =>
                    println!("ls: couldn't access '{path}': {}", e.to_string()),
                NotFound =>
                    println!("ls: couldn't open the directory '{path}': {}",
                       e.to_string()),
                _ => println!("ls: unknown error")
            };
            exit(1)
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
