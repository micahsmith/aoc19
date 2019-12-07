use std::cmp;
use std::collections::{HashMap, HashSet};
use std::i32;

pub fn start(input: &str) {
    let (wire_one, wire_two) = format_input(input);

    let wire_one_pm = get_point_map(&wire_one);
    let wire_two_pm = get_point_map(&wire_two);

    let wire_one_ps = get_point_map(&wire_one);
    let wire_two_ps = get_point_map(&wire_two);

    let is: HashSet<&Point> = wire_one_ps.intersection(&wire_two_ps).collect();
    get_min_mh_distance(&is);
    get_min_step_distance(&wire_one, &wire_two, &is);
}

fn format_input(input: &str) -> (Vec<Instruction>, Vec<Instruction>) {
    let mut formatted: Vec<Vec<Instruction>> = input
        .trim()
        .split("\n")
        .map(|wire| format_wire(wire))
        .collect();

    (formatted.remove(0), formatted.remove(0))
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Instruction {
    direction: Direction,
    distance: u32,
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

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn get_point_map(wire: &Vec<Instruction>) -> HashMap<Point, u32> {
    let mut point_map: HashMap<Point, u32> = HashMap::new();

    let mut x = 0;
    let mut y = 0;

    for instruction in wire.iter() {
        for step_num in 0..instruction.distance {
            match instruction.direction {
                Direction::Up => {
                    y = y + 1;
                    point_map.entry(Point { x, y }).or_insert(step_num);
                }
                Direction::Down => {
                    y = y - 1;
                    point_map.entry(Point { x, y }).or_insert(step_num);
                }
                Direction::Left => {
                    x = x - 1;
                    point_map.entry(Point { x, y }).or_insert(step_num);
                }
                Direction::Right => {
                    x = x + 1;
                    point_map.entry(Point { x, y }).or_insert(step_num);
                }
            }
        }
    }

    point_map
}

fn get_min_mh_distance(ps: &HashSet<&Point>) {
    let min = ps
        .iter()
        .map(|point| point.x.abs() + point.y.abs())
        .fold(std::i32::MAX, |acc, v| cmp::min(acc, v));

    println!("Minimum manhattan distance: {}", min);
}

fn get_min_step_distance(
    wire_one: &Vec<Instruction>,
    wire_two: &Vec<Instruction>,
    ps: &HashSet<&Point>,
) {
    for point in ps.iter() {
        println!("{:?}", point);
    }
}
