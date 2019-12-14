use std::collections::HashMap;

pub fn start(input: &str) {
    let mut map: HashMap<&str, &str> = HashMap::new();

    input
        .trim()
        .split("\n")
        .map(|orbit| parse_orbit(orbit))
        .for_each(|(orbitted, orbitter)| {
            map.insert(orbitter, orbitted);
        });

    find_number_of_orbits(&map);
    find_shortest_path(&map);
}

fn parse_orbit(orbit: &str) -> (&str, &str) {
    let mut parsed = orbit.split(")");
    return (parsed.next().unwrap(), parsed.next().unwrap());
}

fn find_number_of_orbits(map: &HashMap<&str, &str>) {
    let mut counter = 0;
    for k in map.keys() {
        let mut parent = map.get(k).unwrap();
        counter += 1;
        while *parent != "COM" {
            parent = map.get(parent).unwrap();
            counter += 1;
        }
    }

    println!("Number of direct and indirect orbits: {}", counter);
}

fn find_shortest_path(map: &HashMap<&str, &str>) {
    let mut santa_path = path_to_com("SAN", map);
    let mut you_path = path_to_com("YOU", map);

    while santa_path[0] == you_path[0] {
        santa_path.remove(0);
        you_path.remove(0);
    }

    println!("Transfers required: {}", santa_path.len() + you_path.len());
}

fn path_to_com<'a>(name: &'a str, map: &HashMap<&'a str, &'a str>) -> Vec<&'a str> {
    let mut path: Vec<&str> = Vec::new();
    let mut parent = map.get(name).unwrap();
    while *parent != "COM" {
        path.push(parent);
        parent = map.get(parent).unwrap();
    }
    path.push(parent);
    path.reverse();
    return path;
}
