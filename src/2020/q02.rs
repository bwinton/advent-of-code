//-----------------------------------------------------
// Setup.

use once_cell::sync::Lazy;
use regex::Regex;

static INPUT: &str = include_str!("data/q02.data");

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)-(\d+) (.): (.*)").unwrap());

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    for line in data.lines() {
        let cap = RE.captures(line).unwrap();
        let low: usize = cap[1].parse().unwrap();
        let high: usize = cap[2].parse().unwrap();
        let character = &cap[3].chars().next().unwrap();
        let password = &cap[4];
        let count = password.chars().filter(|x| x == character).count();
        if count >= low && count <= high {
            rv += 1;
        }
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    for line in data.lines() {
        let cap = RE.captures(line).unwrap();
        let low: usize = cap[1].parse().unwrap();
        let high: usize = cap[2].parse().unwrap();
        let character = cap[3].chars().next().unwrap();
        let password = &cap[4];
        let mut chars = password.chars();
        let first = chars.nth(low - 1).unwrap();
        let second = chars.nth(high - low - 1).unwrap();
        if (first == character) ^ (second == character) {
            rv += 1;
        }
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("2");

#[test]
fn a() {
    assert_eq!(
        process_data_a(
            "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"
        ),
        2
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(
            "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"
        ),
        1
    );
}
