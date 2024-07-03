//-----------------------------------------------------
// Setup.

use rayon::{
    iter::ParallelIterator,
    str::{self, ParallelString},
};
use std::collections::HashMap;

static INPUT: &str = include_str!("data/q12.data");

type Key<'a> = (&'a [Condition], &'a [usize]);

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[allow(dead_code)]
fn print_line(line: &[Condition]) -> String {
    let mut rv = String::new();
    for &c in line.iter() {
        match c {
            Condition::Damaged => {
                rv += "#";
            }
            Condition::Operational => {
                rv += ".";
            }
            Condition::Unknown => {
                rv += "?";
            }
        }
    }
    rv
}

fn parse(line: &str) -> (Vec<Condition>, Vec<usize>) {
    let (input, values) = line.split_once(' ').unwrap();
    let input: Vec<Condition> = input
        .chars()
        .map(|c| match c {
            '#' => Condition::Damaged,
            '.' => Condition::Operational,
            '?' => Condition::Unknown,
            _ => {
                panic!("Unknown map char {}", c);
            }
        })
        .collect();
    let values: Vec<usize> = values.split(',').map(|v| v.parse().unwrap()).collect();
    (input, values)
}

fn get_combinations<'a>(
    base_input: &'a [Condition],
    values: &'a [usize],
    seen: &mut HashMap<Key<'a>, usize>,
) -> usize {
    if let Some(rv) = seen.get(&(base_input, values)) {
        return *rv;
    }

    let next = base_input.first();
    if next.is_none() {
        let rv = if values.is_empty() { 1 } else { 0 };
        seen.insert((base_input, values), rv);
        return rv;
    }
    let input = &base_input[1..];
    let next = next.unwrap();
    match next {
        Condition::Operational => handle_operational(input, values, seen, (base_input, values)),
        Condition::Damaged => handle_damaged(values, input, seen, (base_input, values)),
        Condition::Unknown => {
            let count: usize = handle_operational(input, values, seen, (base_input, values));
            let mut rv = count;

            let count: usize = handle_damaged(values, input, seen, (base_input, values));
            rv += count;
            seen.insert((base_input, values), rv);
            rv
        }
    }
}

fn handle_damaged<'a>(
    mut values: &'a [usize],
    mut input: &'a [Condition],
    seen: &mut HashMap<Key<'a>, usize>,
    base: Key<'a>,
) -> usize {
    if values.is_empty() || (input.len() < values[0] - 1) {
        seen.insert(base, 0);
        0
    } else {
        let check = input
            .iter()
            .take(values[0] - 1)
            .all(|&c| c != Condition::Operational);
        let next = input.get(values[0] - 1);

        if check && ((next.is_none()) || (next.unwrap() != &Condition::Damaged)) {
            input = &input[values[0] - 1..];
            if !input.is_empty() && input[0] == Condition::Unknown {
                input = &input[1..];
            }
            values = &values[1..];
            let rv = get_combinations(input, values, seen);
            seen.insert(base, rv);
            rv
        } else {
            seen.insert(base, 0);
            0
        }
    }
}

fn handle_operational<'a>(
    input: &'a [Condition],
    values: &'a [usize],
    seen: &mut HashMap<Key<'a>, usize>,
    key: Key<'a>,
) -> usize {
    let rv = get_combinations(input, values, seen);
    seen.insert(key, rv);
    rv
}

fn process_data_a(data: &str) -> usize {
    data.par_lines()
        .map(|line| {
            let (input, values) = parse(line);
            // 230 derived experimentally.
            get_combinations(&input, &values, &mut HashMap::with_capacity(230))
        })
        .sum()
}

fn process_data_b(data: &str) -> usize {
    data.par_lines()
        .map(|line| {
            let (mut input, mut values) = parse(line);

            input.push(Condition::Unknown);
            input = input.repeat(5);
            input.pop();

            values = values.repeat(5);
            // 4440 derived experimentally.
            get_combinations(&input, &values, &mut HashMap::with_capacity(4440))
        })
        .sum()
}

//-----------------------------------------------------
// Questions.

q_impl!("12");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_a(indoc!("???.### 1,1,3")), 1);
    assert_eq!(process_data_a(indoc!(".??..??...?##. 1,1,3")), 4);
    assert_eq!(process_data_a(indoc!("?#?#?#?#?#?#?#? 1,3,1,6")), 1);
    assert_eq!(process_data_a(indoc!("????.#...#... 4,1,1")), 1);
    assert_eq!(process_data_a(indoc!("????.######..#####. 1,6,5")), 4);
    assert_eq!(process_data_a(indoc!("?###???????? 3,2,1")), 10);

    assert_eq!(
        process_data_a(indoc!(
            "???.### 1,1,3
    .??..??...?##. 1,1,3
    ?#?#?#?#?#?#?#? 1,3,1,6
    ????.#...#... 4,1,1
    ????.######..#####. 1,6,5
    ?###???????? 3,2,1
    "
        )),
        21
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "???.### 1,1,3
    .??..??...?##. 1,1,3
    ?#?#?#?#?#?#?#? 1,3,1,6
    ????.#...#... 4,1,1
    ????.######..#####. 1,6,5
    ?###???????? 3,2,1
    "
        )),
        525152
    );
}
