//-----------------------------------------------------
// Setup.

use std::collections::HashMap;

static INPUT: &str = include_str!("data/q06.data");

fn process_data_a(data: &str) -> i32 {
    let mut orbits: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in data.lines() {
        let mut values = line.split(')');
        let center = values.next().unwrap();
        let planet = values.next().unwrap();
        orbits.entry(center).or_default().push(planet);
    }
    let mut total = 0;
    let mut queue = vec![(&orbits["COM"], 1)];
    while !queue.is_empty() {
        let (curr, level) = queue.pop().unwrap();
        for planet in curr {
            total += level;
            if let Some(next) = orbits.get(planet) {
                queue.push((next, level + 1));
            }
        }
    }

    total
}

fn process_data_b(data: &str) -> usize {
    let mut orbits: HashMap<&str, &str> = HashMap::new();
    for line in data.lines() {
        let mut values = line.split(')');
        let center = values.next().unwrap();
        let planet = values.next().unwrap();
        orbits.insert(planet, center);
    }
    let mut my_path = vec![];
    let mut curr = orbits["YOU"];
    while curr != "COM" {
        my_path.push(curr);
        curr = orbits[curr];
    }
    let mut santa_path = vec![];
    curr = orbits["SAN"];
    while curr != "COM" && !my_path.contains(&curr) {
        santa_path.push(curr);
        curr = orbits[curr];
    }
    my_path = my_path.into_iter().take_while(|&x| x != curr).collect();
    // println!("{:?}, {:?}, {:?}", my_path, curr, santa_path);
    my_path.len() + santa_path.len()
}

//-----------------------------------------------------
// Questions.

q_impl!("6");

#[test]
fn a() {
    assert_eq!(
        process_data_a(
            "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"
        ),
        42
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(
            "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"
        ),
        4
    );
}
