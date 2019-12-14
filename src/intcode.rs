#[derive(Clone, Debug)]
pub struct IntCodeProgram {
    program: Vec<i32>,
    pub in_buf: Vec<i32>,
    pub out_buf: Vec<i32>,
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
            in_buf: Vec::new(),
            out_buf: Vec::new(),
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
            // println!("{:?}", self.program);
            // println!("{:?}", idx);
            // println!("{:?}", (opcode, p_one, p_two, p_three));
            match opcode {
                1 => self.opcode_one(&mut idx, p_one, p_two, p_three),
                2 => self.opcode_two(&mut idx, p_one, p_two, p_three),
                3 => self.opcode_three(&mut idx, p_one),
                4 => self.opcode_four(&mut idx, p_one),
                5 => self.opcode_five(&mut idx, p_one, p_two),
                6 => self.opcode_six(&mut idx, p_one, p_two),
                7 => self.opcode_seven(&mut idx, p_one, p_two, p_three),
                8 => self.opcode_eight(&mut idx, p_one, p_two, p_three),
                99 => break,
                _ => panic!("Unknown opcode: {}", opcode),
            }
        }
    }

    fn opcode_one(&mut self, idx: &mut usize, one: usize, two: usize, three: usize) {
        self.program[three] = self.program[one] + self.program[two];
        *idx += 4;
    }

    fn opcode_two(&mut self, idx: &mut usize, one: usize, two: usize, three: usize) {
        self.program[three] = self.program[one] * self.program[two];
        *idx += 4;
    }

    fn opcode_three(&mut self, idx: &mut usize, one: usize) {
        self.program[one] = self.in_buf.remove(0);
        *idx += 2;
    }

    fn opcode_four(&mut self, idx: &mut usize, one: usize) {
        self.out_buf.push(self.program[one]);
        *idx += 2;
    }

    fn opcode_five(&mut self, idx: &mut usize, one: usize, two: usize) {
        if self.program[one] != 0 {
            *idx = self.program[two] as usize;
        } else {
            *idx += 3;
        }
    }

    fn opcode_six(&mut self, idx: &mut usize, one: usize, two: usize) {
        if self.program[one] == 0 {
            *idx = self.program[two] as usize;
        } else {
            *idx += 3;
        }
    }

    fn opcode_seven(&mut self, idx: &mut usize, one: usize, two: usize, three: usize) {
        if self.program[one] < self.program[two] {
            self.program[three] = 1;
        } else {
            self.program[three] = 0;
        }
        *idx += 4;
    }

    fn opcode_eight(&mut self, idx: &mut usize, one: usize, two: usize, three: usize) {
        if self.program[one] == self.program[two] {
            self.program[three] = 1;
        } else {
            self.program[three] = 0;
        }
        *idx += 4;
    }

    fn get_opcode_and_parameters(&self, idx: usize) -> (i32, usize, usize, usize) {
        let mut digits: [u32; 5] = [0; 5];
        let oc_str = self.program[idx].to_string();

        let mut i = 0;
        for c in oc_str.chars().rev() {
            digits[i] = c.to_digit(10).unwrap();
            i += 1;
        }

        return (
            (digits[0] + digits[1] * 10) as i32,
            self.get_index_from_mode(digits[2], idx + 1),
            self.get_index_from_mode(digits[3], idx + 2),
            self.get_index_from_mode(digits[4], idx + 3),
        );
    }

    fn get_index_from_mode(&self, mode: u32, idx: usize) -> usize {
        match mode {
            0 => {
                if let Some(v) = self.program.get(idx) {
                    return (*v) as usize;
                }
                return 0 as usize;
            }
            1 => return idx,
            _ => panic!("Unknown parameter mode: {}", mode),
        }
    }
}
