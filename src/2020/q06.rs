use std::collections::{HashMap, HashSet};

//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q06.data");

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    let mut group = HashSet::new();
    for line in data.lines() {
        if line.trim().is_empty() {
            rv += group.len();
            group = HashSet::new();
            continue;
        }
        for char in line.chars() {
            group.insert(char);
        }
    }
    rv += group.len();
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    let mut group = HashMap::new();
    let mut people = 0;
    for line in data.lines() {
        if line.trim().is_empty() {
            let answers: Vec<_> = group.iter().filter(|&(_, &y)| y == people).collect();
            rv += answers.len();
            group = HashMap::new();
            people = 0;
            continue;
        }
        people += 1;
        for char in line.chars() {
            if let Some(value) = group.get_mut(&char) {
                *value += 1;
            } else {
                group.insert(char, 1);
            }
        }
    }
    let answers: Vec<_> = group.iter().filter(|&(_, &y)| y == people).collect();
    rv += answers.len();
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("6");

#[test]
fn a() {
    assert_eq!(
        process_data_a(
            "abcx
abcy
abcz"
        ),
        6
    );
    assert_eq!(
        process_data_a(
            "abc

a
b
c

ab
ac

a
a
a
a

b"
        ),
        11
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(
            "abc

a
b
c

ab
ac

a
a
a
a

b"
        ),
        6
    );
}
