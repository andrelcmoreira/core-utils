use utils::filesystem::is_regular_file;

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
