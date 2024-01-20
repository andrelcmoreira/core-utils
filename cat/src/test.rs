#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn parse_cli_args_with_no_args() {
        let args = vec!["cat".to_string()];
        let ret = parse_cli_args(args).unwrap();

        assert_eq!(ret.inputs.is_empty(), true);
        assert_eq!(ret.flags.is_empty(), true)
    }

    #[test]
    fn parse_cli_args_with_hyphen_opt() {
        let args = vec!["cat".to_string(), "-".to_string()];
        let ret = parse_cli_args(args.clone()).unwrap();

        assert_eq!(ret.inputs.contains(&InputParam::Stdin), true);
        assert_eq!(ret.inputs.len(), args.len() - 1);
        assert_eq!(ret.flags.is_empty(), true)
    }

    #[test]
    fn parse_cli_args_with_single_file() {
        let args = vec!["cat".to_string(), "foo".to_string()];
        let ret = parse_cli_args(args.clone()).unwrap();

        assert_eq!(ret.inputs.contains(&InputParam::File("foo".to_string())),
                   true);
        assert_eq!(ret.inputs.len(), args.len() - 1);
        assert_eq!(ret.flags.is_empty(), true)
    }

    #[test]
    fn parse_cli_args_with_multiple_files() {
        let args = vec!["cat".to_string(),
                        "foo".to_string(),
                        "bar".to_string(),
                        "baz".to_string()];
        let ret = parse_cli_args(args.clone()).unwrap();

        for file in &args[1..] {
            assert_eq!(ret.inputs.contains(&InputParam::File(file.to_string())),
                       true);
        }

        assert_eq!(ret.inputs.len(), args.len() - 1);
        assert_eq!(ret.flags.is_empty(), true)
    }
}
