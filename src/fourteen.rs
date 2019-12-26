use std::collections::HashMap;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Order {
    kind: String,
    quantity: u32,
}

impl Order {
    pub fn from_str(kind_quantity: &str) -> Order {
        let split: Vec<&str> = kind_quantity.trim().split(" ").collect();
        return Order {
            kind: String::from(split[1]),
            quantity: split[0].parse().unwrap(),
        };
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Formula {
    output: Order,
    input: Vec<Order>,
}

impl Formula {
    pub fn from_str(formula: &str) -> Formula {
        let split: Vec<&str> = formula.split("=>").collect();

        let output = Order::from_str(split[1]);
        let input = split[0].split(",").map(|s| Order::from_str(s)).collect();

        return Formula {
            output: output,
            input: input,
        };
    }
}

pub fn start(input: &str) {
    let formulae = parse_input(&input);
    let ore_count = ore_for_fuel(&formulae);
    println!("Ore required: {}", ore_count);
}

fn parse_input(input: &str) -> HashMap<String, Formula> {
    let mut formulae = HashMap::new();

    input
        .trim()
        .split("\n")
        .map(|s| Formula::from_str(s))
        .for_each(|f| {
            if let Some(_) = formulae.insert(f.output.kind.clone(), f) {
                panic!("Multiple formulae of the same kind detected");
            }
        });

    return formulae;
}

fn ore_for_fuel(formulae: &HashMap<String, Formula>) -> u32 {
    let mut queue = Vec::new();
    queue.push(Order::from_str("1 FUEL"));
    let mut reserves = HashMap::new();

    loop {
        if queue.len() <= 0 {
            break;
        }

        let order = queue.pop().unwrap();
        let resource_count = reserves.entry(order.kind.clone()).or_insert(0);

        if order.kind == "ORE" {
            *resource_count += order.quantity;
        } else {
            let mut quantity_produced = 0;
            if *resource_count > 0 {
                quantity_produced = *resource_count;
                *resource_count = 0;
            }

            while order.quantity > quantity_produced {
                let formula = formulae.get(&order.kind).unwrap();
                formula.input.iter().for_each(|kq| queue.push(kq.clone()));
                quantity_produced += formula.output.quantity;
            }

            if quantity_produced > order.quantity {
                *resource_count = quantity_produced - order.quantity;
            }
        }
    }

    return *reserves.get("ORE").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input = "9 ORE => 2 A\n8 ORE => 3 B\n7 ORE => 5 C\n3 A, 4 B => 1 AB\n\
                     5 B, 7 C => 1 BC\n4 C, 1 A => 1 CA\n2 AB, 3 BC, 4 CA => 1 FUEL";

        let formulae = parse_input(&input);
        let ore_count = ore_for_fuel(&formulae);

        assert_eq!(165, ore_count);
    }

    #[test]
    fn test_two() {
        let input = "157 ORE => 5 NZVS\n165 ORE => 6 DCFZ\n\
                     44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n\
                     12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n179 ORE => 7 PSHF\n\
                     177 ORE => 5 HKGWZ\n7 DCFZ, 7 PSHF => 2 XJWVT\n165 ORE => 2 GPVTF\n\
                     3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

        let formulae = parse_input(&input);
        let ore_count = ore_for_fuel(&formulae);

        assert_eq!(13312, ore_count);
    }
}
