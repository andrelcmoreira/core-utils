#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn parse_with_no_args() {
        let args = vec!["cat".to_string()];
        let ret = parse_cli_args(args).unwrap();

        assert_eq!(ret.inputs.is_empty(), true);
        assert_eq!(ret.flags.is_empty(), true)
    }

    #[test]
    fn parse_with_hyphen_opt() {
        let args = vec!["cat".to_string(), "-".to_string()];
        let ret = parse_cli_args(args.clone()).unwrap();

        assert_eq!(ret.inputs.contains(&InputParam::Stdin), true);
        assert_eq!(ret.inputs.len(), args.len() - 1);
        assert_eq!(ret.flags.is_empty(), true)
    }

    #[test]
    fn parse_with_single_input() {
        let args = vec!["cat".to_string(), "foo".to_string()];
        let ret = parse_cli_args(args.clone()).unwrap();

        assert_eq!(ret.inputs.contains(&InputParam::File("foo".to_string())), true);
        assert_eq!(ret.inputs.len(), args.len() - 1);
        assert_eq!(ret.flags.is_empty(), true)
    }

    #[test]
    fn parse_with_multiple_inputs() {
        let args = vec!["cat".to_string(),
                        "foo".to_string(),
                        "bar".to_string(),
                        "baz".to_string()];
        let ret = parse_cli_args(args.clone()).unwrap();

        for arg in &args[1..] {
            assert_eq!(ret.inputs.contains(&InputParam::File(arg.to_string())),
                       true);
        }

        assert_eq!(ret.inputs.len(), args.len() - 1);
        assert_eq!(ret.flags.is_empty(), true)
    }

    #[test]
    fn parse_with_show_version_opt() {
        let args = vec!["cat".to_string(), "--version".to_string()];
        let ret = parse_cli_args(args.clone()).unwrap();

        assert_eq!(ret.flags.contains(&FlagParam::ShowVersion), true);
        assert_eq!(ret.inputs.is_empty(), true);
        assert_eq!(ret.flags.len(), 1)
    }

    #[test]
    fn parse_with_help_opt() {
        let args = vec!["cat".to_string(), "--help".to_string()];
        let ret = parse_cli_args(args.clone()).unwrap();

        assert_eq!(ret.flags.contains(&FlagParam::Help), true);
        assert_eq!(ret.inputs.is_empty(), true);
        assert_eq!(ret.flags.len(), 1)
    }

    #[test]
    fn parse_with_show_line_number_opt() {
        let args = vec!["cat".to_string(),
                        "-n".to_string(),
                        "foo".to_string()];
        let ret = parse_cli_args(args.clone()).unwrap();

        assert_eq!(ret.inputs.contains(&InputParam::File("foo".to_string())), true);
        assert_eq!(ret.flags.contains(&FlagParam::ShowLineNumber), true);
        assert_eq!(ret.inputs.len(), 1);
        assert_eq!(ret.flags.len(), 1)
    }

    #[test]
    fn parse_with_show_ends_opt() {
        let args = vec!["cat".to_string(),
                        "-E".to_string(),
                        "foo".to_string()];
        let ret = parse_cli_args(args.clone()).unwrap();

        assert_eq!(ret.inputs.contains(&InputParam::File("foo".to_string())), true);
        assert_eq!(ret.flags.contains(&FlagParam::ShowEnds), true);
        assert_eq!(ret.inputs.len(), 1);
        assert_eq!(ret.flags.len(), 1)
    }

    #[test]
    fn parse_with_show_non_printing_opt() {
        let args = vec!["cat".to_string(),
                        "-v".to_string(),
                        "foo".to_string()];
        let ret = parse_cli_args(args.clone()).unwrap();

        assert_eq!(ret.inputs.contains(&InputParam::File("foo".to_string())), true);
        assert_eq!(ret.flags.contains(&FlagParam::ShowNonPrinting), true);
        assert_eq!(ret.inputs.len(), 1);
        assert_eq!(ret.flags.len(), 1)
    }

    #[test]
    fn parse_with_show_tabs_opt() {
        let args = vec!["cat".to_string(),
                        "-v".to_string(),
                        "foo".to_string()];
        let ret = parse_cli_args(args.clone()).unwrap();

        assert_eq!(ret.inputs.contains(&InputParam::File("foo".to_string())), true);
        assert_eq!(ret.flags.contains(&FlagParam::ShowNonPrinting), true);
        assert_eq!(ret.inputs.len(), 1);
        assert_eq!(ret.flags.len(), 1)
    }

    #[test]
    fn parse_with_show_all() {
        let args = vec!["cat".to_string(),
                        "-A".to_string(),
                        "foo".to_string()];
        let ret = parse_cli_args(args.clone()).unwrap();

        assert_eq!(ret.inputs.contains(&InputParam::File("foo".to_string())), true);
        assert_eq!(ret.flags.contains(&FlagParam::ShowNonPrinting), true);
        assert_eq!(ret.flags.contains(&FlagParam::ShowEnds), true);
        assert_eq!(ret.flags.contains(&FlagParam::ShowTabs), true);
        assert_eq!(ret.inputs.len(), 1);
        assert_eq!(ret.flags.len(), 3)
    }

    #[test]
    fn parse_with_hyphen_e() {
        let args = vec!["cat".to_string(),
                        "-e".to_string(),
                        "foo".to_string()];
        let ret = parse_cli_args(args.clone()).unwrap();

        assert_eq!(ret.inputs.contains(&InputParam::File("foo".to_string())), true);
        assert_eq!(ret.flags.contains(&FlagParam::ShowNonPrinting), true);
        assert_eq!(ret.flags.contains(&FlagParam::ShowEnds), true);
        assert_eq!(ret.inputs.len(), 1);
        assert_eq!(ret.flags.len(), 2)
    }

    #[test]
    fn parse_with_number_non_blank_short() {
        let args = vec!["cat".to_string(),
                        "-b".to_string(),
                        "foo".to_string()];
        let ret = parse_cli_args(args.clone()).unwrap();

        assert_eq!(ret.inputs.contains(&InputParam::File("foo".to_string())), true);
        assert_eq!(ret.flags.contains(&FlagParam::NumberNonBlank), true);
        assert_eq!(ret.inputs.len(), 1);
        assert_eq!(ret.flags.len(), 1)
    }

    #[test]
    fn parse_with_number_non_blank_long() {
        let args = vec!["cat".to_string(),
                        "--number-nonblank".to_string(),
                        "foo".to_string()];
        let ret = parse_cli_args(args.clone()).unwrap();

        assert_eq!(ret.inputs.contains(&InputParam::File("foo".to_string())), true);
        assert_eq!(ret.flags.contains(&FlagParam::NumberNonBlank), true);
        assert_eq!(ret.inputs.len(), 1);
        assert_eq!(ret.flags.len(), 1)
    }

    #[test]
    fn parse_with_invalid_opt() {
        let args = vec!["cat".to_string(), "-Y".to_string(), "foo".to_string()];
        let expected_msg = "cat: invalid option -- \"-Y\"\n\
             Try cat \"--help\" for more informations.";
        let ret = parse_cli_args(args).unwrap_err();

        assert_eq!(ret.kind(), ErrorKind::InvalidInput);
        assert_eq!(ret.to_string(), expected_msg.to_string());
    }
}
