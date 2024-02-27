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

struct LsOptions {
    flags: Vec<FlagParam>,
    files: Vec<String>,
    directories: Vec<String>
}

struct Ls {
    opts: LsOptions
}

impl LsOptions {
    fn new() -> Self {
        Self {
            flags: Vec::new(),
            files: Vec::new(),
            directories: Vec::new()
        }
    }

    fn has_flag(&self, f: FlagParam) -> bool {
        self.flags.contains(&f)
    }

    fn add_flag(&mut self, f: FlagParam) {
        self.flags.push(f)
    }

    fn add_file(&mut self, f: String) {
        self.files.push(f)
    }

    fn add_directory(&mut self, d: String) {
        self.directories.push(d)
    }
}

impl Ls {
    fn new(opts: LsOptions) -> Self {
        Self {
            opts
        }
    }

    fn run(&mut self) {
        if self.opts.has_flag(FlagParam::ShowVersion) {
            show_version();
            return
        }

        if ! self.opts.files.is_empty() {
            self.opts.files.sort();
            self.ls_files()
        }

        if ! self.opts.directories.is_empty() {
            self.opts.directories.sort();
            self.ls_dirs()
        }
    }

    fn ls_files(&self) {
        for path in &self.opts.files {
            print!("{path}\t")
        }

        println!("")
    }

    fn ls_dirs(&self) {
        // TODO: improve this function
        // FIXME: fix the showing when we have files to show as well
        for path in &self.opts.directories {
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

fn is_file(path: &str) -> bool {
    let f = Path::new(path);

    f.is_file()
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
                        opts.add_directory(entry)
                    } else if is_file(entry.as_str()) {
                        opts.add_file(entry)
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
            let mut ls = Ls::new(opts);

            ls.run()
        },
        Err(e) => println!("{e}")
    }
}
