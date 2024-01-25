use crate::*;

#[test]
fn add_line_number_to_multi_line_str() {
    let mut text = "hello\nworld\nit's me\nagain\n".to_string();
    let expected = "1\thello\n2\tworld\n3\tit's me\n4\tagain\n".to_string();

    text.add_line_number();

    assert_eq!(text, expected)
}

#[test]
fn add_line_number_to_single_line_str() {
    let mut text = "hello\n".to_string();
    let expected = "1\thello\n".to_string();

    text.add_line_number();

    assert_eq!(text, expected)
}

#[test]
fn add_line_number_to_no_lf_str() {
    let mut text = "hello".to_string();
    let expected = "1\thello".to_string();

    text.add_line_number();

    assert_eq!(text, expected)
}

#[test]
fn add_line_number_to_empty_string() {
    let mut text = "".to_string();
    let expected = "".to_string();

    text.add_line_number();

    assert_eq!(text, expected)
}

#[test]
fn show_ends_to_multi_line_str() {
    let mut text = "hello\nworld\nit's me\nagain\n".to_string();
    let expected = "hello$\nworld$\nit's me$\nagain$\n".to_string();

    text.add_end_char();

    assert_eq!(text, expected)
}

#[test]
fn show_ends_to_single_line_str() {
    let mut text = "hello\n".to_string();
    let expected = "hello$\n".to_string();

    text.add_end_char();

    assert_eq!(text, expected)
}

#[test]
fn show_ends_to_no_lf_str() {
    let mut text = "hello".to_string();
    let expected = "hello".to_string();

    text.add_end_char();

    assert_eq!(text, expected)
}

#[test]
fn show_ends_to_empty_string() {
    let mut text = "".to_string();
    let expected = "".to_string();

    text.add_end_char();

    assert_eq!(text, expected)
}
