use std::fs;
use std::io;

fn main() {
    let mut input: String = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let path = input.trim();

    match fs::read(path) {
        Ok(_) => {
            println!("success");
        }
        Err(_) => {
            println!("failure");
        }
    }
}
