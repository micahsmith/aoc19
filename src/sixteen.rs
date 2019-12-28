pub fn start(input: &str) {
    let parsed = parse(input);
    println!("{:?}", parsed);
}

fn parse(input: &str) -> Vec<i32> {
    return input
        .trim()
        .chars()
        .map(|a| a.to_digit(10).unwrap() as i32)
        .collect();
}
