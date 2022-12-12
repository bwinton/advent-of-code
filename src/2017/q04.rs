//-----------------------------------------------------
// Setup.

use aoc::Day;

use std::collections::HashSet;

static INPUT: &str = include_str!("data/q04.data");

fn process_data_a(data: &str) -> i32 {
    let mut rv = 0;
    'line: for line in data.lines() {
        let tokens = line.split_whitespace();
        let mut values = HashSet::new();
        for token in tokens {
            if !values.insert(token) {
                break 'line;
            }
        }
        rv += 1
    }
    rv
}

fn process_data_b(data: &str) -> i32 {
    let mut rv = 0;
    'line: for line in data.lines() {
        let tokens = line.split_whitespace();
        let mut values = HashSet::new();
        for token in tokens {
            let mut sorted = token.as_bytes().to_owned();
            sorted.sort_unstable();
            if !values.insert(sorted) {
                break 'line;
            }
        }
        rv += 1
    }
    rv
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
    fn number(&self) -> String {
        String::from("4")
    }

    fn a(&self) {
        print!("{}A: ", self.number());
        let result = process_data_a(INPUT);
        println!("Result = {}", result);
    }

    fn b(&self) {
        print!("{}B: ", self.number());
        let result = process_data_b(INPUT);
        println!("Result = {}", result);
    }
}

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_a("aa bb cc dd ee"), 1);
    assert_eq!(process_data_a("aa bb cc dd aa"), 0);
    assert_eq!(process_data_a("aa bb cc dd aaa"), 1);
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b("abcde fghij"), 1);
    assert_eq!(process_data_b("abcde xyz ecdab"), 0);
    assert_eq!(process_data_b("a ab abc abd abf abj"), 1);
    assert_eq!(process_data_b("iiii oiii ooii oooi oooo"), 1);
    assert_eq!(process_data_b("oiii ioii iioi iiio"), 0);
}
