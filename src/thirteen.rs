use crate::intcode::{IntCodeProgram, IntCodeStatus};
use std::cmp::Ordering;
use std::collections::HashMap;

type Screen = HashMap<[i64; 2], i64>;

pub fn start(input: &str) {
    let mut program = IntCodeProgram::from_input(input);
    let screen = run_game(&mut program);
    println!("{}", get_block_count(&screen));

    let mut program = IntCodeProgram::from_input(input);
    program.set_at(0, 2);
    run_game(&mut program);
}

fn run_game(program: &mut IntCodeProgram) -> Screen {
    let mut screen = HashMap::new();

    while program.status != IntCodeStatus::Halted {
        program.run();
        screen = parse_output(&program.out_buf);
        program.in_buf.push(joystick(&screen));
        program.out_buf = Vec::new();
    }

    if let Some(score) = screen.get(&[-1, 0]) {
        println!("Score: {}", score);
    }
    return screen;
}

fn joystick(screen: &Screen) -> i64 {
    let mut ball = [0, 0];
    let mut paddle = [0, 0];

    for (coords, id) in screen.iter() {
        match id {
            3 => paddle = *coords,
            4 => ball = *coords,
            _ => (),
        }
    }

    match ball[0].cmp(&paddle[0]) {
        Ordering::Greater => return 1,
        Ordering::Less => return -1,
        Ordering::Equal => return 0,
    }
}

fn get_block_count(screen: &Screen) -> usize {
    return screen
        .values()
        .filter(|v| {
            if **v == 2 {
                return true;
            } else {
                return false;
            }
        })
        .count();
}

fn parse_output(output: &Vec<i64>) -> Screen {
    let mut screen = HashMap::new();

    for chunk in output.chunks(3) {
        if let &[x, y, value] = chunk {
            screen.insert([x, y], value);
        }
    }

    return screen;
}
