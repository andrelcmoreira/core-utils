use std::env::args;

use cat::{cli_args, it};

fn main() {
    let args = args().collect::<Vec<String>>();

    match cli_args::parse(args) {
        Ok(opts) => {
            let cat = it::Cat::new(opts);

            cat.run()
        },
        Err(e) => println!("{e}")
    }
}
