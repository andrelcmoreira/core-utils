use cat::{cli_args, this};
use std::env::args;

fn main() {
    let args = args().collect::<Vec<String>>();

    match cli_args::parse(args) {
        Ok(opts) => {
            let cat = this::Cat::new(opts);

            cat.run()
        },
        Err(e) => println!("{e}")
    }
}
