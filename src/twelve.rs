use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
struct Moon {
    position: [i32; 3],
    velocity: [i32; 3],
}

impl Moon {
    pub fn new(position: [i32; 3], velocity: Option<[i32; 3]>) -> Moon {
        return Moon {
            position: position,
            velocity: velocity.unwrap_or([0, 0, 0]),
        };
    }

    pub fn apply_gravity(&mut self, other: &Self) {
        for i in 0..3 {
            match self.position[i].cmp(&other.position[i]) {
                Ordering::Greater => self.velocity[i] -= 1,
                Ordering::Less => self.velocity[i] += 1,
                Ordering::Equal => (),
            }
        }
    }

    pub fn apply_velocity(&mut self) {
        for i in 0..3 {
            self.position[i] += self.velocity[i];
        }
    }

    pub fn get_potential_energy(&self) -> i32 {
        let mut sum = 0;
        for i in 0..3 {
            sum += self.position[i].abs();
        }
        return sum;
    }

    pub fn get_kinetic_energy(&self) -> i32 {
        let mut sum = 0;
        for i in 0..3 {
            sum += self.velocity[i].abs();
        }
        return sum;
    }
}

type MoonGroup = [Moon; 4];

pub fn start() {
    let mut moons = get_moons();
    let end = simulate_by_steps(&mut moons, 1000);
    let te = find_total_energy(&end);
    println!("Total energy: {}", te);

    let moons = get_moons();
    let step_count = duplicate_simulation(&moons);
    println!("First duplicate: {}", step_count);
}

fn get_moons() -> [Moon; 4] {
    return [
        Moon::new([3, 2, -6], None),
        Moon::new([-13, 18, 10], None),
        Moon::new([-8, -1, 13], None),
        Moon::new([5, 10, 4], None),
    ];
}

fn simulate_by_steps(moons: &MoonGroup, steps: u16) -> MoonGroup {
    let mut current = moons.clone();
    for _ in 0..steps {
        current = apply_gravity(&current);
        current = apply_velocity(&current);
    }
    return current;
}

fn apply_gravity(moons: &MoonGroup) -> MoonGroup {
    let mut next: [Moon; 4] = Default::default();
    for (i, first) in moons.iter().enumerate() {
        let mut moon = first.clone();
        for second in moons.iter() {
            if first == second {
                continue;
            } else {
                moon.apply_gravity(second);
            }
        }
        next[i] = moon;
    }
    return next;
}

fn apply_velocity(moons: &MoonGroup) -> MoonGroup {
    let mut next: [Moon; 4] = Default::default();
    for (i, first) in moons.iter().enumerate() {
        let mut moon = first.clone();
        moon.apply_velocity();
        next[i] = moon;
    }
    return next;
}

fn find_total_energy(moons: &MoonGroup) -> i32 {
    let mut te = 0;
    for moon in moons.iter() {
        te += moon.get_potential_energy() * moon.get_kinetic_energy();
    }
    return te;
}

fn duplicate_simulation(moons: &MoonGroup) -> u64 {
    let mut counter = 0;
    let mut current = moons.clone();
    let mut history: [HashSet<[i32; 8]>; 3] = Default::default();
    let mut steps: [Option<u64>; 3] = Default::default();

    while steps[0].is_none() || steps[1].is_none() || steps[2].is_none() {
        for i in 0..3 {
            if steps[i].is_none() {
                let dimensions = get_dimensions(&current, i);
                if let None = history[i].get(&dimensions) {
                    history[i].insert(dimensions);
                } else {
                    steps[i] = Some(counter);
                }
            }
        }

        counter += 1;
        current = apply_gravity(&current);
        current = apply_velocity(&current);
    }

    return lcm(steps[0].unwrap(), lcm(steps[1].unwrap(), steps[2].unwrap()));
}

fn get_dimensions(moons: &MoonGroup, dimension: usize) -> [i32; 8] {
    let mut by_dimension: [i32; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    for (i, moon) in moons.iter().enumerate() {
        by_dimension[i] = moon.position[dimension];
        by_dimension[i + 4] = moon.velocity[dimension];
    }
    return by_dimension;
}

fn lcm(a: u64, b: u64) -> u64 {
    return (a * b) / gcd(a, b);
}

fn gcd(one: u64, two: u64) -> u64 {
    if two == 0 {
        return one;
    }
    return gcd(two, one - (two * (one / two)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let moons = [
            Moon::new([-8, -10, 0], None),
            Moon::new([5, 5, 10], None),
            Moon::new([2, -7, 3], None),
            Moon::new([9, -8, -3], None),
        ];

        let step_count = duplicate_simulation(&moons);
        assert_eq!(4686774924, step_count);
    }
}
