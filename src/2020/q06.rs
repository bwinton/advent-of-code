use std::collections::HashSet;

//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q06.data");

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    let mut group = HashSet::new();
    for line in data.lines() {
        if line.trim().is_empty() {
            rv += group.len();
            group.clear();
            continue;
        }
        group.extend(line.chars());
    }
    rv += group.len();
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    let mut group = HashSet::new();
    let mut first_person = true;
    for line in data.lines() {
        if line.trim().is_empty() {
            rv += group.len();
            group.clear();
            first_person = true;
            continue;
        }
        if first_person {
            first_person = false;
            group = line.chars().collect();
            continue;
        }
        group = group
            .intersection(&line.chars().collect())
            .copied()
            .collect();
    }
    rv += group.len();
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
