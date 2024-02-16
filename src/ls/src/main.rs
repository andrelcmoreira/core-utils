use std::env::args;
use std::fs::read_dir;
use std::io::{Error, ErrorKind};
use std::path::Path;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
enum FlagParam {
    ShowVersion
}

#[derive(Debug, PartialEq)]
enum InputParam {
    File(String),
    Directory(String)
}

struct LsOptions {
    flags: Vec<FlagParam>,
    inputs: Vec<InputParam>
}

struct Ls {
    opts: LsOptions
}

impl LsOptions {
    fn new() -> Self {
        Self {
            flags: Vec::new(),
            inputs: Vec::new()
        }
    }

    fn has_flag(&self, f: FlagParam) -> bool {
        self.flags.contains(&f)
    }

    fn add_flag(&mut self, f: FlagParam) {
        self.flags.push(f)
    }

    fn add_input(&mut self, i: InputParam) {
        self.inputs.push(i)
    }
}

impl Ls {
    fn new(opts: LsOptions) -> Self {
        Self {
            opts
        }
    }

    fn run(&self) {
        if self.opts.has_flag(FlagParam::ShowVersion) {
            show_version();
            return
        }

        // TODO: we must to print the files first
        for entry in &self.opts.inputs {
            match entry {
                InputParam::File(f) => self.ls_file(f),
                InputParam::Directory(d) => self.ls_dir(d),
            }
        }
    }

    fn ls_file(&self, path: &str) {
        if self.opts.flags.is_empty() {
            println!("{path}")
        }
    }

    fn ls_dir(&self, path: &str) {
        // TODO: improve this function
        let files = match read_dir(path) {
            Ok(f) => f,
            Err(e) => match e.kind() {
                ErrorKind::PermissionDenied =>
                    panic!("ls: couldn't access '{path}': {}", e.to_string()),
                ErrorKind::NotFound =>
                    panic!("ls: couldn't open the directory '{path}': {}",
                           e.to_string()),
                _ => panic!("ls: unknown error")
            }
        };

        for entry in files {
            let name = entry
                .unwrap()
                .file_name()
                .into_string()
                .unwrap();

            if ! name.starts_with(".") {
                print!("{}  ", name)
            }
        }

        println!("")
    }
}

fn _show_usage() {
    let usage = "";

    println!("{usage}")
}

fn show_version() {
    println!("ls {}", env!("CARGO_PKG_VERSION"))
}

fn is_dir(path: &str) -> bool {
    let p = Path::new(path);

    p.is_dir()
}

fn parse_cli_args(args: Vec<String>) -> Result<LsOptions, Error> {
    let mut opts = LsOptions::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "--version" => opts.add_flag(FlagParam::ShowVersion),
            _ => {
                if arg.starts_with("-") {
                    let m = format!("ls: not recognized option -- \"{arg}\"\n\
                                    Try ls \"--help\" for more informations.");
                    return Err(Error::new(ErrorKind::InvalidInput, m))
                } else {
                    let entry = arg.clone();

                    if is_dir(entry.as_str()) {
                        opts.add_input(InputParam::Directory(entry))
                    } else {
                        opts.add_input(InputParam::File(entry))
                    }
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
            let ls = Ls::new(opts);

            ls.run()
        },
        Err(e) => println!("{e}")
    }
}
