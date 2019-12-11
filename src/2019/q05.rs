//-----------------------------------------------------
// Setup.

use crate::intcode::{Intcode, State};

static INPUT: &str = include_str!("data/q05.data");

fn process_data_a(data: &str) -> i128 {
    let ints: Vec<i128> = data
        .split(',')
        .map(|i| i.parse::<i128>().unwrap())
        .collect();
    let mut machine = Intcode::new(ints, vec![1]);
    match machine.run_tape() {
        Ok(State::Halted) => {
            let final_value = machine.outputs.pop_front().unwrap();
            if machine.outputs.iter().sum::<i128>() == 0 {
                return final_value;
            }
        }
        Err(code) => {
            println!("ERROR!!! {}", code);
        }
        Ok(state) => {
            println!("ERROR, machine not halted! {:?}", state);
        }
    }
    -1
}

fn process_data_b(data: &str) -> i128 {
    let ints: Vec<i128> = data
        .split(',')
        .map(|i| i.parse::<i128>().unwrap())
        .collect();
    let mut machine = Intcode::new(ints, vec![5]);
    match machine.run_tape() {
        Ok(State::Halted) => {
            if machine.outputs.len() == 1 {
                return machine.outputs.pop_back().unwrap();
            }
        }
        Err(code) => {
            println!("ERROR!!! {}", code);
        }
        Ok(state) => {
            println!("ERROR, machine not halted! {:?}", state);
        }
    }
    -1
}

//-----------------------------------------------------
// Questions.

q_impl!("5");

#[test]
fn a() {}

#[test]
fn b() {
    // assert_eq!(process_data_b(""), 0);
}
