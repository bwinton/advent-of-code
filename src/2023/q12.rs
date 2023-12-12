//-----------------------------------------------------
// Setup.

use rayon::{
    iter::ParallelIterator,
    str::{self, ParallelString},
};
use std::collections::{HashMap, VecDeque};

static INPUT: &str = include_str!("data/q12.data");

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[allow(dead_code)]
fn print_line(line: &VecDeque<Condition>) -> String {
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

fn parse(line: &str) -> (VecDeque<Condition>, VecDeque<usize>) {
    let (input, values) = line.split_once(' ').unwrap();
    let input: VecDeque<Condition> = input
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
    let values = values.split(',').map(|v| v.parse().unwrap()).collect();
    (input, values)
}

fn get_combinations(
    base_input: &VecDeque<Condition>,
    base_values: &VecDeque<usize>,
    seen: &mut HashMap<(VecDeque<Condition>, VecDeque<usize>), usize>,
) -> usize {
    if let Some(rv) = seen.get(&(base_input.clone(), base_values.clone())) {
        return *rv;
    }
    let mut input = base_input.clone();
    let mut values = base_values.clone();

    let next = input.pop_front();
    if next.is_none() {
        let rv = if values.is_empty() { 1 } else { 0 };
        seen.insert((base_input.clone(), base_values.clone()), rv);
        return rv;
    }
    let next = next.unwrap();
    let rv = match next {
        Condition::Operational => {
            let rv = get_combinations(&input, &values, seen);
            seen.insert((base_input.clone(), base_values.clone()), rv);
            rv
        }
        Condition::Damaged => {
            if values.is_empty() || (input.len() < values[0] - 1) {
                seen.insert((base_input.clone(), base_values.clone()), 0);
                0
            } else {
                let check = input
                    .iter()
                    .take(values[0] - 1)
                    .all(|&c| c != Condition::Operational);
                let next = input.get(values[0] - 1);

                if check && ((next.is_none()) || (next.unwrap() != &Condition::Damaged)) {
                    input.drain(0..values[0] - 1);
                    if !input.is_empty() && input[0] == Condition::Unknown {
                        input[0] = Condition::Operational;
                    }
                    values.pop_front();
                    let rv = get_combinations(&input, &values, seen);
                    seen.insert((base_input.clone(), base_values.clone()), rv);
                    rv
                } else {
                    seen.insert((base_input.clone(), base_values.clone()), 0);
                    0
                }
            }
        }
        Condition::Unknown => {
            input.push_front(Condition::Operational);
            let count: usize = get_combinations(&input, &values, seen);
            let mut rv = count;

            input[0] = Condition::Damaged;
            let count: usize = get_combinations(&input, &values, seen);
            rv += count;
            seen.insert((base_input.clone(), base_values.clone()), rv);
            rv
        }
    };
    rv
}

fn process_data_a(data: &str) -> usize {
    data.par_lines()
        .map(|line| {
            let (input, values) = parse(line);
            get_combinations(&input, &values, &mut HashMap::new())
        })
        .sum()
}

fn process_data_b(data: &str) -> usize {
    data.par_lines()
        .map(|line| {
            let (input, values) = parse(line);

            let mut input: Vec<_> = input.into();
            input.push(Condition::Unknown);
            input = input.repeat(5);
            input.pop();
            let input: VecDeque<_> = input.into();

            let mut values: Vec<_> = values.into();
            values = values.repeat(5);
            let values: VecDeque<_> = values.into();
            get_combinations(&input, &values, &mut HashMap::new())
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
