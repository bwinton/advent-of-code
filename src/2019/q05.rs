//-----------------------------------------------------
// Setup.

use crate::intcode::{Intcode, State};

static INPUT: &str = include_str!("data/q05.data");

fn process_data_a(data: &str) -> i64 {
    let ints: Vec<i64> = data.split(',').map(|i| i.parse::<i64>().unwrap()).collect();
    let mut machine = Intcode::new(ints, vec![1]);
    match machine.run_tape() {
        Ok(State::Halted) => {
            let final_value = *machine.outputs.last().unwrap();
            if machine.outputs.iter().sum::<i64>() == final_value {
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

fn process_data_b(data: &str) -> i64 {
    let ints: Vec<i64> = data.split(',').map(|i| i.parse::<i64>().unwrap()).collect();
    let mut machine = Intcode::new(ints, vec![5]);
    match machine.run_tape() {
        Ok(State::Halted) => {
            if machine.outputs.len() == 1 {
                return machine.outputs[0];
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
