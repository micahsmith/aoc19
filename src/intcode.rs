#[derive(Clone, Debug)]
pub struct IntCodeProgram {
    program: Vec<i32>,
    pointer: usize,
    pub in_buf: Vec<i32>,
    pub out_buf: Vec<i32>,
    pub status: IntCodeStatus,
}

#[derive(Clone, Debug, PartialEq)]
pub enum IntCodeStatus {
    Paused,
    Halted,
    Ready,
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
            pointer: 0,
            status: IntCodeStatus::Ready,
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
        while self.status == IntCodeStatus::Ready {
            let (opcode, p_one, p_two, p_three) = self.get_opcode_and_parameters();
            match opcode {
                1 => self.opcode_one(p_one, p_two, p_three),
                2 => self.opcode_two(p_one, p_two, p_three),
                3 => self.opcode_three(p_one),
                4 => self.opcode_four(p_one),
                5 => self.opcode_five(p_one, p_two),
                6 => self.opcode_six(p_one, p_two),
                7 => self.opcode_seven(p_one, p_two, p_three),
                8 => self.opcode_eight(p_one, p_two, p_three),
                99 => self.opcode_ninety_nine(),
                _ => panic!("Unknown opcode: {}", opcode),
            }
        }
    }

    fn opcode_one(&mut self, one: usize, two: usize, three: usize) {
        self.program[three] = self.program[one] + self.program[two];
        self.pointer += 4;
    }

    fn opcode_two(&mut self, one: usize, two: usize, three: usize) {
        self.program[three] = self.program[one] * self.program[two];
        self.pointer += 4;
    }

    fn opcode_three(&mut self, one: usize) {
        if self.in_buf.get(0) == None {
            self.status = IntCodeStatus::Paused;
        } else {
            self.program[one] = self.in_buf.remove(0);
            self.pointer += 2;
        }
    }

    fn opcode_four(&mut self, one: usize) {
        self.out_buf.push(self.program[one]);
        self.pointer += 2;
    }

    fn opcode_five(&mut self, one: usize, two: usize) {
        if self.program[one] != 0 {
            self.pointer = self.program[two] as usize;
        } else {
            self.pointer += 3;
        }
    }

    fn opcode_six(&mut self, one: usize, two: usize) {
        if self.program[one] == 0 {
            self.pointer = self.program[two] as usize;
        } else {
            self.pointer += 3;
        }
    }

    fn opcode_seven(&mut self, one: usize, two: usize, three: usize) {
        if self.program[one] < self.program[two] {
            self.program[three] = 1;
        } else {
            self.program[three] = 0;
        }
        self.pointer += 4;
    }

    fn opcode_eight(&mut self, one: usize, two: usize, three: usize) {
        if self.program[one] == self.program[two] {
            self.program[three] = 1;
        } else {
            self.program[three] = 0;
        }
        self.pointer += 4;
    }
    
    fn opcode_ninety_nine(&mut self) {
        self.status = IntCodeStatus::Halted;
    }

    fn get_opcode_and_parameters(&self) -> (i32, usize, usize, usize) {
        let mut digits: [u32; 5] = [0; 5];
        let oc_str = self.program[self.pointer].to_string();

        let mut i = 0;
        for c in oc_str.chars().rev() {
            digits[i] = c.to_digit(10).unwrap();
            i += 1;
        }

        return (
            (digits[0] + digits[1] * 10) as i32,
            self.get_index_from_mode(digits[2], self.pointer + 1),
            self.get_index_from_mode(digits[3], self.pointer + 2),
            self.get_index_from_mode(digits[4], self.pointer + 3),
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
