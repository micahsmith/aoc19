#[derive(Clone, Debug)]
pub struct IntCodeProgram {
    program: Vec<u32>,
}

impl IntCodeProgram {
    pub fn from_input(input: &str) -> IntCodeProgram {
        IntCodeProgram {
            program: input
                .trim()
                .split(',')
                .map(|num| {
                    return num.parse::<u32>().unwrap();
                })
                .collect(),
        }
    }

    pub fn set_at(&mut self, idx: usize, value: u32) {
        self.program[idx] = value;
    }

    pub fn get(&self, idx: usize) -> u32 {
        return self.program[idx];
    }

    pub fn run(&mut self) {
        let mut idx = 0;

        loop {
            match self.program[idx] {
                1 => self.opcode_one(idx),
                2 => self.opcode_two(idx),
                99 => break,
                _ => panic!("Unknown opcode: {}", self.program[idx]),
            }
            idx += 4;
        }

        return;
    }

    fn opcode_one(&mut self, idx: usize) {
        let input_one = self.program[idx + 1] as usize;
        let input_two = self.program[idx + 2] as usize;
        let output_pos = self.program[idx + 3] as usize;
        self.program[output_pos] = self.program[input_one] + self.program[input_two];
    }

    fn opcode_two(&mut self, idx: usize) {
        let input_one = self.program[idx + 1] as usize;
        let input_two = self.program[idx + 2] as usize;
        let output_pos = self.program[idx + 3] as usize;
        self.program[output_pos] = self.program[input_one] * self.program[input_two];
    }
}
