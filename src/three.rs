use std::cmp;
use std::collections::{HashMap, HashSet};
use std::i32;

pub fn start(input: &str) {
    let (wire_one, wire_two) = format_input(input);

    let wire_one_pm = get_point_map(&wire_one);
    let wire_two_pm = get_point_map(&wire_two);

    let intersection = get_map_key_intersection(&wire_one_pm, &wire_two_pm);
    get_min_mh_distance(&intersection);
    get_min_step_distance(&wire_one_pm, &wire_two_pm, &intersection);
}

fn format_input(input: &str) -> (Vec<Instruction>, Vec<Instruction>) {
    let mut formatted: Vec<Vec<Instruction>> = input
        .trim()
        .split("\n")
        .map(|wire| format_wire(wire))
        .collect();

    (formatted.remove(0), formatted.remove(0))
}

struct Instruction {
    direction: Direction,
    distance: u32,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn format_wire(wire: &str) -> Vec<Instruction> {
    wire.trim()
        .split(",")
        .map(|instr| Instruction {
            direction: get_direction(instr.get(0..1).unwrap()),
            distance: instr.get(1..).unwrap().parse::<u32>().unwrap(),
        })
        .collect()
}

fn get_direction(dir_char: &str) -> Direction {
    match dir_char {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!("Invalid direction."),
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn get_point_map(wire: &Vec<Instruction>) -> HashMap<Point, u32> {
    let mut point_map: HashMap<Point, u32> = HashMap::new();

    let mut steps = 0;
    let mut x = 0;
    let mut y = 0;

    for instruction in wire.iter() {
        for _ in 0..instruction.distance {
            match instruction.direction {
                Direction::Up => y = y + 1,
                Direction::Down => y = y - 1,
                Direction::Left => x = x - 1,
                Direction::Right => x = x + 1,
            }

            // NB: We only track the smallest number of steps to a particular point, and
            // do not store any information in the case that a line crosses itself.
            steps += 1;
            point_map.entry(Point { x, y }).or_insert(steps);
        }
    }

    return point_map;
}

fn get_map_key_intersection(
    one: &HashMap<Point, u32>,
    two: &HashMap<Point, u32>,
) -> HashSet<Point> {
    let mut hs: HashSet<Point> = HashSet::new();

    for key in one.keys() {
        if two.contains_key(key) {
            hs.insert(key.clone());
        }
    }

    return hs;
}

fn get_min_mh_distance(point_set: &HashSet<Point>) {
    let min = point_set
        .iter()
        .map(|point| point.x.abs() + point.y.abs())
        .fold(std::i32::MAX, |acc, v| cmp::min(acc, v));

    println!("Minimum manhattan distance: {}", min);
}

fn get_min_step_distance(
    wire_one_hm: &HashMap<Point, u32>,
    wire_two_hm: &HashMap<Point, u32>,
    point_set: &HashSet<Point>,
) {
    let mut min = std::u32::MAX;

    for point in point_set.iter() {
        let sum = wire_one_hm.get(point).unwrap() + wire_two_hm.get(point).unwrap();
        if sum < min {
            min = sum;
        }
    }

    println!("Minimum step count: {}", min);
}
