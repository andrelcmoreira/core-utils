use std::env::args;
use ::cat::{cat, cli_args};

fn main() {
    let args = args().collect::<Vec<String>>();

    match cli_args::parse(args) {
        Ok(opts) => {
            let cat = cat::Cat::new(opts);

            cat.run()
        },
        Err(e) => println!("{e}")
    }
}
