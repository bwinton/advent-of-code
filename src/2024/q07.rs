//-----------------------------------------------------
// Setup.

use itertools::Itertools;

static INPUT: &str = include_str!("data/q07.data");

#[derive(Debug)]
enum Operators {
    Plus,
    Times,
    Concatenation,
}

fn run_calculation(ops: &[&Operators], values: &[i64], max: i64) -> bool {
    let mut rv = values[0];

    for (i, value) in values[1..].iter().enumerate() {
        let op = ops[i];
        match op {
            Operators::Plus => {
                rv += value;
            }
            Operators::Times => {
                rv *= value;
            }
            Operators::Concatenation => {
                let mut mult = 1;
                while mult <= *value {
                    mult *= 10;
                }
                rv *= mult;
                rv += value;
            }
        }
        if rv > max {
            break;
        }
    }
    rv == max
}

fn process_data(data: &str, extended: bool) -> i64 {
    let mut rv = 0;
    let mut operators = vec![Operators::Plus, Operators::Times];
    if extended {
        operators.push(Operators::Concatenation);
    }
    for line in data.lines() {
        let (first, rest) = line.split_once(": ").unwrap();
        let first: i64 = first.parse().unwrap();
        let values: Vec<i64> = rest
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        for ops in (0..values.len() - 1)
            .map(|_| operators.iter())
            .multi_cartesian_product()
        {
            if run_calculation(&ops, &values, first) {
                rv += first;
                break;
            }
        }
    }
    rv
}

fn process_data_a(data: &str) -> i64 {
    process_data(data, false)
}

fn process_data_b(data: &str) -> i64 {
    process_data(data, true)
}

//-----------------------------------------------------
// Questions.

q_impl!("7");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
        "
        )),
        3749
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
        "
        )),
        11387
    );
}
