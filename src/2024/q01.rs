//-----------------------------------------------------
// Setup.

use std::collections::HashMap;

static INPUT: &str = include_str!("data/q01.data");

fn process_data_a(data: &str) -> i32 {
    let mut rv = 0;
    let mut first = vec![];
    let mut second = vec![];
    for line in data.lines() {
        let numbers = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        first.push(numbers[0]);
        second.push(numbers[1]);
    }
    first.sort();
    second.sort();
    for (a, b) in first.iter().zip(&second) {
        let diff = (a - b).abs();
        rv += diff;
    }
    rv
}

fn process_data_b(data: &str) -> i32 {
    let mut rv = 0;
    let mut first = vec![];
    let mut second: HashMap<i32, i32> = HashMap::new();

    for line in data.lines() {
        let numbers = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        first.push(numbers[0]);
        second
            .entry(numbers[1])
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    for value in first {
        rv += value * *second.entry(value).or_default();
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("1");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "3   4
4   3
2   5
1   3
3   9
3   3"
        )),
        11
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "3   4
4   3
2   5
1   3
3   9
3   3"
        )),
        31
    );
}
