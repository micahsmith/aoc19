use crate::intcode::{IntCodeProgram, IntCodeStatus};

pub fn start(input: &str) {
    let mut permutations: Vec<Vec<u32>> = Vec::new();
    heaps_algo(5, &mut [0, 1, 2, 3, 4], &mut permutations);
    run_without_feedback(input, permutations);

    let mut permutations: Vec<Vec<u32>> = Vec::new();
    heaps_algo(5, &mut [5, 6, 7, 8, 9], &mut permutations);
    run_with_feedback(input, permutations);
}

fn run_without_feedback(input: &str, permutations: Vec<Vec<u32>>) {
    let mut max_output_signal = 0;
    for phase_setting in permutations.iter() {
        let mut output_signal = 0;
        for amp_phase in phase_setting.iter() {
            let mut amp = IntCodeProgram::from_input(&input);
            amp.in_buf.push(*amp_phase as i64);
            amp.in_buf.push(output_signal);
            amp.run();
            output_signal = amp.out_buf.remove(0);
        }
        if output_signal > max_output_signal {
            max_output_signal = output_signal;
        }
    }

    println!("Max output signal: {}", max_output_signal);
}

fn run_with_feedback(input: &str, permutations: Vec<Vec<u32>>) {
    let mut max_output_signal = 0;
    for phase_setting in permutations.iter() {
        let output_signal = feedback(input, &phase_setting);
        if output_signal > max_output_signal {
            max_output_signal = output_signal;
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

fn feedback(input: &str, phase_setting: &Vec<u32>) -> i64 {
    let mut amplifiers: Vec<IntCodeProgram> = Vec::new();
    for amp_phase in phase_setting.iter() {
        let mut amp = IntCodeProgram::from_input(&input);
        amp.in_buf.push(*amp_phase as i64);
        amplifiers.push(amp);
    }

    let mut output_signal: i64 = 0;
    loop {
        for amp in amplifiers.iter_mut() {
            amp.in_buf.push(output_signal);
            amp.status = IntCodeStatus::Ready;
            amp.run();
            output_signal = amp.out_buf.remove(0);
        }

        if amplifiers.last().unwrap().status == IntCodeStatus::Halted {
            break;
        }
    }

    return output_signal;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feedback_one() {
        let input = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,\
                     26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let phase_setting = vec![9, 8, 7, 6, 5];

        assert_eq!(feedback(input, &phase_setting), 139629729);
    }
}
