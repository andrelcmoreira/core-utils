use std::env::args;
use std::fs::{read_dir, ReadDir};
use std::io::ErrorKind::{NotFound, PermissionDenied};
use std::path::Path;

fn show_files(files: ReadDir) {
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

fn ls_dir(path: &str) {
    match read_dir(path) {
        Ok(files) => show_files(files),
        Err(e) => match e.kind() {
            PermissionDenied =>
                println!("ls: couldn't access '{path}': {}", e.to_string()),
            NotFound =>
                println!("ls: couldn't open the directory '{path}': {}",
                         e.to_string()),
            _ => println!("ls: unknown error")
        }
    };
}

fn ls_file(path: &str) {
    println!("{}", path)
}

fn is_dir(path: &str) -> bool {
    let p = Path::new(path);

    p.is_dir()
}

fn ls(path: &str) {
    if is_dir(path) {
        ls_dir(path)
    } else {
        ls_file(path)
    }
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() == 1 {
        ls(".")
    } else {
        for i in 1..args.len() {
            ls(args[i].as_str())
        }
    }
}
