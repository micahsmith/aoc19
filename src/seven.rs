use crate::intcode::IntCodeProgram;

pub fn start(input: &str) {
    without_feedback(input);
    with_feedback(input);

}

fn without_feedback(input: &str) {
    let mut max_output_signal = 0;
    let mut permutations: Vec<Vec<u32>> = Vec::new();
    heaps_algo(5, &mut [0, 1, 2, 3, 4], &mut permutations);

    for current_phase_setting in permutations.iter() {
        let current_output_signal = run_amplifiers(input, &current_phase_setting);
        if current_output_signal > max_output_signal {
            max_output_signal = current_output_signal;
        }
    }

    println!("Max output signal: {}", max_output_signal);
}

fn with_feedback(input: &str) {
    let mut max_output_signal = 0;
    let mut permutations: Vec<Vec<u32>> = Vec::new();
    heaps_algo(5, &mut [5, 6, 7, 8, 9], &mut permutations);

    for current_phase_setting in permutations.iter() {
        let current_output_signal = run_amplifiers(input, &current_phase_setting);
        if current_output_signal > max_output_signal {
            max_output_signal = current_output_signal;
        }
    }

    println!("Max output signal: {}", max_output_signal);
}

fn heaps_algo(k: u8, a: &mut [u32], v: &mut Vec<Vec<u32>>) {
    if k == 1 {
        let mut permutation = Vec::new();
        permutation.extend_from_slice(a);
        v.push(permutation);
    } else {
        heaps_algo(k - 1, a, v);
        for i in 0..k - 1 {
            if k % 2 == 0 {
                a.swap(i as usize, (k - 1) as usize);
            } else {
                a.swap(0, (k - 1) as usize);
            }
            heaps_algo(k - 1, a, v)
        }
    }
}

fn run_amplifiers(input: &str, phase_setting: &Vec<u32>) -> i32 {
    let mut output_signal = 0;
    for amp_phase in phase_setting.iter() {
        let mut amp = IntCodeProgram::from_input(&input);
        amp.in_buf.push(*amp_phase as i32);
        amp.in_buf.push(output_signal);
        amp.run();
        output_signal = amp.out_buf.remove(0);
    }
    return output_signal;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_one() {
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let phase_setting: Vec<u32> = vec![4, 3, 2, 1, 0];

        let output = run_amplifiers(&input, &phase_setting);
        assert_eq!(output, 43210);
    }

    #[test]
    fn test_input_two() {
        let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let phase_setting: Vec<u32> = vec![0, 1, 2, 3, 4];

        let output = run_amplifiers(&input, &phase_setting);
        assert_eq!(output, 54321);
    }

    #[test]
    fn test_input_three() {
        let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,\
                     1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let phase_setting: Vec<u32> = vec![1, 0, 4, 3, 2];

        let output = run_amplifiers(&input, &phase_setting);
        assert_eq!(output, 65210);
    }
}
