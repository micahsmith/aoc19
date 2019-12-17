use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        return Point { x, y };
    }

    fn add(&self, v: &Vector) -> Point {
        return Point {
            x: (self.x as i32 + v.x) as usize,
            y: (self.y as i32 + v.y) as usize,
        };
    }
}

#[derive(Debug)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    fn new(one: &Point, two: &Point) -> Option<Vector> {
        let x = one.x as i32 - two.x as i32;
        let y = one.y as i32 - two.y as i32;

        if x == 0 {
            return Some(Vector {
                x: x,
                y: if y.is_negative() { 1 } else { -1 },
            });
        }
        if y == 0 {
            return Some(Vector {
                x: if x.is_negative() { 1 } else { -1 },
                y: y,
            });
        }
        if x % y == 0 {
            return Some(Vector {
                x: -x / y.abs(),
                y: -y / y.abs(),
            });
        }
        if y % x == 0 {
            return Some(Vector {
                x: -x / x.abs(),
                y: -y / x.abs(),
            });
        }

        return None;
    }
}

pub fn start(input: &str) {
    let set = generate_map(input);
    let max = process_map(&set);
    println!("Maximum: {}", max);
}

fn generate_map(input: &str) -> HashSet<Point> {
    let mut set: HashSet<Point> = HashSet::new();

    for (i, row) in input.trim().split("\n").enumerate() {
        for (j, c) in row.chars().enumerate() {
            if c == '#' {
                set.insert(Point::new(j, i));
            }
        }
    }

    return set;
}

fn process_map(set: &HashSet<Point>) -> i32 {
    let mut max = 0;

    for spotter in set.iter() {
        let mut spottable = 0;
        for spottee in set.iter() {
            if spotter == spottee {
                continue;
            } else if is_spottable(set, spotter, spottee) {
                spottable += 1;
            }
        }

        if spottable > max {
            max = spottable;
        }
    }

    return max;
}

fn is_spottable(set: &HashSet<Point>, spotter: &Point, spottee: &Point) -> bool {
    println!("Spotter: {:?}, Spottee: {:?}", spotter, spottee);
    match Vector::new(spotter, spottee) {
        Some(v) => {
            println!("Vector: {:?}", v);
            let mut check_point = spotter.add(&v);
            loop {
                println!("Check point: {:?}", check_point);
                if spottee == &check_point {
                    println!("Can be seen!");
                    return true;
                } else if let Some(_) = set.get(&check_point) {
                    println!("Blocked!");
                    return false;
                } else {
                    check_point = check_point.add(&v);
                }
            }
        }
        None => {
            return true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input = ".#..#\n.....\n#####\n....#\n...##";
        let set = generate_map(input);
        assert_eq!(process_map(&set), 8);
    }

    #[test]
    fn test_two() {
        let input = "......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n\
                     ..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####";
        let set = generate_map(input);
        assert_eq!(process_map(&set), 33);
    }

    #[test]
    fn test_three() {
        let input = "#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n\
                     .##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.";
        let set = generate_map(input);
        assert_eq!(process_map(&set), 35);
    }
}
