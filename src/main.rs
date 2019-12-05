use std::env;
use std::io::{self, Read};

mod one;
mod two;

fn get_stdin() -> String {
    let mut buffer = String::new();
    match io::stdin().read_to_string(&mut buffer) {
        Err(e) => println!("Error encountered: {}", e),
        _ => (),
    }
    return buffer
}

fn call_from_str(s: &str) {
    let input = get_stdin();

    match s {
        "one" => one::start(&input),
        "two" => two::start(&input),
        _ => println!("No matching function"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0 | 1 => println!("Too few arguments"),
        2 => call_from_str(&args[1]),
        _ => println!("Too many arguments"),
    }
}
