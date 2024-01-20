#[cfg(test)]
mod tests {
    use crate::parse_cli_args;

    #[test]
    fn parse_cli_args_with_no_args() {
        let args = vec!["cat".to_string()];
        let ret = parse_cli_args(args).unwrap();

        assert_eq!(ret.inputs.is_empty(), true);
        assert_eq!(ret.flags.is_empty(), true)
    }
}
