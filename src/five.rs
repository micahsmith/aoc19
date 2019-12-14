use crate::intcode::IntCodeProgram;

pub fn start(input: &str) {
    let mut program = IntCodeProgram::from_input(&input);

    program.in_buf.push(5);
    program.run();
    println!("{}", program.out_buf.first().unwrap());
}
