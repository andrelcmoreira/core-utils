#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn add_line_number_to_multi_line_str() {
        let mut text = "\
            hello\n\
            world\n\
            it's me\n\
            again\n".to_string();
        let expected = "\
            1\thello\n\
            2\tworld\n\
            3\tit's me\n\
            4\tagain\n".to_string();

        text.add_line_number(false);

        assert_eq!(text, expected)
    }

    #[test]
    fn add_line_number_to_multi_line_str_with_blank_lines() {
        let mut text = "\
            hello\n\
            world\n\
            \n\
            it's me\n\
            again\n".to_string();
        let expected = "\
            1\thello\n\
            2\tworld\n\
            3\t\n\
            4\tit's me\n\
            5\tagain\n".to_string();

        text.add_line_number(false);

        assert_eq!(text, expected)
    }

    #[test]
    fn add_line_number_to_single_line_str() {
        let mut text = "hello\n".to_string();
        let expected = "1\thello\n".to_string();

        text.add_line_number(false);

        assert_eq!(text, expected)
    }

    #[test]
    fn add_line_number_to_no_lf_str() {
        let mut text = "hello".to_string();
        let expected = "1\thello".to_string();

        text.add_line_number(false);

        assert_eq!(text, expected)
    }

    #[test]
    fn add_line_number_to_empty_string() {
        let mut text = "".to_string();
        let expected = "".to_string();

        text.add_line_number(false);

        assert_eq!(text, expected)
    }

    #[test]
    fn add_line_number_to_multi_line_str_with_skip_blank() {
        let mut text = "\
            hello\n\n\
            world\n\
            today\n\
            is a\n\n\
            good day\n".to_string();
        let expected = "\
            1\thello\n\n\
            2\tworld\n\
            3\ttoday\n\
            4\tis a\n\n\
            5\tgood day\n".to_string();

        text.add_line_number(true);

        assert_eq!(text, expected)
    }

    #[test]
    fn add_line_number_to_single_line_str_with_skip_blank() {
        let mut text = "hello\n".to_string();
        let expected = "1\thello\n".to_string();

        text.add_line_number(true);

        assert_eq!(text, expected)
    }

    #[test]
    fn add_line_number_to_no_lf_str_with_skip_blank() {
        let mut text = "hello".to_string();
        let expected = "1\thello".to_string();

        text.add_line_number(true);

        assert_eq!(text, expected)
    }

    #[test]
    fn add_line_number_to_empty_string_with_skip_blank() {
        let mut text = "".to_string();
        let expected = "".to_string();

        text.add_line_number(true);

        assert_eq!(text, expected)
    }

    #[test]
    fn add_end_to_multi_line_str() {
        let mut text = "\
            hello\n\
            world\n\
            it's me\n\
            again\n".to_string();
        let expected = "\
            hello$\n\
            world$\n\
            it's me$\n\
            again$\n".to_string();

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
        let mut text = "\
            hello\r\n\
            world\r\n\
            it's me\r\n\
            again\r\n".to_string();
        let expected = "\
            hello^M\n\
            world^M\n\
            it's me^M\n\
            again^M\n".to_string();

        text.add_cr();

        assert_eq!(text, expected)
    }

    #[test]
    fn add_cr_to_str_without_cr() {
        let mut text = "\
            hello\n\
            world\n\
            it's me\n\
            again\n".to_string();
        let expected = "\
            hello\n\
            world\n\
            it's me\n\
            again\n".to_string();

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
        let mut text = "\
            hello\t\
            world\t\
            it's me\t\
            again\n".to_string();
        let expected = "\
            hello^I\
            world^I\
            it's me^I\
            again\n".to_string();

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

    #[test]
    fn replace_byte_with_empty_string() {
        let mut text = "".to_string();
        let expected = "".to_string();

        text.replace_byte(0xd, "@");

        assert_eq!(text, expected)
    }

    #[test]
    fn replace_byte_with_multi_line_string() {
        let mut text = "hello\nworld!".to_string();
        let expected = "he@@o\nwor@d!".to_string();

        text.replace_byte(0x6c, "@");

        assert_eq!(text, expected)
    }

    #[test]
    fn replace_byte_with_single_line_string() {
        let mut text = "hello, world!".to_string();
        let expected = "he@@o, wor@d!".to_string();

        text.replace_byte(0x6c, "@");

        assert_eq!(text, expected)
    }

    #[test]
    fn replace_byte_with_not_existent_byte() {
        let mut text = "hello, world!".to_string();
        let expected = "hello, world!".to_string();

        text.replace_byte(0x7c, "@");

        assert_eq!(text, expected)
    }

    #[test]
    fn squeeze_multi_line_str() {
        let mut text = "\
            hello\n\
            beautiful\n\n\n\n\
            world\n\n\n\
            !!!\n".to_string();
        let expected = "\
            hello\n\
            beautiful\n\n\
            world\n\n\
            !!!\n";

        text.squeeze_blank_lines();

        assert_eq!(text, expected)
    }

    #[test]
    fn squeeze_single_line_str() {
        let mut text = "hello, world!!!".to_string();
        let expected = "hello, world!!!";

        text.squeeze_blank_lines();

        assert_eq!(text, expected)
    }

    #[test]
    fn squeeze_multi_line_str_with_no_blank_lines() {
        let mut text = "\
            hello\n\
            beautiful\n\
            world\n\
            !!!\n".to_string();
        let expected = "\
            hello\n\
            beautiful\n\
            world\n\
            !!!\n";

        text.squeeze_blank_lines();

        assert_eq!(text, expected)
    }

    #[test]
    fn squeeze_empty_str() {
        let mut text = "".to_string();
        let expected = "";

        text.squeeze_blank_lines();

        assert_eq!(text, expected)
    }
}
