extern crate rand;

use crate::intcode::IntCodeProgram;
use rand::Rng;
use std::collections::HashMap;

#[derive(Clone, Debug)]
enum DroidMovement {
    North,
    South,
    West,
    East,
}

fn direction_from_movement(movement: &DroidMovement) -> [i64; 2] {
    match movement {
        DroidMovement::North => [0, 1],
        DroidMovement::South => [0, -1],
        DroidMovement::West => [-1, 0],
        DroidMovement::East => [1, 0],
    }
}

fn instr_from_movement(movement: &DroidMovement) -> i64 {
    match movement {
        DroidMovement::North => 1,
        DroidMovement::South => 2,
        DroidMovement::West => 3,
        DroidMovement::East => 4,
    }
}

fn change_movement() -> DroidMovement {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0, 4) {
        0 => DroidMovement::North,
        1 => DroidMovement::South,
        2 => DroidMovement::West,
        3 => DroidMovement::East,
        _ => panic!("Rng out of range"),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum DroidStatus {
    Wall,
    Step,
    OxSys,
}

fn status_from_status_code(status_code: i64) -> DroidStatus {
    match status_code {
        0 => DroidStatus::Wall,
        1 => DroidStatus::Step,
        2 => DroidStatus::OxSys,
        _ => panic!("Unknown status code: {}", status_code),
    }
}

type Point = [i64; 2];
type Map = HashMap<Point, DroidStatus>;

pub fn start(input: &str) {
    let mut program = IntCodeProgram::from_input(&input);
    let map = droid_loop(&mut program);
    let mh_dist = find_mh_distance(&map);
    println!("Fewest movements: {}", mh_dist);
}

fn droid_loop(program: &mut IntCodeProgram) -> Map {
    let mut map = HashMap::new();
    let mut movement = DroidMovement::North;
    let mut pos = [0, 0];
    let mut status: DroidStatus;
    map.insert(pos, DroidStatus::Step);

    loop {
        program.in_buf.push(instr_from_movement(&movement));
        program.run();
        status = status_from_status_code(program.out_buf.remove(0));
        let direction = direction_from_movement(&movement);

        match status {
            DroidStatus::Wall => {
                let wall_pos = [pos[0] + direction[0], pos[1] + direction[1]];
                map.insert(wall_pos, DroidStatus::Wall);
                movement = change_movement();
            }
            DroidStatus::Step => {
                pos = [pos[0] + direction[0], pos[1] + direction[1]];
                map.insert(pos, DroidStatus::Step);
                movement = change_movement();
            }
            DroidStatus::OxSys => {
                pos = [pos[0] + direction[0], pos[1] + direction[1]];
                map.insert(pos, DroidStatus::OxSys);
                break;
            }
        }
    }

    return map;
}

fn find_mh_distance(map: &Map) -> u32 {
    let mut ox_sys = [0, 0];

    for (pos, status) in map.iter() {
        if *status == DroidStatus::OxSys {
            ox_sys = *pos;
            break;
        }
    }

    return (ox_sys[0].abs() + ox_sys[1].abs()) as u32;
}
