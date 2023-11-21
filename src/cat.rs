use std::env::args;
use std::fs::File;
use std::io::{ErrorKind, Read};

fn cat(filename: &str) {
    let mut buf = String::new();

    let mut file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound =>
                    panic!("cat: {filename}: No such file or directory"),
                ErrorKind::PermissionDenied =>
                    panic!("cat: {filename}: Permission denied"),
                _ => panic!("cat: {filename}: Unknown error")
            }
        }
    };

    file.read_to_string(&mut buf).unwrap();

    print!("{buf}")
}

fn main() {
    let args: Vec<String> = args().collect();

    cat(args[1].as_str());
}
