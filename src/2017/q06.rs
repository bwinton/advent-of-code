//-----------------------------------------------------
// Setup.

use aoc::Day;

use std::collections::{HashMap, HashSet};

static INPUT: &str = "5	1	10	0	1	7	13	14	3	12	8	10	7	12	0	6";

fn redistribute(cells: &[usize]) -> Vec<usize> {
    let mut rv = cells.to_owned();
    let max = cells.iter().enumerate().rev().max_by_key(|x| x.1).unwrap();
    {
        let elem = &mut rv[max.0];
        *elem = 0;
    }
    {
        for i in max.0 + 1..max.0 + 1 + max.1 {
            let elem = &mut rv[i % cells.len()];
            *elem += 1;
        }
    }
    rv
}

fn process_data_a(data: &str) -> usize {
    let mut cells: Vec<usize> = data
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect();
    let mut rv = 0;
    let mut seen = HashSet::new();
    while !seen.contains(&cells) {
        rv += 1;
        seen.insert(cells.clone());
        cells = redistribute(&cells);
    }

    rv
}

fn process_data_b(data: &str) -> i32 {
    let mut cells: Vec<usize> = data
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect();
    let mut rv = 0;
    let mut seen = HashMap::new();
    while !seen.contains_key(&cells) {
        seen.insert(cells.clone(), rv);
        rv += 1;
        cells = redistribute(&cells);
    }

    rv - seen[&cells]
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
    fn number(&self) -> String {
        String::from("6")
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

    assert_eq!(process_data_a("0 2 7 0"), 5);
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b("2 4 1 2"), 4);
}
