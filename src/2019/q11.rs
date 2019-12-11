//-----------------------------------------------------
// Setup.

use std::collections::HashMap;
use std::fmt::Write;

use crate::intcode::{Intcode, State};

static INPUT: &str = include_str!("data/q11.data");

#[derive(Debug)]
enum Heading {
    North,
    East,
    South,
    West,
}

impl Heading {
    fn turn(&mut self, dir: i128) {
        match *self {
            Heading::North => {
                if dir == 1 {
                    *self = Heading::East
                } else {
                    *self = Heading::West
                }
            }
            Heading::East => {
                if dir == 1 {
                    *self = Heading::South
                } else {
                    *self = Heading::North
                }
            }
            Heading::South => {
                if dir == 1 {
                    *self = Heading::West
                } else {
                    *self = Heading::East
                }
            }
            Heading::West => {
                if dir == 1 {
                    *self = Heading::North
                } else {
                    *self = Heading::South
                }
            }
        }
    }
}

fn run_painter(data: &str, start: i128) -> HashMap<(i32, i32), i128> {
    let mut panels: HashMap<(i32, i32), i128> = HashMap::new();
    let mut current = (0, 0);
    panels.insert(current, start);
    let mut direction = Heading::North;
    let ints: Vec<i128> = data
        .split(',')
        .map(|i| i.parse::<i128>().unwrap())
        .collect();
    let mut machine = Intcode::new(ints, vec![*panels.get(&current).unwrap_or(&0)]);
    loop {
        match machine.run_tape() {
            Ok(State::Halted) => {
                break;
            }
            Err(code) => {
                panic!("ERROR!!! {}", code);
            }
            Ok(State::Ready) => {
                panic!("ERROR!!! Unexpected Ready!");
            }
            Ok(State::WaitingForInput) => {
                panels.insert(current, machine.outputs.pop_back().unwrap());
                let out = machine.outputs.pop_back().unwrap();
                direction.turn(out);
                match direction {
                    Heading::North => current.0 += 1,
                    Heading::East => current.1 += 1,
                    Heading::South => current.0 -= 1,
                    Heading::West => current.1 -= 1,
                }
                machine
                    .inputs
                    .push_back(*panels.get(&current).unwrap_or(&0));
            }
        }
    }
    panels
}

fn process_data_a(data: &str) -> usize {
    run_painter(data, 0).len()
}

fn process_data_b(data: &str) -> String {
    let panels = run_painter(data, 1);
    let keys: Vec<_> = panels.keys().collect();
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    for key in keys {
        // println!("key = {:?}", key);
        if key.0 < min_x {
            min_x = key.0;
        }
        if key.0 > max_x {
            max_x = key.0;
        }
        if key.1 < min_y {
            min_y = key.1;
        }
        if key.1 > max_y {
            max_y = key.1;
        }
    }
    let mut s = String::new();
    if let Err(e) = writeln!(s) {
        return format!("Error {}", e);
    }

    for x in max_x..=-min_x {
        for y in min_y..=max_y {
            if let Err(e) = write!(
                s,
                "{}",
                if panels.get(&(-x, y)) == Some(&1) {
                    "█"
                } else {
                    " "
                }
            ) {
                return format!("Error {}", e);
            }
        }
        if let Err(e) = writeln!(s) {
            return format!("Error {}", e);
        }
    }
    s
}

//-----------------------------------------------------
// Questions.

q_impl!("11");

#[test]
fn a() {
    // assert_eq!(process_data_a(""), 0);
}

#[test]
fn b() {
    // assert_eq!(process_data_b(""), 0);
}
