#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn parse_with_show_version() {
        let args = vec!["ls".to_string(), "--version".to_string()];
        let ret = parse_cli_args(args.clone()).unwrap();

        assert_eq!(ret.flags.contains(&FlagParam::ShowVersion), true);
        assert_eq!(ret.flags.len(), 1)
    }
}
