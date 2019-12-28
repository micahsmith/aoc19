use crate::intcode::IntCodeProgram;
use std::collections::HashMap;
use std::{thread, time, usize};

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
    let (ox_sys, _) = map.iter().find(|(_, v)| **v == DroidStatus::OxSys).unwrap();
    let dist = a_star(&map, &[0, 0], &ox_sys);
    println!("Fewest movements: {}", dist);

    let iters = flood_fill(&map, &ox_sys);
    println!("Oxygen fill in min: {}", iters);
}

fn droid_loop(program: &mut IntCodeProgram) -> Map {
    let mut breadcrumbs = Vec::new();
    let mut map = HashMap::new();
    let mut movement = DroidMovement::North;
    let mut pos = [0, 0];
    let mut status: DroidStatus;

    map.insert(pos, DroidStatus::Blank);

    loop {
        //print_map(&map);
        program.in_buf.push(movement.get_instr());
        program.run();
        status = DroidStatus::from_status_code(program.out_buf.remove(0));
        let direction = movement.get_direction();

        match status {
            DroidStatus::Wall => {
                map.insert(add_points(&pos, &direction), DroidStatus::Wall);
                breadcrumbs.pop();
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
            movement = dir.clone();
            breadcrumbs.push(dir.rev());
        } else if breadcrumbs.len() > 0 {
            let bc = breadcrumbs.pop().unwrap();
            movement = bc;
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PointNode {
    point: Point,
    priority: usize,
}

impl PointNode {
    fn new(point: Point, priority: usize) -> PointNode {
        return PointNode {
            point: point,
            priority: priority,
        };
    }
}

fn a_star(map: &Map, src: &Point, tar: &Point) -> usize {
    let mut children: HashMap<Point, Point> = HashMap::new();
    let mut g_scores: HashMap<Point, usize> = HashMap::new();
    let mut f_scores: HashMap<Point, usize> = HashMap::new();
    let mut pq = Vec::new();

    g_scores.insert(src.clone(), 0);
    f_scores.insert(src.clone(), heuristic(src, tar));
    pq.push(PointNode::new(src.clone(), 0));

    while let Some(mut current_node) = pq.pop() {
        if current_node.point == *tar {
            let mut path = vec![current_node.point];
            while let Some(parent) = children.get(&current_node.point) {
                path.insert(0, *parent);
                current_node = PointNode::new(*parent, 0);
            }
            return path.len() - 1;
        }

        let neighbors = get_neighbors(&map, &current_node.point);
        for neighbor in neighbors.iter() {
            let tent_g_score = g_scores.get(&current_node.point).unwrap() + 1;
            let neighbor_g_score = g_scores.entry(*neighbor).or_insert(usize::MAX);

            if tent_g_score < *neighbor_g_score {
                children.insert(neighbor.clone(), current_node.point.clone());
                g_scores.insert(neighbor.clone(), tent_g_score);

                let f_score = tent_g_score + heuristic(neighbor, tar);
                f_scores.insert(neighbor.clone(), f_score);

                let new_node = PointNode::new(*neighbor, f_score);
                if !pq.contains(&new_node) {
                    pq.push(new_node);
                }
            }
        }

        pq.sort_by(|a, b| a.priority.cmp(&b.priority));
    }

    return 0;
}

fn heuristic(src: &Point, tar: &Point) -> usize {
    return ((src[0] - tar[0]) + (src[1] - tar[1])).abs() as usize;
}

fn get_neighbors(map: &Map, point: &Point) -> Vec<Point> {
    let directions = [
        DroidMovement::North,
        DroidMovement::South,
        DroidMovement::West,
        DroidMovement::East,
    ];

    let mut neighbors = Vec::new();

    for dir in directions.iter() {
        let point = add_points(point, &dir.get_direction());
        if let Some(status) = map.get(&point) {
            if *status != DroidStatus::Wall {
                neighbors.push(point.clone());
            }
        }
    }

    return neighbors;
}

fn flood_fill(map: &Map, ox_sys: &Point) -> usize {
    let mut map = map.clone();
    let mut minutes = 0;
    let mut next = vec![ox_sys.clone()];

    loop {
        if next.len() == 0 {
            break;
        }

        let mut current = Vec::new();
        current.append(&mut next);
        minutes += 1;

        while let Some(point) = current.pop() {
            map.insert(point, DroidStatus::Wall);
            let mut neighbors = get_neighbors(&map, &point);
            next.append(&mut neighbors);
        }
    }

    return minutes - 1;
}

#[allow(dead_code)]
fn print_map(map: &Map) {
    let mut screen = String::new();
    for row_num in -28..28 {
        let mut row = String::new();
        for col_num in -30..30 {
            if let Some(o) = map.get(&[col_num, row_num]) {
                match o {
                    DroidStatus::Wall => row.push_str("█"),
                    DroidStatus::Blank => row.push_str("░"),
                    DroidStatus::OxSys => row.push_str("O"),
                }
            } else {
                row.push_str("▒");
            }
        }
        screen.push_str(&row);
        screen.push_str("\n");
    }

    println!("{}", screen);
    thread::sleep(time::Duration::from_millis(20));
}
