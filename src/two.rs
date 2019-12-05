pub fn start(input: &str) {
    let program: Vec<u32> = input
        .trim()
        .split(',')
        .map(|num| {
            return num.parse::<u32>().unwrap();
        })
        .collect();

    println!("{:?}", program);

    for noun in 0..100 {
        for verb in 0..100 {
            let mut cp = program.clone();
            cp[1] = noun;
            cp[2] = verb;
            run_program(&mut cp);

            if cp[0] == 19690720 {
                println!("{}", 100 * noun + verb);
                break;
            }
        }
    }

}

fn run_program(program: &mut [u32]) -> u32 {
    let mut index = 0;

    loop {
        let opcode = program[index];
        match opcode {
            1 => {
                let first_idx = program[index + 1] as usize;
                let second_idx = program[index + 2] as usize;
                let change_idx = program[index + 3] as usize;
                program[change_idx] = program[first_idx] + program[second_idx];
            },
            2 => {
                let first_idx = program[index + 1] as usize;
                let second_idx = program[index + 2] as usize;
                let change_idx = program[index + 3] as usize;
                program[change_idx] = program[first_idx] * program[second_idx];
            }
            99 => break 0,
            _ => {
                println!("Unknown opcode: {}", opcode);
                break 0;
            }
        }
        index += 4;
    }
}
