pub fn start(input: &str) {
    let parsed = parse(input);
    let result = run_fft(&parsed, 100);
    println!("First eight digits: {}", result);
}

fn parse(input: &str) -> Vec<i32> {
    return input
        .trim()
        .chars()
        .map(|a| a.to_digit(10).unwrap() as i32)
        .collect();
}

fn run_fft(input: &Vec<i32>, iters: usize) -> String {
    let mut current = Vec::new();

    for _ in 0..iters {}

    return String::new();
}

fn build_pattern(pos: usize) -> Vec<i32> {
    let base_pattern = vec![0, 1, 0, -1];
    let mut pattern = Vec::new();

    for base in base_pattern.iter() {
        for _ in 0..(pos + 1) {
            pattern.push(*base);
        }
    }

    pattern.remove(0);
    return pattern;
}
