use std::env::args;
use std::fs::File;
use std::io::{stdin, Read};

mod core;
use core::error::set_panic_handler;

#[derive(PartialEq)]
enum Options {
    Help
}

struct ProgCtx {
    files: Vec<String>,
    flags: Vec<Options>
}

impl ProgCtx {
    fn new() -> ProgCtx {
        ProgCtx {
            files: Vec::new(),
            flags: Vec::new()
        }
    }

    fn has_option(&self, opt: Options) -> bool {
        self.flags.contains(&opt)
    }

    fn has_files(&self) -> bool {
        self.files.len() > 0
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

fn build_ctx(args: Vec<String>) -> Result<ProgCtx, String> {
    let mut ctx = ProgCtx::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "--help" => {
                ctx.flags.push(Options::Help);
                break
            },
            _ => {
                if arg.starts_with("-") {
                    let msg = format!("cat: invalid option -- \"{arg}\"\n\
                                      Try cat \"--help\" for more informations.");
                    return Err(msg)
                } else {
                    ctx.files.push(arg.clone())
                }
            }
        }
    }

    Ok(ctx)
}

fn cat(ctx: ProgCtx) {
    if ctx.has_option(Options::Help) {
        show_usage();
        return
    }

    if ctx.has_files() {
        for file in ctx.files {
            cat_from_file(file.as_str(), &ctx.flags);
        }
    } else {
        cat_from_stdin()
    }
}

fn cat_from_stdin() {
    let mut buf = String::new();

    loop {
        stdin().read_line(&mut buf).unwrap();

        print!("{buf}");
        buf.clear()
    }
}

fn cat_from_file(filename: &str, _flags: &Vec<Options>) {
    let mut buf = String::new();

    match File::open(filename) {
        Ok(mut f) => {
            match f.read_to_string(&mut buf) {
                Ok(_) => print!("{buf}"),
                Err(e) => panic!("cat: {filename}: {}", e.to_string())
            };
        },
        Err(e) => panic!("cat: {filename}: {}", e.to_string())
    }
}

fn main() {
    set_panic_handler();

    let args = args().collect::<Vec<String>>();
    match build_ctx(args) {
        Ok(ctx) => cat(ctx),
        Err(e) => panic!("{e}")
    }
}
