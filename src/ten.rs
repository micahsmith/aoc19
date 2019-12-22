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

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
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

    fn get_ordering(&self, other: &Vector) -> Ordering {
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
                Quadrant::One | Quadrant::Two => self.get_ordering(&other),
                Quadrant::Three | Quadrant::Four => self.get_ordering(&other).reverse(),
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
    engage_lasers(&set, &vectors, &max_point);
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

fn engage_lasers(set: &HashSet<Point>, vectors: &Vec<Vector>, station: &Point) {
    let mut field = set.clone();
    let mut destroyed: Vec<Vec<Point>> = Vec::new();

    let mut prev_counter = 0;
    let mut prev_vector: Vector = vectors.last().unwrap().clone();
    for vector in vectors.iter() {
        if vector == &prev_vector {
            prev_counter += 1;
        } else {
            prev_counter = 0;
        }

        let point = get_first_by_vector(set, station, vector);
        field.remove(&point);
        if let Some(v) = destroyed.get_mut(prev_counter) {
            v.push(point);
        } else {
            destroyed.push(Vec::new());
            destroyed[prev_counter].push(point);
        }

        prev_vector = *vector;
    }

    let flattened: Vec<Point> = destroyed.into_iter().flatten().collect();
    let two_hundredth = flattened[199];
    println!(
        "Coordinates 200th: {}",
        two_hundredth.x * 100 + two_hundredth.y
    );
}

fn get_first_by_vector(set: &HashSet<Point>, station: &Point, vector: &Vector) -> Point {
    let mut check_point = station.add(&vector);
    loop {
        if let Some(p) = set.get(&check_point) {
            return p.clone();
        }
        check_point = check_point.add(&vector);
    }
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
