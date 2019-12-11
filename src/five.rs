use crate::intcode::IntCodeProgram;

pub fn start(input: &str) {
    let mut program = IntCodeProgram::from_input(&input);
    println!("{:?}", program);

    program.run();
}
