use std::collections::HashMap;

type Coordinate = (usize, usize);
type Direction = (i32, i32);

pub fn start(input: &str) {
    let mut map = generate_map(input);
    process_map(&mut map);
    // println!("{:?}", map);
}

fn generate_map(input: &str) -> HashMap<Coordinate, i32> {
    let mut map: HashMap<Coordinate, i32> = HashMap::new();

    for (i, row) in input.trim().split("\n").enumerate() {
        for (j, c) in row.chars().enumerate() {
            if c == '#' {
                map.insert((j, i), 0);
            }
        }
    }

    return map;
}

fn process_map(map: &mut HashMap<Coordinate, i32>) {
    for spotter in map.keys() {
        for spottee in map.keys() {
            println!("Spotter: {:?}, Spottee: {:?}", spotter, spottee);
            let x_diff = spotter.0 as i32 - spottee.0 as i32;
            let y_diff = spotter.1 as i32 - spottee.1 as i32;
            println!("XDiff: {:?}, YDiff: {:?}", x_diff, y_diff);

            if should_check_path(x_diff, y_diff) {
                println!("yes");
            }
        }
    }
}

fn should_check_path(x_diff: i32, y_diff: i32) -> Option<Direction> {
    if x_diff == 0 && y_diff == 0 {
        return false;
    }
    if x_diff == 0 || y_diff == 0 {
        return true;
    }
    if x_diff % y_diff == 0 || y_diff % x_diff == 0 {
        return true;
    }
    return false;
}
