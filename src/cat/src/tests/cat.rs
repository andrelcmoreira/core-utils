#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn is_regular_file_with_regular_file() {
        let opts = CatOptions::new();
        let cat = Cat::new(opts);
        let file = "/etc/passwd".to_string();

        assert_eq!(cat.is_regular_file(&file), true)
    }

    #[test]
    fn is_regular_file_with_non_regular_file() {
        let opts = CatOptions::new();
        let cat = Cat::new(opts);
        let file = "/dev/urandom".to_string();

        assert_eq!(cat.is_regular_file(&file), false)
    }

//    #[test]
//    fn read_file_with_no_opts() {
//        // TODO
//    }
//
//    #[test]
//    fn read_file_with_squeeze_blank() {
//        // TODO
//    }
//
//    #[test]
//    fn read_file_with_show_ends() {
//        // TODO
//    }
//
//    #[test]
//    fn read_file_with_add_tabs() {
//        // TODO
//    }
//
//    #[test]
//    fn read_file_with_add_cr() {
//        // TODO
//    }
//
//    #[test]
//    fn read_file_with_number_non_blank() {
//        // TODO
//    }
//
//    #[test]
//    fn read_file_with_show_line_number() {
//        // TODO
//    }
//
//    #[test]
//    fn read_file_with_show_line_number_and_number_non_blank() {
//        // TODO
//    }
}
