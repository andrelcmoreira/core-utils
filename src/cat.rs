use std::env::args;
use std::fs::File;
use std::io::Read;
use std::panic::set_hook;

fn set_panic_handler() {
    set_hook(Box::new(|info| {
        if let Some(s) = info.payload().downcast_ref::<String>() {
            println!("{}", s);
        }
    }));
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

    let args: Vec<String> = args().collect();
    for arg in &args[1..] {
        cat(arg.as_str());
    }
}
