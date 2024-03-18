use std::io::{Error, ErrorKind};

use crate::cat_options::CatOptions;
use crate::types::{FlagParam, InputParam};

pub fn parse_cli_args(args: Vec<String>) -> Result<CatOptions, Error> {
    let mut opts = CatOptions::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-b" | "--number-nonblank" =>
                opts.add_flag(FlagParam::NumberNonBlank),
            "-e" => {
                opts.add_flag(FlagParam::ShowNonPrinting);
                opts.add_flag(FlagParam::ShowEnds)
            },
            "-n" | "--number" => opts.add_flag(FlagParam::ShowLineNumber),
            "-s" | "--squeeze-blank" => opts.add_flag(FlagParam::SqueezeBlank),
            "-t" => {
                opts.add_flag(FlagParam::ShowNonPrinting);
                opts.add_flag(FlagParam::ShowTabs)
            },
            "-u" => {},
            "-v" | "--show-nonprinting" =>
                opts.add_flag(FlagParam::ShowNonPrinting),
            "-A" => {
                opts.add_flag(FlagParam::ShowNonPrinting);
                opts.add_flag(FlagParam::ShowEnds);
                opts.add_flag(FlagParam::ShowTabs)
            },
            "-E" => opts.add_flag(FlagParam::ShowEnds),
            "-T" | "--show-tabs" => opts.add_flag(FlagParam::ShowTabs),
            "--help" => opts.add_flag(FlagParam::ShowHelp),
            "--version" => opts.add_flag(FlagParam::ShowVersion),
            "-" => opts.add_input(InputParam::Stdin),
            _ => {
                if arg.starts_with("-") {
                    let m = format!("cat: invalid option -- \"{arg}\"\n\
                                    Try cat \"--help\" for more informations.");
                    return Err(Error::new(ErrorKind::InvalidInput, m))
                } else {
                    opts.add_input(InputParam::File(arg.clone()))
                }
            }
        }
    }

    Ok(opts)
}
