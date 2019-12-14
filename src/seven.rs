use crate::intcode::IntCodeProgram;

pub fn start(input: &str) {
    IntCodeProgram::from_input(&input).run();
}
