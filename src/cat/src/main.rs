use std::env::args;

use cat::cli_args::parse_cli_args;
use cat::cat::Cat;

fn main() {
    let args = args().collect::<Vec<String>>();

    match parse_cli_args(args) {
        Ok(opts) => {
            let cat = Cat::new(opts);

            cat.run()
        },
        Err(e) => println!("{e}")
    }
}
