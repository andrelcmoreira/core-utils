use utils::filesystem::*;

#[test]
fn is_regular_file_with_regular_file() {
    let file = "/etc/passwd".to_string();

    assert_eq!(is_regular_file(&file), true)
}

#[test]
fn is_regular_file_with_non_regular_file() {
    let file = "/dev/urandom".to_string();

    assert_eq!(is_regular_file(&file), false)
}

#[test]
fn is_file_with_special_file() {
    let file = "/dev/urandom".to_string();

    assert_eq!(is_file(&file), true)
}

#[test]
fn is_file_with_regular_file() {
    let file = "/etc/passwd".to_string();

    assert_eq!(is_file(&file), true)
}

#[test]
fn is_file_with_dir() {
    let file = "/etc".to_string();

    assert_eq!(is_file(&file), false)
}

#[test]
fn is_dir_with_special_file() {
    let file = "/dev/urandom".to_string();

    assert_eq!(is_dir(&file), false)
}

#[test]
fn is_dir_with_regular_file() {
    let file = "/etc/passwd".to_string();

    assert_eq!(is_dir(&file), false)
}

#[test]
fn is_dir_with_dir() {
    let dir = "/etc".to_string();

    assert_eq!(is_dir(&dir), true)
}
