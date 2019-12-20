use std::cmp::Ordering;
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

#[derive(Eq, PartialEq, PartialOrd, Ord)]
enum Quadrant {
    One,
    Two,
    Three,
    Four,
}

#[derive(Debug, Eq, PartialEq)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    fn new(one: &Point, two: &Point) -> Vector {
        let x = one.x as i32 - two.x as i32;
        let y = one.y as i32 - two.y as i32;

        let divisor = -Vector::gcd(x.abs(), y.abs());

        return Vector {
            x: x / divisor,
            y: y / divisor,
        };
    }

    fn gcd(one: i32, two: i32) -> i32 {
        if two == 0 {
            return one;
        }
        return Vector::gcd(two, one - (two * (one / two)));
    }

    fn get_quadrant(&self) -> Quadrant {
        if self.x >= 0 && self.y < 0 {
            return Quadrant::One;
        }
        if self.x >= 0 && self.y >= 0 {
            return Quadrant::Two;
        }
        if self.x < 0 && self.y >= 0 {
            return Quadrant::Three;
        }
        if self.x < 0 && self.y < 0 {
            return Quadrant::Four;
        }
        panic!("Unknown quadrant");
    }

    fn cmp_quadrant(&self, other: &Vector) -> Ordering {
        return self.get_quadrant().cmp(&other.get_quadrant());
    }

    fn std_ordering(&self, other: &Vector) -> Ordering {
        let s = (self.x as f32 / self.y as f32).abs();
        let o = (other.x as f32 / other.y as f32).abs();
        if s < o {
            return Ordering::Less;
        }
        if s > o {
            return Ordering::Greater;
        } 
        return Ordering::Equal;
    }
}

impl Ord for Vector {
    fn cmp(&self, other: &Vector) -> Ordering {
        match self.cmp_quadrant(&other) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => match self.get_quadrant() {
                Quadrant::One => self.std_ordering(&other),
                Quadrant::Two => self.std_ordering(&other),
                Quadrant::Three => self.std_ordering(&other).reverse(), 
                Quadrant::Four => self.std_ordering(&other).reverse(),
            },
        }
    }
}

impl PartialOrd for Vector {
    fn partial_cmp(&self, other: &Vector) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

pub fn start(input: &str) {
    let set = generate_map(input);
    let (max, max_point) = process_map(&set);

    println!("Maximum spottable: {}", max);

    let vectors = generate_vectors(&set, &max_point);
    println!("Vectors: {:?}", vectors);
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

fn process_map(set: &HashSet<Point>) -> (i32, Point) {
    let mut max = 0;
    let mut max_point: Point = Point::new(0, 0);

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
            max_point = *spotter;
        }
    }

    return (max, max_point);
}

fn is_spottable(set: &HashSet<Point>, spotter: &Point, spottee: &Point) -> bool {
    let v = Vector::new(spotter, spottee);
    let mut check_point = spotter.add(&v);
    loop {
        if spottee == &check_point {
            return true;
        }
        if let Some(_) = set.get(&check_point) {
            return false;
        }
        check_point = check_point.add(&v);
    }
}

fn generate_vectors(set: &HashSet<Point>, station: &Point) -> Vec<Vector> {
    let mut vectors = Vec::new();

    for asteroid in set.iter() {
        if station == asteroid {
            continue;
        } else {
            vectors.push(Vector::new(station, asteroid));
        }
    }

    vectors.sort();
    return vectors;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input = "#####\n.....\n.####\n....#\n...##";
        let set = generate_map(input);
        let (max, _) = process_map(&set);
        assert_eq!(max, 10);
    }

    #[test]
    fn test_two() {
        let input = ".#..#\n.....\n#####\n....#\n...##";
        let set = generate_map(input);
        let (max, _) = process_map(&set);
        assert_eq!(max, 8);
    }

    #[test]
    fn test_three() {
        let input = "......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n\
                     ..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####";
        let set = generate_map(input);
        let (max, _) = process_map(&set);
        assert_eq!(max, 33);
    }

    #[test]
    fn test_four() {
        let input = "#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n\
                     .##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.";
        let set = generate_map(input);
        let (max, _) = process_map(&set);
        assert_eq!(max, 35);
    }

    #[test]
    fn test_five() {
        let input = ".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n\
                     ....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..";
        let set = generate_map(input);
        let (max, _) = process_map(&set);
        assert_eq!(max, 41);
    }
}
