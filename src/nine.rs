use crate::intcode::IntCodeProgram;

pub fn start(input: &str) {
    let mut program = IntCodeProgram::from_input(input);
    program.in_buf.push(1);
    program.run();
    println!("{}", program.out_buf.remove(0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut program = IntCodeProgram::from_input(&input);
        program.run();

        assert_eq!(
            program.out_buf,
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
    }

    #[test]
    fn test_two() {
        let input = "104,1125899906842624,99";
        let mut program = IntCodeProgram::from_input(&input);
        program.run();

        assert_eq!(program.out_buf.remove(0), 1125899906842624);
    }
}
