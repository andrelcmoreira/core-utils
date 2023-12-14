use std::env::args;
use std::fs::File;
use std::io::{stdin, Error, Read};

#[derive(PartialEq)]
enum Options {
    Help
}

struct CliOptions {
    files: Vec<String>,
    flags: Vec<Options>
}

impl CliOptions {
    fn new() -> Self {
        Self {
            files: Vec::new(),
            flags: Vec::new()
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

fn get_options(args: Vec<String>) -> Result<CliOptions, String> {
    let mut opts = CliOptions::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "--help" => {
                opts.flags.push(Options::Help);
                break
            },
            _ => {
                if arg.starts_with("-") {
                    let msg = format!("cat: invalid option -- \"{arg}\"\n\
                                      Try cat \"--help\" for more informations.");
                    return Err(msg)
                } else {
                    opts.files.push(arg.clone())
                }
            }
        }
    }

    Ok(opts)
}

fn cat(opts: CliOptions) {
    if opts.flags.contains(&Options::Help) {
        show_usage();
        return
    }

    if opts.files.len() > 0 {
        for file in opts.files {
            match cat_from_file(&file) {
                Ok(buf) => print!("{buf}"),
                Err(e) => println!("cat: {file}: {}", e)
            }
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

fn cat_from_file(filename: &String) -> Result<String, Error> {
    let mut buf = String::new();
    let mut file = File::open(filename)?;

    file.read_to_string(&mut buf)?;
    Ok(buf)
}

fn main() {
    let args = args().collect::<Vec<String>>();

    match get_options(args) {
        Ok(ctx) => cat(ctx),
        Err(e) => println!("{e}")
    }
}
