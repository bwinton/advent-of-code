//-----------------------------------------------------
// Setup.

use std::collections::HashMap;

use crate::intcode::{Intcode, State};

static INPUT: &str = include_str!("data/q17.data");

fn process_data_a(data: &str) -> i32 {
    let ints: Vec<i128> = data
        .split(',')
        .map(|i| i.parse::<i128>().unwrap())
        .collect();
    let mut machine = Intcode::new(ints, vec![]);
    match machine.run_tape() {
        Ok(State::Halted) => {}
        Err(code) => {
            println!("ERROR!!! {}", code);
        }
        Ok(state) => {
            println!("ERROR, machine not halted! {:?}", state);
        }
    }
    let outputs = &mut machine.outputs;
    // println!();
    let mut board = HashMap::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    while !outputs.is_empty() {
        let cell = outputs.pop_back().unwrap() as u8 as char;
        match cell {
            '#' => {
                board.insert((x, y), true);
            }
            '\n' => {
                x = -1;
                y += 1;
            }
            _ => {}
        };
        x += 1;
        // print!("{}", cell);
    }
    let mut inter_sum = 0;
    for &(x, y) in board.keys() {
        if board.contains_key(&(x - 1, y))
            && board.contains_key(&(x + 1, y))
            && board.contains_key(&(x, y - 1))
            && board.contains_key(&(x, y + 1))
        {
            // println!("Found intersection at {:?}, adding {}", (x,y), x * y);
            inter_sum += x * y;
        }
    }
    inter_sum

    // 6188 is too high.
    // 5650 is too low.
}

fn process_data_b(data: &str) -> i128 {
    let program = "A,B,A,C,B,C,B,C,A,C
R,12,L,6,R,12
L,8,L,6,L,10
R,12,L,10,L,6,R,10
n
";
    let mut ints: Vec<i128> = data
        .split(',')
        .map(|i| i.parse::<i128>().unwrap())
        .collect();
    ints[0] = 2;
    let inputs = program.chars().map(|c| c as i128).collect();
    let mut machine = Intcode::new(ints, inputs);
    match machine.run_tape() {
        Ok(State::Halted) => {}
        Err(code) => {
            println!("ERROR!!! {}", code);
        }
        Ok(state) => {
            println!("ERROR, machine not halted! {:?}", state);
        }
    }
    let outputs = &mut machine.outputs;
    outputs.pop_front().unwrap()
}

//-----------------------------------------------------
// Questions.

q_impl!("17");

#[test]
fn a() {
    // assert_eq!(process_data_a(""), 0);
}

#[test]
fn b() {
    // assert_eq!(process_data_b(""), 0);
}
