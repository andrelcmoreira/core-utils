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
}

fn show_usage() {
    let usage = format!(
        "Usage: cat [OPTION]... [FILE]...\n\
         Concatenate FILE(S) to the standard output.\n\n\
         If FILE is not specified or be - , read the standard input.\n\n\
         \t--help        display this help and exit");

    println!("{usage}")
}

fn build_ctx(args: Vec<String>) -> ProgCtx {
    let mut ctx = ProgCtx::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "--help" => ctx.flags.push(Options::Help),
            _ => ctx.files.push(arg.clone())
        }
    }

    return ctx
}

fn cat(ctx: ProgCtx) {
    if ctx.flags.contains(&Options::Help) {
        show_usage();
        return
    }

    if ! ctx.files.is_empty() {
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

    let ctx = build_ctx(args().collect());
    cat(ctx)
}
