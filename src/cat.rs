use std::env::args;
use std::fs::File;
use std::io::{stdin, Error, Read};

#[derive(PartialEq)]
enum Flags {
    Help,
    ReadFromInput,
    ShowVersion
}

struct CatOptions {
    files: Vec<String>,
    flags: Vec<Flags>
}

struct Cat {
    opts: CatOptions
}

impl CatOptions {
    fn new() -> Self {
        Self {
            files: Vec::new(),
            flags: Vec::new()
        }
    }

    fn _has_flag(&self, f: Flags) -> bool {
        self.flags.contains(&f)
    }

    fn add_flag(&mut self, f: Flags) {
        self.flags.push(f)
    }

    fn has_files(&self) -> bool {
        ! self.files.is_empty()
    }

    fn add_file(&mut self, file: String) {
        self.files.push(file)
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
        for flag in &self.opts.flags {
            match flag {
                Flags::Help => {
                    show_usage();
                    return
                },
                Flags::ShowVersion => {
                    show_version();
                    return
                },
                Flags::ReadFromInput => {
                    Self::read_stdin();
                    return
                }
            }
        }

        if self.opts.has_files() {
            for file in &self.opts.files {
                match Self::read_file(&file) {
                    Ok(buf) => print!("{buf}"),
                    Err(e) => println!("cat: {file}: {}", e)
                }
            }
        } else {
            Self::read_stdin()
        }
    }
}

fn show_usage() {
    let usage =
        "Usage: cat [OPTION]... [FILE]...\n\
         Concatenate FILE(S) to the standard output.\n\n\
         If FILE is not specified or be - , read the standard input.\n\n\
         \t--help        display this help and exit\n\
         \t--version     output version information and exit";

    println!("{usage}")
}

fn show_version() {
    println!("cat {}", env!("CARGO_PKG_VERSION"))
}

fn parse_cli_args(args: Vec<String>) -> Result<CatOptions, String> {
    let mut opts = CatOptions::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "--help" => opts.add_flag(Flags::Help),
            "--version" => opts.add_flag(Flags::ShowVersion),
            "-" => opts.add_flag(Flags::ReadFromInput),
            _ => {
                if arg.starts_with("-") {
                    let msg = format!("cat: invalid option -- \"{arg}\"\n\
                                      Try cat \"--help\" for more informations.");
                    return Err(msg)
                } else {
                    opts.add_file(arg.clone())
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
