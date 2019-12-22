use crate::intcode::{IntCodeProgram, IntCodeStatus};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CardDirection {
    Down,
    Left,
    Up,
    Right,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Rotation {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Robot {
    dir: CardDirection,
    loc: Point,
}

impl Robot {
    pub fn new() -> Robot {
        return Robot {
            dir: CardDirection::Up,
            loc: Point { x: 0, y: 0 },
        };
    }

    pub fn change_direction(&mut self, rotation: &Rotation) {
        match self.dir {
            CardDirection::Down => match rotation {
                Rotation::Left => self.dir = CardDirection::Right,
                Rotation::Right => self.dir = CardDirection::Left,
            },
            CardDirection::Left => match rotation {
                Rotation::Left => self.dir = CardDirection::Down,
                Rotation::Right => self.dir = CardDirection::Up,
            },
            CardDirection::Up => match rotation {
                Rotation::Left => self.dir = CardDirection::Left,
                Rotation::Right => self.dir = CardDirection::Right,
            },
            CardDirection::Right => match rotation {
                Rotation::Left => self.dir = CardDirection::Up,
                Rotation::Right => self.dir = CardDirection::Down,
            },
        }
    }

    pub fn forward(&mut self) -> Point {
        match self.dir {
            CardDirection::Down => {
                self.loc = Point {
                    x: self.loc.x,
                    y: self.loc.y - 1,
                };
            }
            CardDirection::Left => {
                self.loc = Point {
                    x: self.loc.x - 1,
                    y: self.loc.y,
                };
            }
            CardDirection::Up => {
                self.loc = Point {
                    x: self.loc.x,
                    y: self.loc.y + 1,
                };
            }
            CardDirection::Right => {
                self.loc = Point {
                    x: self.loc.x + 1,
                    y: self.loc.y,
                };
            }
        }
        return self.loc;
    }
}

pub fn start(input: &str) {
    let mut program = IntCodeProgram::from_input(input);
    let mut robot = Robot::new();
    let mut hull: HashMap<Point, u8> = HashMap::new();
    run_robot(&mut program, &mut robot, &mut hull);

    let mut program = IntCodeProgram::from_input(input);
    let mut robot = Robot::new();
    let mut hull: HashMap<Point, u8> = HashMap::new();
    hull.insert(Point { x: 0, y: 0 }, 1);
    run_robot(&mut program, &mut robot, &mut hull);
    render(&hull);
}

fn run_robot(program: &mut IntCodeProgram, robot: &mut Robot, hull: &mut HashMap<Point, u8>) {
    let mut changed_panels: HashSet<Point> = HashSet::new();

    while program.status != IntCodeStatus::Halted {
        let hull_point = hull.entry(robot.loc).or_insert(0);
        program.in_buf.push(*hull_point as i64);
        program.run();

        let color = program.out_buf.remove(0) as u8;
        if color != *hull_point {
            *hull_point = color;
            changed_panels.insert(robot.loc);
        }

        let dir = program.out_buf.remove(0);
        match dir {
            0 => robot.change_direction(&Rotation::Left),
            1 => robot.change_direction(&Rotation::Right),
            _ => panic!("Unanticipated direction: {}", dir),
        }

        robot.forward();
    }

    println!("Change set length: {}", changed_panels.len());
}

fn render(hull: &HashMap<Point, u8>) {
    let mut x_max = 0;
    let mut x_min = 0;
    let mut y_max = 0;
    let mut y_min = 0;

    for p in hull.keys() {
        if p.x > x_max {
            x_max = p.x;
        } else if p.x < x_min {
            x_min = p.x;
        }

        if p.y > y_max {
            y_max = p.y;
        } else if p.y < y_min {
            y_min = p.y;
        }
    }

    let mut canvas = Vec::new();
}
