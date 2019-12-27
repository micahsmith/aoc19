use crate::intcode::IntCodeProgram;
use std::collections::HashMap;

#[derive(Clone, Debug)]
enum DroidMovement {
    North,
    South,
    West,
    East,
}

impl DroidMovement {
    fn get_direction(&self) -> [i64; 2] {
        match self {
            DroidMovement::North => [0, 1],
            DroidMovement::South => [0, -1],
            DroidMovement::West => [-1, 0],
            DroidMovement::East => [1, 0],
        }
    }

    fn from_direction(dir: [i64; 2]) -> Self {
        match dir {
            [0, 1] => DroidMovement::North,
            [0, -1] => DroidMovement::South,
            [-1, 0] => DroidMovement::West,
            [1, 0] => DroidMovement::East,
            _ => panic!("Unknown direction: {:?}", dir),
        }
    }

    fn get_instr(&self) -> i64 {
        match self {
            DroidMovement::North => 1,
            DroidMovement::South => 2,
            DroidMovement::West => 3,
            DroidMovement::East => 4,
        }
    }

    fn rev(&self) -> Self {
        match self {
            DroidMovement::North => DroidMovement::South,
            DroidMovement::South => DroidMovement::North,
            DroidMovement::West => DroidMovement::East,
            DroidMovement::East => DroidMovement::West,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum DroidStatus {
    Wall,
    Blank,
    OxSys,
}

impl DroidStatus {
    fn from_status_code(status_code: i64) -> Self {
        match status_code {
            0 => DroidStatus::Wall,
            1 => DroidStatus::Blank,
            2 => DroidStatus::OxSys,
            _ => panic!("Unknown status code: {}", status_code),
        }
    }
}

fn add_points(one: &Point, two: &Point) -> Point {
    return [one[0] + two[0], one[1] + two[1]];
}

type Point = [i64; 2];
type Map = HashMap<Point, DroidStatus>;

pub fn start(input: &str) {
    let mut program = IntCodeProgram::from_input(&input);
    let map = droid_loop(&mut program);
    println!("Fewest movements: {:?}", map);
}

fn droid_loop(program: &mut IntCodeProgram) -> Map {
    let mut breadcrumbs = Vec::new();
    let mut map = HashMap::new();
    let mut movement = DroidMovement::North;
    let mut pos = [0, 0];
    let mut status: DroidStatus;

    map.insert(pos, DroidStatus::Blank);

    loop {
        program.in_buf.push(movement.get_instr());
        program.run();
        status = DroidStatus::from_status_code(program.out_buf.remove(0));
        let direction = movement.get_direction();

        match status {
            DroidStatus::Wall => {
                map.insert(add_points(&pos, &direction), DroidStatus::Wall);
            }
            DroidStatus::Blank => {
                map.insert(add_points(&pos, &direction), DroidStatus::Blank);
                pos = add_points(&pos, &direction);
            }
            DroidStatus::OxSys => {
                map.insert(add_points(&pos, &direction), DroidStatus::OxSys);
                pos = add_points(&pos, &direction);
            }
        }

        if let Some(dir) = find_unexplored(&map, &pos) {
            movement = dir;
            breadcrumbs.push(movement.rev());
        } else if breadcrumbs.len() > 0 {
            movement = breadcrumbs.pop().unwrap();
        } else {
            break;
        }
    }

    return map;
}

fn find_unexplored(map: &Map, pos: &Point) -> Option<DroidMovement> {
    let directions = [
        DroidMovement::North,
        DroidMovement::South,
        DroidMovement::West,
        DroidMovement::East,
    ];
    let mut unexplored = Vec::new();

    for dir in directions.iter() {
        let point = add_points(pos, &dir.get_direction());
        if let None = map.get(&point) {
            unexplored.push(dir.get_direction());
        }
    }

    if let Some(p) = unexplored.get(0) {
        return Some(DroidMovement::from_direction(*p));
    }
    return None;
}
