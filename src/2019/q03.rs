//-----------------------------------------------------
// Setup.

use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("data/q03.data");

fn process_data_a(data: &str) -> i32 {
    let mut lines = data.lines();
    let line1 = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| (x.chars().next().unwrap(), x[1..].parse::<i32>().unwrap()));
    let line2 = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| (x.chars().next().unwrap(), x[1..].parse::<i32>().unwrap()));
    let mut current = (0, 0);
    let mut points1: HashSet<(i32, i32)> = HashSet::new();
    for direction in line1 {
        match direction.0 {
            'L' => {
                for i in 1..=direction.1 {
                    points1.insert((current.0 - i, current.1));
                }
                current.0 -= direction.1
            }
            'R' => {
                for i in 1..=direction.1 {
                    points1.insert((current.0 + i, current.1));
                }
                current.0 += direction.1
            }
            'U' => {
                for i in 1..=direction.1 {
                    points1.insert((current.0, current.1 + i));
                }
                current.1 += direction.1
            }
            'D' => {
                for i in 1..=direction.1 {
                    points1.insert((current.0, current.1 - i));
                }
                current.1 -= direction.1
            }
            _ => {
                println!("ERROR!!!");
                return -1;
            }
        }
    }
    current = (0, 0);
    let mut points2: HashSet<(i32, i32)> = HashSet::new();
    for direction in line2 {
        match direction.0 {
            'L' => {
                for i in 1..=direction.1 {
                    points2.insert((current.0 - i, current.1));
                }
                current.0 -= direction.1
            }
            'R' => {
                for i in 1..=direction.1 {
                    points2.insert((current.0 + i, current.1));
                }
                current.0 += direction.1
            }
            'U' => {
                for i in 1..=direction.1 {
                    points2.insert((current.0, current.1 + i));
                }
                current.1 += direction.1
            }
            'D' => {
                for i in 1..=direction.1 {
                    points2.insert((current.0, current.1 - i));
                }
                current.1 -= direction.1
            }
            _ => {
                println!("ERROR!!!");
                return -1;
            }
        }
    }
    let result = points1
        .intersection(&points2)
        .min_by_key(|x| x.0.abs() + x.1.abs())
        .unwrap();
    result.0.abs() + result.1.abs()
}

fn process_data_b(data: &str) -> i32 {
    let mut lines = data.lines();
    let line1 = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| (x.chars().next().unwrap(), x[1..].parse::<i32>().unwrap()));
    let line2 = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| (x.chars().next().unwrap(), x[1..].parse::<i32>().unwrap()));
    let mut current = (0, 0);
    let mut points1: HashMap<(i32, i32), i32> = HashMap::new();
    let mut steps: i32 = 0;
    for direction in line1 {
        match direction.0 {
            'L' => {
                for i in 1..=direction.1 {
                    steps += 1;
                    points1.entry((current.0 - i, current.1)).or_insert(steps);
                }
                current.0 -= direction.1
            }
            'R' => {
                for i in 1..=direction.1 {
                    steps += 1;
                    points1.entry((current.0 + i, current.1)).or_insert(steps);
                }
                current.0 += direction.1
            }
            'U' => {
                for i in 1..=direction.1 {
                    steps += 1;
                    points1.entry((current.0, current.1 + i)).or_insert(steps);
                }
                current.1 += direction.1
            }
            'D' => {
                for i in 1..=direction.1 {
                    steps += 1;
                    points1.entry((current.0, current.1 - i)).or_insert(steps);
                }
                current.1 -= direction.1
            }
            _ => {
                println!("ERROR!!!");
                return -1;
            }
        }
    }
    current = (0, 0);
    steps = 0;
    let mut intersections: HashMap<(i32, i32), i32> = HashMap::new();
    for direction in line2 {
        match direction.0 {
            'L' => {
                for i in 1..=direction.1 {
                    steps += 1;
                    let key = (current.0 - i, current.1);
                    if points1.contains_key(&key) && !intersections.contains_key(&key) {
                        let total = points1.get(&key).unwrap() + steps;
                        intersections.insert(key, total);
                    }
                }
                current.0 -= direction.1
            }
            'R' => {
                for i in 1..=direction.1 {
                    steps += 1;
                    let key = (current.0 + i, current.1);
                    if points1.contains_key(&key) && !intersections.contains_key(&key) {
                        let total = points1.get(&key).unwrap() + steps;
                        intersections.insert(key, total);
                    }
                }
                current.0 += direction.1
            }
            'U' => {
                for i in 1..=direction.1 {
                    steps += 1;
                    let key = (current.0, current.1 + i);
                    if points1.contains_key(&key) && !intersections.contains_key(&key) {
                        let total = points1.get(&key).unwrap() + steps;
                        intersections.insert(key, total);
                    }
                }
                current.1 += direction.1
            }
            'D' => {
                for i in 1..=direction.1 {
                    steps += 1;
                    let key = (current.0, current.1 - i);
                    if points1.contains_key(&key) && !intersections.contains_key(&key) {
                        let total = points1.get(&key).unwrap() + steps;
                        intersections.insert(key, total);
                    }
                }
                current.1 -= direction.1
            }
            _ => {
                println!("ERROR!!!");
                return -1;
            }
        }
    }
    *intersections.values().min().unwrap()
}

//-----------------------------------------------------
// Questions.

q_impl!("3");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(
            "R8,U5,L5,D3
U7,R6,D4,L4"
        ),
        6
    );
    assert_eq!(
        process_data_a(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"
        ),
        159
    );
    assert_eq!(
        process_data_a(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
        ),
        135
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(
            "R8,U5,L5,D3
U7,R6,D4,L4"
        ),
        30
    );
    assert_eq!(
        process_data_b(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"
        ),
        610
    );
    assert_eq!(
        process_data_b(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
        ),
        410
    );
}
