//-----------------------------------------------------
// Setup.

use std::collections::{hash_map::RandomState, HashSet};

static INPUT: &str = include_str!("data/q03.data");

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    for line in data.lines() {
        let (a, b) = line.split_at(line.len() / 2);
        let a: HashSet<char, RandomState> = HashSet::from_iter(a.chars());
        let b = HashSet::from_iter(b.chars());
        let common = &a & &b;
        let first = *common.iter().next().unwrap();
        rv += match first {
            'a'..='z' => first as usize - 'a' as usize + 1,
            'A'..='Z' => first as usize - 'A' as usize + 27,
            _ => panic!("Invalid char, {}", first),
        }
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    let lines = data.lines().collect::<Vec<_>>();
    for chunk in lines.chunks(3) {
        let a: HashSet<char, RandomState> = HashSet::from_iter(chunk[0].chars());
        let b: HashSet<char, RandomState> = HashSet::from_iter(chunk[1].chars());
        let c: HashSet<char, RandomState> = HashSet::from_iter(chunk[2].chars());
        let common = &(&a & &b) & &c;
        let first = *common.iter().next().unwrap();
        rv += match first {
            'a'..='z' => first as usize - 'a' as usize + 1,
            'A'..='Z' => first as usize - 'A' as usize + 27,
            _ => panic!("Invalid char, {}", first),
        }
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("3");

#[test]
fn a() {
    assert_eq!(
        process_data_a(indoc!(
            "vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw
    "
        )),
        157
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(indoc!(
            "vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw
    "
        )),
        70
    );
}
