//-----------------------------------------------------
// Setup.

use regex::Regex;
use std::collections::HashSet;

static INPUT: &str = include_str!("data/q05.data");

fn has_three_vowels(line: &str) -> bool {
    let re: &Regex = regex!(r"[aeiou].*[aeiou].*[aeiou]");
    re.is_match(line)
}

fn has_pair(line: &str) -> bool {
    let mut prev = '\0';
    let mut rv = false;
    for curr in line.chars() {
        if prev == curr {
            rv = true;
            break;
        }
        prev = curr;
    }
    rv
}

fn is_disallowed(line: &str) -> bool {
    let re: &Regex = regex!(r"ab|cd|pq|xy");
    re.is_match(line)
}

fn has_two_pair(line: &str) -> bool {
    let mut rv = false;
    let mut seen_pairs = HashSet::new();
    let mut prev_pair = ('\0', '\0');
    let one = line.chars();
    let two = line.chars().skip(1);
    let pairs = one.zip(two);
    for pair in pairs {
        if pair == prev_pair {
            continue;
        }
        prev_pair = pair;
        if seen_pairs.contains(&pair) {
            rv = true;
            break;
        }
        seen_pairs.insert(pair);
    }
    rv
}

fn has_aba(line: &str) -> bool {
    let mut rv = false;
    let mut remaining = line.chars();
    let mut a = remaining.next().unwrap();
    let mut b = remaining.next().unwrap();
    for curr in remaining {
        if a == curr {
            rv = true;
            break;
        }
        a = b;
        b = curr;
    }
    rv
}

fn process_data_a(data: &str) -> i32 {
    let mut rv = 0;
    for line in data.lines() {
        let curr = !is_disallowed(line) && has_three_vowels(line) && has_pair(line);
        if curr {
            rv += 1;
        }
    }
    rv
}

fn process_data_b(data: &str) -> i32 {
    let mut rv = 0;
    for line in data.lines() {
        let curr = has_two_pair(line) && has_aba(line);
        if curr {
            rv += 1;
        }
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("5");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_a("ugknbfddgicrmopn"), 1);
    assert_eq!(process_data_a("aaa"), 1);
    assert_eq!(process_data_a("jchzalrnumimnmhp"), 0);
    assert_eq!(process_data_a("haegwjzuvuyypxyu"), 0);
    assert_eq!(process_data_a("dvszwmarrgswjxmb"), 0);
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b("qjhvhtzxzqqjkmpb"), 1);
    assert_eq!(process_data_b("xxyxx"), 1);
    assert_eq!(process_data_b("uurcxstgmygtbstg"), 0);
    assert_eq!(process_data_b("ieodomkazucvgmuy"), 0);
}
