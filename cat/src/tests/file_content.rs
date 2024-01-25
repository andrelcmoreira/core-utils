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
fn add_end_to_multi_line_str() {
    let mut text = "hello\nworld\nit's me\nagain\n".to_string();
    let expected = "hello$\nworld$\nit's me$\nagain$\n".to_string();

    text.add_end_char();

    assert_eq!(text, expected)
}

#[test]
fn add_end_to_single_line_str() {
    let mut text = "hello\n".to_string();
    let expected = "hello$\n".to_string();

    text.add_end_char();

    assert_eq!(text, expected)
}

#[test]
fn add_end_to_no_lf_str() {
    let mut text = "hello".to_string();
    let expected = "hello".to_string();

    text.add_end_char();

    assert_eq!(text, expected)
}

#[test]
fn add_end_to_empty_string() {
    let mut text = "".to_string();
    let expected = "".to_string();

    text.add_end_char();

    assert_eq!(text, expected)
}

#[test]
fn add_cr_to_multi_line_str() {
    let mut text = "hello\r\nworld\r\nit's me\r\nagain\r\n".to_string();
    let expected = "hello^M\nworld^M\nit's me^M\nagain^M\n".to_string();

    text.add_cr();

    assert_eq!(text, expected)
}

#[test]
fn add_cr_to_str_without_cr() {
    let mut text = "hello\nworld\nit's me\nagain\n".to_string();
    let expected = "hello\nworld\nit's me\nagain\n".to_string();

    text.add_cr();

    assert_eq!(text, expected)
}

#[test]
fn add_cr_to_str_with_single_line() {
    let mut text = "hello\r\n".to_string();
    let expected = "hello^M\n".to_string();

    text.add_cr();

    assert_eq!(text, expected)
}

#[test]
fn add_cr_to_str_without_crlf() {
    let mut text = "hello".to_string();
    let expected = "hello".to_string();

    text.add_cr();

    assert_eq!(text, expected)
}

#[test]
fn add_cr_to_empty_string() {
    let mut text = "".to_string();
    let expected = "".to_string();

    text.add_cr();

    assert_eq!(text, expected)
}

#[test]
fn add_tab_char_to_str() {
    let mut text = "hello\tworld\tit's me\tagain\n".to_string();
    let expected = "hello^Iworld^Iit's me^Iagain\n".to_string();

    text.add_tabs();

    assert_eq!(text, expected)
}

#[test]
fn add_tab_to_str_without_tab() {
    let mut text = "hello".to_string();
    let expected = "hello".to_string();

    text.add_tabs();

    assert_eq!(text, expected)
}

#[test]
fn add_tab_to_empty_string() {
    let mut text = "".to_string();
    let expected = "".to_string();

    text.add_tabs();

    assert_eq!(text, expected)
}
