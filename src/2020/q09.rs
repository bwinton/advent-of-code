//-----------------------------------------------------
// Setup.

use itertools::Itertools;
use std::collections::VecDeque;

static INPUT: &str = include_str!("data/q09.data");

fn run_a(data: &str, limit: usize) -> usize {
    let mut previous: VecDeque<usize> = VecDeque::new();
    for line in data.lines() {
        if previous.len() < limit {
            previous.push_front(line.parse().unwrap());
            continue;
        }

        let curr: usize = line.parse().unwrap();
        let mut found = false;
        for values in previous.iter().combinations(2) {
            if values[0] + values[1] == curr {
                found = true;
                break;
            }
        }
        if !found {
            return curr;
        }
        previous.pop_back();
        previous.push_front(curr);
    }
    0
}

fn run_b(data: &str, target: usize) -> usize {
    let mut values = data.lines().map(|x| x.parse::<usize>().unwrap());
    let mut current_sum = 0;
    let mut current_values = VecDeque::new();
    loop {
        while current_sum < target {
            let next = values.next().unwrap();
            current_values.push_back(next);
            current_sum += next;
        }
        while current_sum > target {
            let next = current_values.pop_front().unwrap();
            current_sum -= next;
        }
        if current_sum == target {
            return current_values.iter().min().unwrap() + current_values.iter().max().unwrap();
        }
    }
}

fn process_data_a(data: &str) -> usize {
    run_a(data, 25)
}

fn process_data_b(data: &str) -> usize {
    run_b(data, 375054920)
}

//-----------------------------------------------------
// Questions.

q_impl!("9");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        run_a(
            "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576",
            5
        ),
        127
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        run_b(
            "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576",
            127
        ),
        62
    );
}
