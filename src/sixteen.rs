pub fn start(input: &str) {
    let parsed = parse(input);
    let result = run_fft(&parsed, 100);
    println!("First eight digits: {}", &result[..8]);
}

fn parse(input: &str) -> Vec<i32> {
    return input
        .trim()
        .chars()
        .map(|a| a.to_digit(10).unwrap() as i32)
        .collect();
}

fn run_fft(input: &Vec<i32>, iters: usize) -> String {
    let mut current: Vec<i32> = input.clone();
    let mut next: Vec<i32> = Vec::new();

    for _ in 0..iters {
        for (i, _) in current.iter().enumerate() {
            next.push(get_value_for_pos(&current, i));
        }

        current = next;
        next = Vec::new();
    }

    return current.iter().map(|i| i.to_string()).collect();
}

fn get_value_for_pos(current: &Vec<i32>, pos: usize) -> i32 {
    let pattern = build_pattern(current.len(), pos);
    let mut sum = 0;

    for (a, b) in current.iter().zip(pattern.iter()) {
        sum += a * b;
    }

    return (sum % 10).abs();
}

fn build_pattern(length: usize, pos: usize) -> Vec<i32> {
    let base_pattern = vec![0, 1, 0, -1];
    let mut pattern = Vec::new();

    loop {
        if pattern.len() >= length + 1 {
            break;
        }

        for base in base_pattern.iter() {
            for _ in 0..(pos + 1) {
                pattern.push(*base);
            }
        }
    }

    pattern.remove(0);
    pattern.truncate(length + 1);
    return pattern;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let parsed = parse("12345678");
        let result = run_fft(&parsed, 4);
        assert_eq!("01029498", &result[..8]);
    }
}
