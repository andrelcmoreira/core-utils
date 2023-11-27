use std::env::args;
use std::fs::read_dir;
use std::io::ErrorKind::{NotFound, PermissionDenied};
use std::path::Path;
use std::process::exit;

fn ls_directory(path: &str) {
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

fn ls_file(path: &str) {
    println!("{}", path)
}

fn is_directory(path: &str) -> bool {
    let p = Path::new(path);

    return p.is_dir()
}

fn ls(path: &str) {
    if is_directory(path) {
        ls_directory(path)
    } else {
        ls_file(path)
    }
}

fn main() {
    let args: Vec<String> = args().collect();

    for i in 1..args.len() {
        ls(args[i].as_str())
    }
}
