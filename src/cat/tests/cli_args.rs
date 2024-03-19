use cat::cli_args;
use cat::cli_param;
use std::io::ErrorKind;

#[test]
fn parse_with_no_args() {
    let args = vec!["cat".to_string()];
    let ret = cli_args::parse(args).unwrap();

    assert_eq!(ret.inputs().is_empty(), true);
    assert_eq!(ret.flags().is_empty(), true)
}

#[test]
fn parse_with_hyphen_opt() {
    let args = vec!["cat".to_string(), "-".to_string()];
    let ret = cli_args::parse(args.clone()).unwrap();

    assert_eq!(ret.inputs().contains(&cli_param::Input::Stdin), true);
    assert_eq!(ret.inputs().len(), args.len() - 1);
    assert_eq!(ret.flags().is_empty(), true)
}

#[test]
fn parse_with_single_input() {
    let args = vec!["cat".to_string(), "foo".to_string()];
    let ret = cli_args::parse(args.clone()).unwrap();

    assert_eq!(ret.inputs().contains(&cli_param::Input::File("foo".to_string())), true);
    assert_eq!(ret.inputs().len(), args.len() - 1);
    assert_eq!(ret.flags().is_empty(), true)
}

#[test]
fn parse_with_multiple_inputs() {
    let args = vec!["cat".to_string(),
                    "foo".to_string(),
                    "bar".to_string(),
                    "baz".to_string()];
    let ret = cli_args::parse(args.clone()).unwrap();

    for arg in &args[1..] {
        assert_eq!(ret.inputs().contains(&cli_param::Input::File(arg.to_string())),
                   true);
    }

    assert_eq!(ret.inputs().len(), args.len() - 1);
    assert_eq!(ret.flags().is_empty(), true)
}

#[test]
fn parse_with_show_version_opt() {
    let args = vec!["cat".to_string(), "--version".to_string()];
    let ret = cli_args::parse(args.clone()).unwrap();

    assert_eq!(ret.flags().contains(&cli_param::Flag::ShowVersion), true);
    assert_eq!(ret.inputs().is_empty(), true);
    assert_eq!(ret.flags().len(), 1)
}

#[test]
fn parse_with_help_opt() {
    let args = vec!["cat".to_string(), "--help".to_string()];
    let ret = cli_args::parse(args.clone()).unwrap();

    assert_eq!(ret.flags().contains(&cli_param::Flag::ShowHelp), true);
    assert_eq!(ret.inputs().is_empty(), true);
    assert_eq!(ret.flags().len(), 1)
}

#[test]
fn parse_with_show_line_number_short_opt() {
    let args = vec!["cat".to_string(),
                    "-n".to_string(),
                    "foo".to_string()];
    let ret = cli_args::parse(args.clone()).unwrap();

    assert_eq!(ret.inputs().contains(&cli_param::Input::File("foo".to_string())), true);
    assert_eq!(ret.flags().contains(&cli_param::Flag::ShowLineNumber), true);
    assert_eq!(ret.inputs().len(), 1);
    assert_eq!(ret.flags().len(), 1)
}

#[test]
fn parse_with_show_line_number_long_opt() {
    let args = vec!["cat".to_string(),
                    "--number".to_string(),
                    "foo".to_string()];
    let ret = cli_args::parse(args.clone()).unwrap();

    assert_eq!(ret.inputs().contains(&cli_param::Input::File("foo".to_string())), true);
    assert_eq!(ret.flags().contains(&cli_param::Flag::ShowLineNumber), true);
    assert_eq!(ret.inputs().len(), 1);
    assert_eq!(ret.flags().len(), 1)
}

#[test]
fn parse_with_show_ends_opt() {
    let args = vec!["cat".to_string(),
                    "-E".to_string(),
                    "foo".to_string()];
    let ret = cli_args::parse(args.clone()).unwrap();

    assert_eq!(ret.inputs().contains(&cli_param::Input::File("foo".to_string())), true);
    assert_eq!(ret.flags().contains(&cli_param::Flag::ShowEnds), true);
    assert_eq!(ret.inputs().len(), 1);
    assert_eq!(ret.flags().len(), 1)
}

#[test]
fn parse_with_show_non_printing_short_opt() {
    let args = vec!["cat".to_string(),
                    "-v".to_string(),
                    "foo".to_string()];
    let ret = cli_args::parse(args.clone()).unwrap();

    assert_eq!(ret.inputs().contains(&cli_param::Input::File("foo".to_string())), true);
    assert_eq!(ret.flags().contains(&cli_param::Flag::ShowNonPrinting), true);
    assert_eq!(ret.inputs().len(), 1);
    assert_eq!(ret.flags().len(), 1)
}

#[test]
fn parse_with_show_non_printing_long_opt() {
    let args = vec!["cat".to_string(),
                    "--show-nonprinting".to_string(),
                    "foo".to_string()];
    let ret = cli_args::parse(args.clone()).unwrap();

    assert_eq!(ret.inputs().contains(&cli_param::Input::File("foo".to_string())), true);
    assert_eq!(ret.flags().contains(&cli_param::Flag::ShowNonPrinting), true);
    assert_eq!(ret.inputs().len(), 1);
    assert_eq!(ret.flags().len(), 1)
}

#[test]
fn parse_with_show_tabs_short_opt() {
    let args = vec!["cat".to_string(),
                    "-v".to_string(),
                    "foo".to_string()];
    let ret = cli_args::parse(args.clone()).unwrap();

    assert_eq!(ret.inputs().contains(&cli_param::Input::File("foo".to_string())), true);
    assert_eq!(ret.flags().contains(&cli_param::Flag::ShowNonPrinting), true);
    assert_eq!(ret.inputs().len(), 1);
    assert_eq!(ret.flags().len(), 1)
}

#[test]
fn parse_with_show_tabs_long_opt() {
    let args = vec!["cat".to_string(),
                    "--show-nonprinting".to_string(),
                    "foo".to_string()];
    let ret = cli_args::parse(args.clone()).unwrap();

    assert_eq!(ret.inputs().contains(&cli_param::Input::File("foo".to_string())), true);
    assert_eq!(ret.flags().contains(&cli_param::Flag::ShowNonPrinting), true);
    assert_eq!(ret.inputs().len(), 1);
    assert_eq!(ret.flags().len(), 1)
}

#[test]
fn parse_with_show_all() {
    let args = vec!["cat".to_string(),
                    "-A".to_string(),
                    "foo".to_string()];
    let ret = cli_args::parse(args.clone()).unwrap();

    assert_eq!(ret.inputs().contains(&cli_param::Input::File("foo".to_string())), true);
    assert_eq!(ret.flags().contains(&cli_param::Flag::ShowNonPrinting), true);
    assert_eq!(ret.flags().contains(&cli_param::Flag::ShowEnds), true);
    assert_eq!(ret.flags().contains(&cli_param::Flag::ShowTabs), true);
    assert_eq!(ret.inputs().len(), 1);
    assert_eq!(ret.flags().len(), 3)
}

#[test]
fn parse_with_hyphen_e_opt() {
    let args = vec!["cat".to_string(),
                    "-e".to_string(),
                    "foo".to_string()];
    let ret = cli_args::parse(args.clone()).unwrap();

    assert_eq!(ret.inputs().contains(&cli_param::Input::File("foo".to_string())), true);
    assert_eq!(ret.flags().contains(&cli_param::Flag::ShowNonPrinting), true);
    assert_eq!(ret.flags().contains(&cli_param::Flag::ShowEnds), true);
    assert_eq!(ret.inputs().len(), 1);
    assert_eq!(ret.flags().len(), 2)
}

#[test]
fn parse_with_number_non_blank_short_opt() {
    let args = vec!["cat".to_string(),
                    "-b".to_string(),
                    "foo".to_string()];
    let ret = cli_args::parse(args.clone()).unwrap();

    assert_eq!(ret.inputs().contains(&cli_param::Input::File("foo".to_string())), true);
    assert_eq!(ret.flags().contains(&cli_param::Flag::NumberNonBlank), true);
    assert_eq!(ret.inputs().len(), 1);
    assert_eq!(ret.flags().len(), 1)
}

#[test]
fn parse_with_number_non_blank_long_opt() {
    let args = vec!["cat".to_string(),
                    "--number-nonblank".to_string(),
                    "foo".to_string()];
    let ret = cli_args::parse(args.clone()).unwrap();

    assert_eq!(ret.inputs().contains(&cli_param::Input::File("foo".to_string())), true);
    assert_eq!(ret.flags().contains(&cli_param::Flag::NumberNonBlank), true);
    assert_eq!(ret.inputs().len(), 1);
    assert_eq!(ret.flags().len(), 1)
}

#[test]
fn parse_with_hyphen_u_opt() {
    let args = vec!["cat".to_string(),
                    "-u".to_string(),
                    "foo".to_string()];
    let ret = cli_args::parse(args.clone()).unwrap();

    assert_eq!(ret.inputs().contains(&cli_param::Input::File("foo".to_string())), true);
    assert_eq!(ret.inputs().len(), 1);
    assert_eq!(ret.flags().len(), 0)
}

#[test]
fn parse_with_hyphen_t_opt() {
    let args = vec!["cat".to_string(),
                    "-t".to_string(),
                    "foo".to_string()];
    let ret = cli_args::parse(args.clone()).unwrap();

    assert_eq!(ret.inputs().contains(&cli_param::Input::File("foo".to_string())), true);
    assert_eq!(ret.flags().contains(&cli_param::Flag::ShowTabs), true);
    assert_eq!(ret.flags().contains(&cli_param::Flag::ShowNonPrinting), true);
    assert_eq!(ret.inputs().len(), 1);
    assert_eq!(ret.flags().len(), 2)
}

#[test]
fn parse_with_invalid_opt() {
    let args = vec!["cat".to_string(), "-Y".to_string(), "foo".to_string()];
    let expected_msg = "cat: invalid option -- \"-Y\"\n\
         Try cat \"--help\" for more informations.";
    let ret = cli_args::parse(args).unwrap_err();

    assert_eq!(ret.kind(), ErrorKind::InvalidInput);
    assert_eq!(ret.to_string(), expected_msg.to_string());
}

#[test]
fn parse_with_squeeze_blank_short_opt() {
    let args = vec!["cat".to_string(),
                    "-s".to_string(),
                    "foo".to_string()];
    let ret = cli_args::parse(args.clone()).unwrap();

    assert_eq!(ret.inputs().contains(&cli_param::Input::File("foo".to_string())), true);
    assert_eq!(ret.flags().contains(&cli_param::Flag::SqueezeBlank), true);
    assert_eq!(ret.inputs().len(), 1);
    assert_eq!(ret.flags().len(), 1)
}

#[test]
fn parse_with_squeeze_blank_long_opt() {
    let args = vec!["cat".to_string(),
                    "--squeeze-blank".to_string(),
                    "foo".to_string()];
    let ret = cli_args::parse(args.clone()).unwrap();

    assert_eq!(ret.inputs().contains(&cli_param::Input::File("foo".to_string())), true);
    assert_eq!(ret.flags().contains(&cli_param::Flag::SqueezeBlank), true);
    assert_eq!(ret.inputs().len(), 1);
    assert_eq!(ret.flags().len(), 1)
}
