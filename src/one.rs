fn fuel_mass(input: i32) -> i32 {
    let mass = (input / 3) - 2;

    if mass <= 0 {
        return 0;
    } else {
        return mass + fuel_mass(mass);
    }
}

pub fn start(input: &str) {
    let sum = input
        .split('\n')
        .filter(|&line| line != "")
        .map(|line| {
            let parsed = line.parse::<i32>().unwrap();
            let init_mass = (parsed / 3) - 2;
            init_mass + fuel_mass(init_mass)
        })
        .fold(0, |acc, val| acc + val);

    println!("{}", sum);
}
