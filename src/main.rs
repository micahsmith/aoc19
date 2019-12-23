use std::{env, fs};

mod eight;
mod eighteen;
mod eleven;
mod fifteen;
mod five;
mod four;
mod fourteen;
mod intcode;
mod nine;
mod nineteen;
mod one;
mod seven;
mod seventeen;
mod six;
mod sixteen;
mod ten;
mod thirteen;
mod three;
mod twelve;
mod twenty;
mod twentyfive;
mod twentyfour;
mod twentyone;
mod twentythree;
mod twentytwo;
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
        "six" => six::start(&get_input("inputs/six")),
        "seven" => seven::start(&get_input("inputs/seven")),
        "eight" => eight::start(&get_input("inputs/eight")),
        "nine" => nine::start(&get_input("inputs/nine")),
        "ten" => ten::start(&get_input("inputs/ten")),
        "eleven" => eleven::start(&get_input("inputs/eleven")),
        "twelve" => twelve::start(),
        "thirteen" => thirteen::start(&get_input("inputs/thirteen")),
        "fourteen" => fourteen::start(&get_input("inputs/fourteen")),
        "fifteen" => fifteen::start(&get_input("inputs/fifteen")),
        "sixteen" => sixteen::start(&get_input("inputs/sixteen")),
        "seventeen" => seventeen::start(&get_input("inputs/seventeen")),
        "eighteen" => eighteen::start(&get_input("inputs/eighteen")),
        "nineteen" => nineteen::start(&get_input("inputs/nineteen")),
        "twenty" => twenty::start(&get_input("inputs/twenty")),
        "twentyone" => twentyone::start(&get_input("inputs/twentyone")),
        "twentytwo" => twentytwo::start(&get_input("inputs/twentytwo")),
        "twentythree" => twentythree::start(&get_input("inputs/twentythree")),
        "twentyfour" => twentyfour::start(&get_input("inputs/twentyfour")),
        "twentyfive" => twentyfive::start(&get_input("inputs/twentyfive")),
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
