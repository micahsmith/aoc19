use std::{env, fs};

mod five;
mod four;
mod intcode;
mod one;
mod three;
mod two;

fn get_input(filename: &str) -> String {
    let err = format!("Error: could not read file with filename {}.", filename);
    fs::read_to_string(filename).expect(&err)
}

fn call_from_str(s: &str) {
    match s {
        "one" => one::start(&get_input("inputs/one")),
        "two" => two::start(&get_input("inputs/two")),
        "three" => three::start(&get_input("inputs/three")),
        "four" => four::start(),
        "five" => five::start(&get_input("inputs/five")),
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
