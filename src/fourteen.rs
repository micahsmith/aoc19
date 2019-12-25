use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq)]
struct KindQuantity {
    kind: String,
    quantity: u32,
}

impl KindQuantity {
    pub fn from_str(kind_quantity: &str) -> KindQuantity {
        let split: Vec<&str> = kind_quantity.split(" ").collect();
        return KindQuantity {
            kind: String::from(split[1]),
            quantity: split[0].parse().unwrap(),
        };
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Formula {
    output: KindQuantity,
    input: Vec<KindQuantity>,
}

impl Formula {
    pub fn from_str(formula: &str) -> Formula {
        let split: Vec<&str> = formula.split("=>").collect();
        let inputs = split[0].split(","

    
    }
}

pub fn start(input: &str) {
    let formulae = parse_input(&input);
    println!("{:?}", formulae);
}

fn parse_input(input: &str) -> Vec<Formula> {
    let mut formulae = Vec::new();

    input
        .trim()
        .split("\n")
        .map(|line| {
        })
        .collect();

    return formulae;
}
