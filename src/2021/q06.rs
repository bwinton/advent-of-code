use std::collections::HashMap;

//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q06.data");

fn increment(fish: &[u64;9]) -> [u64;9] {
    let mut rv = [0;9];
    for i in 0..8 {
        rv[i] = fish[i + 1];
    }
    rv[6] += fish[0];
    rv[8] += fish[0];
    rv
}

fn process_data_a(data: &str) -> u64 {
    let line: Vec<usize> = data.trim().split(',').map(|x| x.parse().unwrap()).collect();
    let mut fish = [0;9];
    for i in line {
        fish[i] += 1
    }
    for _ in 0..80 {
        fish = increment(&fish);
    }
    fish.iter().sum()
}

fn process_data_b(data: &str) -> u64 {
    let line: Vec<usize> = data.trim().split(',').map(|x| x.parse().unwrap()).collect();
    let mut fish = [0;9];
    for i in line {
        fish[i] += 1
    }
    for _ in 0..256 {
        fish = increment(&fish);
    }
    fish.iter().sum()
}

//-----------------------------------------------------
// Questions.

q_impl!("6");

#[test]
fn a() {
    assert_eq!(process_data_a(indoc!("3,4,3,1,2")), 5934);
}

#[test]
fn b() {
    assert_eq!(process_data_b(indoc!("3,4,3,1,2")), 26_984_457_539);
}
