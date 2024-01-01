use std::env::args;
use std::fs::File;
use std::io::{stdin, Error, Read};

#[derive(PartialEq)]
enum FlagParam {
    Help,
    ShowVersion
}

#[derive(PartialEq)]
enum InputParam {
    File(String),
    Stdin
}

struct CatOptions {
    inputs: Vec<InputParam>,
    flags: Vec<FlagParam>
}

struct Cat {
    opts: CatOptions
}

impl CatOptions {
    fn new() -> Self {
        Self {
            inputs: Vec::new(),
            flags: Vec::new()
        }
    }

    fn has_flag(&self, f: FlagParam) -> bool {
        self.flags.contains(&f)
    }

    fn add_flag(&mut self, f: FlagParam) {
        self.flags.push(f)
    }

    fn add_input(&mut self, input: InputParam) {
        self.inputs.push(input)
    }
}

impl Cat {
    fn new(opts: CatOptions) -> Self {
        Self {
            opts
        }
    }

    fn read_file(filename: &String) -> Result<String, Error> {
        let mut buf = String::new();
        let mut file = File::open(filename)?;

        file.read_to_string(&mut buf)?;
        Ok(buf)
    }

    fn read_stdin() {
        let mut buf = String::new();

        loop {
            stdin().read_line(&mut buf).unwrap();

            print!("{buf}");
            buf.clear()
        }
    }

    fn run(&self) {
        if self.opts.has_flag(FlagParam::Help) {
            show_usage();
            return
        }

        if self.opts.has_flag(FlagParam::ShowVersion) {
            show_version();
            return
        }

        for opt in &self.opts.inputs {
            match &opt {
                InputParam::Stdin => Self::read_stdin(),
                InputParam::File(f) => {
                    match Self::read_file(&f) {
                        Ok(buf) => print!("{buf}"),
                        Err(e) => println!("cat: {f}: {}", e)
                    }
                }
            }
        }
    }
}

fn show_usage() {
    let usage =
        "Usage: cat [OPTION]... [FILE]...\n\
         Concatenate FILE(S) to the standard output.\n\n\
         If FILE is not specified or be - , read the standard input.\n\n\
         \t--help        display this help and exit\n\
         \t--version     output version information and exit\n\n\
         Examples:\n\
         \tcat f - g\tEmits the content of f, after the standard input, and\n\
         \t\t\tthen the content of g at the end.\n\
         \tcat\t\tCopy the standart input to the standard output.";

    println!("{usage}")
}

fn show_version() {
    println!("cat {}", env!("CARGO_PKG_VERSION"))
}

fn parse_cli_args(args: Vec<String>) -> Result<CatOptions, String> {
    let mut opts = CatOptions::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "--help" => opts.add_flag(FlagParam::Help),
            "--version" => opts.add_flag(FlagParam::ShowVersion),
            "-" => opts.add_input(InputParam::Stdin),
            _ => {
                if arg.starts_with("-") {
                    let msg = format!("cat: invalid option -- \"{arg}\"\n\
                                      Try cat \"--help\" for more informations.");
                    return Err(msg)
                } else {
                    opts.add_input(InputParam::File(arg.clone()))
                }
            }
        }
    }

    Ok(opts)
}

fn main() {
    let args = args().collect::<Vec<String>>();

    match parse_cli_args(args) {
        Ok(opts) => {
            let cat = Cat::new(opts);

            cat.run()
        },
        Err(e) => println!("{e}")
    }
}