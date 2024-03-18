use std::fs::metadata;
use std::path::Path;

pub fn is_regular_file(filename: &String) -> bool {
    let data = metadata(filename)
        .unwrap();

    data.is_file()
}

pub fn is_special_file(filename: &String) -> bool {
    // TODO
    false
}

pub fn is_dir(path: &str) -> bool {
    let p = Path::new(path);

    p.is_dir()
}

pub fn is_file(path: &str) -> bool {
    let p = path.to_string();

    is_regular_file(&p) || is_special_file(&p)
}
