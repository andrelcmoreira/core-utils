use std::io::{Error, ErrorKind};
use crate::{this, cli_param};

pub fn parse(args: Vec<String>) -> Result<this::Options, Error> {
    let mut opts = this::Options::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-b" | "--number-nonblank" =>
                opts.add_flag(cli_param::Flag::NumberNonBlank),
            "-e" => {
                opts.add_flag(cli_param::Flag::ShowNonPrinting);
                opts.add_flag(cli_param::Flag::ShowEnds)
            },
            "-n" | "--number" =>
                opts.add_flag(cli_param::Flag::ShowLineNumber),
            "-s" | "--squeeze-blank" =>
                opts.add_flag(cli_param::Flag::SqueezeBlank),
            "-t" => {
                opts.add_flag(cli_param::Flag::ShowNonPrinting);
                opts.add_flag(cli_param::Flag::ShowTabs)
            },
            "-u" => {},
            "-v" | "--show-nonprinting" =>
                opts.add_flag(cli_param::Flag::ShowNonPrinting),
            "-A" => {
                opts.add_flag(cli_param::Flag::ShowNonPrinting);
                opts.add_flag(cli_param::Flag::ShowEnds);
                opts.add_flag(cli_param::Flag::ShowTabs)
            },
            "-E" => opts.add_flag(cli_param::Flag::ShowEnds),
            "-T" | "--show-tabs" => opts.add_flag(cli_param::Flag::ShowTabs),
            "--help" => opts.add_flag(cli_param::Flag::ShowHelp),
            "--version" => opts.add_flag(cli_param::Flag::ShowVersion),
            "-" => opts.add_input(cli_param::Input::Stdin),
            _ => {
                if arg.starts_with("-") {
                    let m = format!("cat: invalid option -- \"{arg}\"\n\
                                    Try cat \"--help\" for more informations.");
                    return Err(Error::new(ErrorKind::InvalidInput, m))
                } else {
                    opts.add_input(cli_param::Input::File(arg.clone()))
                }
            }
        }
    }

    Ok(opts)
}
