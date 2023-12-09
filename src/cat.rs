use std::env::args;
use std::fs::File;
use std::io::Read;

mod core;
use core::error::set_panic_handler;

enum CatFlags {
    NoOption,
}

struct CatCtx {
    files: Vec<String>,
    flags: Vec<CatFlags>
}

impl CatCtx {
    fn new() -> CatCtx {
        CatCtx {
            files: Vec::new(),
            flags: Vec::new()
        }
    }
}

fn build_ctx_from_args(args: Vec<String>) -> CatCtx {
    let mut ctx = CatCtx::new();

    for arg in &args[1..] {
        match arg.as_str() {
            _ => ctx.files.push(arg.clone())
        }
    }

    return ctx
}

fn cat(filename: &str) {
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

    let ctx = build_ctx_from_args(args().collect());
    for file in &ctx.files {
        cat(file.as_str());
    }
}
