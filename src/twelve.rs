use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Moon {
    position: (i32, i32, i32),
    velocity: (i32, i32, i32),
}

impl Moon {
    pub fn new(x: i32, y: i32, z: i32) -> Moon {
        return Moon {
            position: (x, y, z),
            velocity: (0, 0, 0),
        };
    }
}

pub fn start() {
    let moons = get_moons();
    simulate(moons, 10);

    println!("{:?}", moons);
}

fn get_moons() -> HashSet<Moon> {
    let mut moons = HashSet::new();
    moons.insert(Moon::new(3, 2, -6));
    moons.insert(Moon::new(-13, 18, 10));
    moons.insert(Moon::new(-8, -1, 13));
    moons.insert(Moon::new(5, 10, 4));
    return moons;
}

fn simulate(mut moons: HashSet<Moon>, steps: i8) {
    for _ in 0..steps {
        apply_gravity(&mut moons);
        apply_velocity(&mut moons);
    }
}

fn apply_gravity(moons: &mut HashSet<Moon>) {
    for moon in moons.iter() {}
}

fn apply_velocity(moons: &mut HashSet<Moon>) {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let mut moons = HashSet::new();
        moons.insert(Moon::new(-1, 0, 2));
        moons.insert(Moon::new(2, -10, -7));
        moons.insert(Moon::new(4, -8, 8));
        moons.insert(Moon::new(3, 5, -1));
    }
}
