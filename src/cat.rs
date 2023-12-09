use std::env::args;
use std::fs::File;
use std::io::{stdin, Read};

mod core;
use core::error::set_panic_handler;

enum Flags {
}

struct ProgCtx {
    files: Vec<String>,
    flags: Vec<Flags>
}

impl ProgCtx {
    fn new() -> ProgCtx {
        ProgCtx {
            files: Vec::new(),
            flags: Vec::new()
        }
    }
}

fn build_ctx(args: Vec<String>) -> ProgCtx {
    let mut ctx = ProgCtx::new();

    for arg in &args[1..] {
        match arg.as_str() {
            _ => ctx.files.push(arg.clone())
        }
    }

    return ctx
}

fn cat(ctx: ProgCtx) {
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

fn cat_from_file(filename: &str, _flags: &Vec<Flags>) {
    let mut buf = String::new();

    let mut file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => panic!("cat: {filename}: {}", e.to_string())
    };

    match file.read_to_string(&mut buf) {
        Ok(_) => print!("{buf}"),
        Err(e) => panic!("cat: {filename}: {}", e.to_string())
    };
}

fn main() {
    set_panic_handler();

    let ctx = build_ctx(args().collect());
    cat(ctx)
}
