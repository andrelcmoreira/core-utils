use std::env::args;
use std::fs::File;
use std::io::Read;

fn cat(filename: &str) {
    let mut buf = String::new();

    match File::open(filename) {
        Ok(mut f) => {
            f.read_to_string(&mut buf).unwrap();
            print!("{buf}")
        },
        Err(e) => println!("cat: {filename}: {}", e.to_string())
    };
}

fn main() {
    let args: Vec<String> = args().collect();

    for arg in &args[1..] {
        cat(arg.as_str());
    }
}
