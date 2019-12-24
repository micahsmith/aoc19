use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
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

pub fn start() {
    let mut moons = get_moons();
    let end = simulate(&mut moons, 1000);
    let te = find_total_energy(&end);
    println!("Total energy: {}", te);
}

fn get_moons() -> HashSet<Moon> {
    let mut moons = HashSet::new();
    moons.insert(Moon::new([3, 2, -6], None));
    moons.insert(Moon::new([-13, 18, 10], None));
    moons.insert(Moon::new([-8, -1, 13], None));
    moons.insert(Moon::new([5, 10, 4], None));
    return moons;
}

fn simulate(moons: &HashSet<Moon>, steps: u16) -> HashSet<Moon> {
    let mut current = moons.clone();
    for _ in 0..steps {
        current = apply_gravity(&current);
        current = apply_velocity(&current);
    }
    return current;
}

fn apply_gravity(moons: &HashSet<Moon>) -> HashSet<Moon> {
    let mut next = HashSet::new();
    for first in moons.iter() {
        let mut moon = first.clone();
        for second in moons.iter() {
            if first == second {
                continue;
            } else {
                moon.apply_gravity(second);
            }
        }
        next.insert(moon);
    }
    return next;
}

fn apply_velocity(moons: &HashSet<Moon>) -> HashSet<Moon> {
    let mut next = HashSet::new();
    for first in moons.iter() {
        let mut moon = first.clone();
        moon.apply_velocity();
        next.insert(moon);
    }
    return next;
}

fn find_total_energy(moons: &HashSet<Moon>) -> i32 {
    let mut te = 0;
    for moon in moons.iter() {
        te += moon.get_potential_energy() * moon.get_kinetic_energy();
    }
    return te;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let mut moons = HashSet::new();
        moons.insert(Moon::new([-1, 0, 2], None));
        moons.insert(Moon::new([2, -10, -7], None));
        moons.insert(Moon::new([4, -8, 8], None));
        moons.insert(Moon::new([3, 5, -1], None));

        let end = simulate(&moons, 1);
        println!("{:?}", end);
    }
}
