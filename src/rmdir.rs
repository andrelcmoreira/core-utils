use std::env::args;
use std::fs::remove_dir;

fn rmdir(path: &str) {
    match remove_dir(path) {
        Ok(_) => {},
        Err(e) =>
            panic!("rmdir: fail to remove '{}': {path}", e.to_string())
    };
}

fn main() {
    let args: Vec<String> = args().collect();

    rmdir(args[1].as_str());
}
