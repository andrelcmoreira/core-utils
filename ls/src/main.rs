use std::env::args;
use std::io::Error;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
enum FlagParam {
    ShowVersion
}

struct LsOptions {
    flags: Vec<FlagParam>
}

struct Ls {
    opts: LsOptions
}

impl LsOptions {
    fn new() -> Self {
        Self {
            flags: Vec::new()
        }
    }

    fn has_flag(&self, f: FlagParam) -> bool {
        self.flags.contains(&f)
    }

    fn add_flag(&mut self, f: FlagParam) {
        self.flags.push(f)
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
    }
}

fn _show_usage() {
    let usage = "";

    println!("{usage}")
}

fn show_version() {
    println!("ls {}", env!("CARGO_PKG_VERSION"))
}

fn parse_cli_args(args: Vec<String>) -> Result<LsOptions, Error> {
    let mut opts = LsOptions::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "--version" => opts.add_flag(FlagParam::ShowVersion),
            _ => {} // TODO
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
