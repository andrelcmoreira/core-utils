use std::env::args;
use ::cat::{cat, cli_args};

fn main() {
    let args = args().collect::<Vec<String>>();

    match cli_args::parse(args) {
        Ok(opts) => {
            let instance = cat::Cat::new(opts);

            instance.run()
        },
        Err(e) => println!("{e}")
    }
}
