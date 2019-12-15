//-----------------------------------------------------
// Setup.

use std::collections::{HashSet, VecDeque};

use crate::intcode::{Intcode, State};

static INPUT: &str = include_str!("data/q15.data");

fn process_data_a(data: &str) -> usize {
    let ints: Vec<i128> = data
        .split(',')
        .map(|i| i.parse::<i128>().unwrap())
        .collect();
    let mut paths: VecDeque<(Vec<i128>, (i128, i128), Intcode)> = VecDeque::new();
    let machine = Intcode::new(ints, vec![]);
    let cell: (i128, i128) = (0, 0);
    let mut seen: HashSet<(i128, i128)> = HashSet::new();
    seen.insert(cell);
    for i in 1..=4 {
        let mut next = machine.clone();
        next.inputs.push_back(i);
        paths.push_back((vec![i], cell, next));
    }
    loop {
        let (path, mut cell, mut curr) = paths.pop_front().unwrap();
        match path.last().unwrap() {
            1 => cell.1 -= 1,
            2 => cell.1 += 1,
            3 => cell.0 -= 1,
            4 => cell.0 += 1,
            _ => {
                println!("ERROR!!! {:?}", curr.outputs);
                return 0;
            }
        }
        if !seen.insert(cell) {
            continue;
        }

        match curr.run_tape() {
            Ok(State::Halted) => {
                println!("ERROR!!! {:?}", curr.outputs);
                return 0;
            }
            Ok(State::WaitingForInput) => {
                let rv = curr.outputs.pop_front().unwrap_or(0);
                if rv == 0 {
                    continue;
                } else if rv == 2 {
                    // Found it!
                    return path.len();
                }
                for i in 1..=4 {
                    let mut next = curr.clone();
                    next.inputs.push_back(i);
                    let mut path2 = path.clone();
                    path2.push(i);
                    paths.push_back((path2, cell, next));
                }
            }
            Err(code) => {
                println!("ERROR!!! {}", code);
                return 0;
            }
            Ok(state) => {
                println!("ERROR, machine not halted! {:?}", state);
                return 0;
            }
        }
    }
}

fn process_data_b(data: &str) -> usize {
    let ints: Vec<i128> = data
        .split(',')
        .map(|i| i.parse::<i128>().unwrap())
        .collect();
    let mut paths: VecDeque<(Vec<i128>, (i128, i128), Intcode)> = VecDeque::new();
    let mut machine = Intcode::new(ints, vec![]);
    let mut cell: (i128, i128) = (0, 0);
    let mut seen: HashSet<(i128, i128)> = HashSet::new();
    seen.insert(cell);
    for i in 1..=4 {
        let mut next = machine.clone();
        next.inputs.push_back(i);
        paths.push_back((vec![i], cell, next));
    }
    loop {
        let (path, mut next_cell, mut curr) = paths.pop_front().unwrap();
        match path.last().unwrap() {
            1 => next_cell.1 -= 1,
            2 => next_cell.1 += 1,
            3 => next_cell.0 -= 1,
            4 => next_cell.0 += 1,
            _ => {
                println!("ERROR!!! {:?}", curr.outputs);
                return 0;
            }
        }
        if !seen.insert(next_cell) {
            continue;
        }

        match curr.run_tape() {
            Ok(State::Halted) => {
                println!("ERROR!!! {:?}", curr.outputs);
                return 0;
            }
            Ok(State::WaitingForInput) => {
                let rv = curr.outputs.pop_front().unwrap_or(0);
                if rv == 0 {
                    continue;
                } else if rv == 2 {
                    // Found it!
                    cell = next_cell;
                    machine = curr;
                    break;
                }
                for i in 1..=4 {
                    let mut next = curr.clone();
                    next.inputs.push_back(i);
                    let mut path2 = path.clone();
                    path2.push(i);
                    paths.push_back((path2, next_cell, next));
                }
            }
            Err(code) => {
                println!("ERROR!!! {}", code);
                return 0;
            }
            Ok(state) => {
                println!("ERROR, machine not halted! {:?}", state);
                return 0;
            }
        }
    }

    // We have the oxygen diffuser in cell, and we have the machine located there in machine, so diffuse from there!
    let mut paths: VecDeque<(Vec<i128>, (i128, i128), Intcode)> = VecDeque::new();
    let mut seen: HashSet<(i128, i128)> = HashSet::new();
    let mut max = 0;
    seen.insert(cell);
    for i in 1..=4 {
        let mut next = machine.clone();
        next.inputs.push_back(i);
        paths.push_back((vec![i], cell, next));
    }
    loop {
        if paths.is_empty() {
            break;
        }
        let (path, mut next_cell, mut curr) = paths.pop_front().unwrap();
        match path.last().unwrap() {
            1 => next_cell.1 -= 1,
            2 => next_cell.1 += 1,
            3 => next_cell.0 -= 1,
            4 => next_cell.0 += 1,
            _ => {
                println!("ERROR!!! {:?}", curr.outputs);
                return 0;
            }
        }
        if !seen.insert(next_cell) {
            continue;
        }
        if path.len() > max {
            max = path.len();
        }

        match curr.run_tape() {
            Ok(State::Halted) => {
                println!("ERROR!!! {:?}", curr.outputs);
                return 0;
            }
            Ok(State::WaitingForInput) => {
                let rv = curr.outputs.pop_front().unwrap_or(0);
                if rv == 0 {
                    continue;
                }
                if rv == 2 {
                    println!("ERROR!!! {:?}", curr.outputs);
                    return 0;
                }
                for i in 1..=4 {
                    let mut next = curr.clone();
                    next.inputs.push_back(i);
                    let mut path2 = path.clone();
                    path2.push(i);
                    paths.push_back((path2, next_cell, next));
                }
            }
            Err(code) => {
                println!("ERROR!!! {}", code);
                return 0;
            }
            Ok(state) => {
                println!("ERROR, machine not halted! {:?}", state);
                return 0;
            }
        }
    }

    max
}

//-----------------------------------------------------
// Questions.

q_impl!("15");

#[test]
fn a() {
    // assert_eq!(process_data_a(""), 0);
}

#[test]
fn b() {
    // assert_eq!(process_data_b(""), 0);
}
