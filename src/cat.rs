use std::env::args;
use std::fs::File;
use std::io::{stdin, Error, Read};

#[derive(PartialEq)]
enum Options {
    Help
}

struct CatOptions {
    files: Vec<String>,
    options: Vec<Options>
}

struct Cat {
    opts: CatOptions
}

impl CatOptions {
    fn new() -> Self {
        Self {
            files: Vec::new(),
            options: Vec::new()
        }
    }

    fn has_option(&self, opt: Options) -> bool {
        self.options.contains(&opt)
    }

    fn add_option(&mut self, opt: Options) {
        self.options.push(opt)
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

    fn from_file(filename: &String) -> Result<String, Error> {
        let mut buf = String::new();
        let mut file = File::open(filename)?;

        file.read_to_string(&mut buf)?;
        Ok(buf)
    }

    fn from_stdin() {
        let mut buf = String::new();

        loop {
            stdin().read_line(&mut buf).unwrap();

            print!("{buf}");
            buf.clear()
        }
    }

    fn run(&self) {
        if self.opts.has_option(Options::Help) {
            show_usage();
            return
        }

        if self.opts.has_files() {
            for file in &self.opts.files {
                match Self::from_file(&file) {
                    Ok(buf) => print!("{buf}"),
                    Err(e) => println!("cat: {file}: {}", e)
                }
            }
        } else {
            Self::from_stdin()
        }
    }
}

fn show_usage() {
    let usage =
        "Usage: cat [OPTION]... [FILE]...\n\
         Concatenate FILE(S) to the standard output.\n\n\
         If FILE is not specified or be - , read the standard input.\n\n\
         \t--help        display this help and exit";

    println!("{usage}")
}

fn parse_cli_args(args: Vec<String>) -> Result<CatOptions, String> {
    let mut opts = CatOptions::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "--help" => {
                opts.add_option(Options::Help);
                break
            },
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
