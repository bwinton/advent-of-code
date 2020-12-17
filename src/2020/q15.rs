use std::collections::{HashMap, VecDeque};

//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q15.data");

fn run(data: &str, iterations: usize) -> usize {
    let numbers: Vec<usize> = data.split(',').map(|x| x.parse().unwrap()).collect();
    let mut spoken = HashMap::new();
    let mut prev = 0;
    for i in 0..iterations {
        if i < numbers.len() {
            let mut value = VecDeque::new();
            value.push_front(i + 1);
            spoken.insert(numbers[i], value);
            prev = numbers[i];
            continue;
        }

        let occurances = spoken.entry(prev).or_insert_with(VecDeque::new);
        let value = if occurances.len() == 2 {
            occurances[0] - occurances[1]
        } else {
            0
        };
        let occurances = spoken.entry(value).or_insert_with(VecDeque::new);
        occurances.push_front(i + 1);
        if occurances.len() > 2 {
            occurances.pop_back();
        }
        prev = value;
    }
    prev
}

fn process_data_a(data: &str) -> usize {
    run(data, 2020)
}

fn process_data_b(data: &str) -> usize {
    run(data, 30_000_000)
}

//-----------------------------------------------------
// Questions.

q_impl!("15");

#[test]
fn a() {
    assert_eq!(process_data_a("0,3,6"), 436);
}

#[test]
#[ignore]
fn b() {
    // I'm brute forcing it, so these take way too long!
    assert_eq!(process_data_b("0,3,6"), 175_594);
    assert_eq!(process_data_b("1,3,2"), 2_578);
    assert_eq!(process_data_b("2,1,3"), 3_544_142);
    assert_eq!(process_data_b("1,2,3"), 261_214);
    assert_eq!(process_data_b("2,3,1"), 6_895_259);
    assert_eq!(process_data_b("3,2,1"), 18);
    assert_eq!(process_data_b("3,1,2"), 362);
}
