use std::env::current_dir;

fn pwd() {
    let cwd = current_dir().unwrap();

    println!("{}", cwd.display())
}

fn main() {
    pwd();
}
