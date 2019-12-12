use std::io::stdin;

#[derive(Clone, Debug)]
pub struct IntCodeProgram {
    program: Vec<i32>,
}

impl IntCodeProgram {
    pub fn from_input(input: &str) -> IntCodeProgram {
        IntCodeProgram {
            program: input
                .trim()
                .split(',')
                .map(|num| {
                    return num.parse::<i32>().unwrap();
                })
                .collect(),
        }
    }

    pub fn set_at(&mut self, idx: usize, value: i32) {
        self.program[idx] = value;
    }

    pub fn get(&self, idx: usize) -> i32 {
        return self.program[idx];
    }

    pub fn run(&mut self) {
        let mut idx = 0;

        loop {
            let (opcode, p_one, p_two, p_three) = self.get_opcode_and_parameters(idx);
            println!("{:?}", self.program);
            println!("{:?}", (opcode, p_one, p_two, p_three));
            match opcode {
                1 => self.opcode_one(&mut idx, p_one, p_two, p_three),
                2 => self.opcode_two(&mut idx, p_one, p_two, p_three),
                3 => self.opcode_three(&mut idx, p_one),
                4 => self.opcode_four(&mut idx, p_one),
                99 => break,
                _ => panic!("Unknown opcode: {}", self.program[idx]),
            }
        }
        println!("after break");
    }

    fn opcode_one(&mut self, idx: &mut usize, one: i32, two: i32, three: i32) {
        self.program[three as usize] = one + two;
        *idx += 4;
    }

    fn opcode_two(&mut self, idx: &mut usize, one: i32, two: i32, three: i32) {
        self.program[three as usize] = one * two;
        *idx += 4;
    }

    fn opcode_three(&mut self, idx: &mut usize, one: i32) {
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();

        self.program[one as usize] = s.trim().parse::<i32>().unwrap();
        *idx += 2;
    }

    fn opcode_four(&mut self, idx: &mut usize, one: i32) {
        println!("{}", self.program[one as usize]);
        *idx += 2;
    }

    fn get_opcode_and_parameters(&self, idx: usize) -> (i32, i32, i32, i32) {
        let mut digits: [u32; 5] = [0; 5];

        let oc_str = self.program[idx].to_string();
        for (i, c) in oc_str.char_indices().rev() {
            digits[i] = c.to_digit(10).unwrap();
        }

        return (
            (digits[0] + digits[1] * 10) as i32,
            self.get_parameter_from_mode(digits[2], idx + 1),
            self.get_parameter_from_mode(digits[3], idx + 2),
            self.get_parameter_from_mode(digits[4], idx + 3),
        );
    }

    fn get_parameter_from_mode(&self, mode: u32, idx: usize) -> i32 {
        match mode {
            0 => self.program[self.program[idx] as usize],
            1 => self.program[idx],
            _ => panic!("Unknown parameter mode: {}", mode),
        }
    }
}
