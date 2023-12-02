use std::env::args;
use std::fs::read_dir;
use std::io::ErrorKind::{NotFound, PermissionDenied};
use std::path::Path;

#[path = "./core/error.rs"]
mod error;
use error::set_panic_handler;

fn ls_dir(path: &str) {
    let files = match read_dir(path) {
        Ok(f) => f,
        Err(e) => match e.kind() {
            PermissionDenied =>
                panic!("ls: couldn't access '{path}': {}", e.to_string()),
            NotFound =>
                panic!("ls: couldn't open the directory '{path}': {}",
                       e.to_string()),
            _ => panic!("ls: unknown error")
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
    match is_file(path) {
        true => println!("{}", path),
        false => {
            // TODO(andrelcmoreira): standardize the error messages
            let error = "No such file or directory";
            panic!("ls: couldn't open the directory '{path}': {error}")
        }
    };
}

fn is_dir(path: &str) -> bool {
    let p = Path::new(path);

    p.is_dir()
}

fn is_file(path: &str) -> bool {
    let p = Path::new(path);

    p.is_file()
}

fn ls(paths: Vec<String>) {
    if paths.len() == 1 {
        ls_dir(".");
        return
    }

    for path in &paths[1..] {
        if is_dir(path) {
            ls_dir(path)
        } else {
            // the entry can be a file or a not existent entry
            ls_file(path)
        }
    }
}

fn main() {
    set_panic_handler();

    let args: Vec<String> = args().collect();
    ls(args)
}
