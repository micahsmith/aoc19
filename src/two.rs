use crate::intcode::IntCodeProgram;

pub fn start(input: &str) {
    let program = IntCodeProgram::from_input(&input);
    println!("{:?}", program);

    for noun in 0..100 {
        for verb in 0..100 {
            let mut cp = program.clone();
            cp.set_at(1, noun);
            cp.set_at(2, verb);
            cp.run();

            if cp.get(0) == 19690720 {
                println!("{}", 100 * noun + verb);
                break;
            }
        }
    }
}
