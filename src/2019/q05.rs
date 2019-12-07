//-----------------------------------------------------
// Setup.

use std::collections::VecDeque;

use crate::intcode::run_tape;

static INPUT: &str = include_str!("data/q05.data");

fn process_data_a(data: &str) -> i64 {
    let mut ints: Vec<i64> = data.split(',').map(|i| i.parse::<i64>().unwrap()).collect();
    let inputs = VecDeque::from(vec![1]);
    match run_tape(&mut ints, inputs) {
        Ok(outputs) => {
            let final_value = *outputs.last().unwrap();
            if outputs.iter().sum::<i64>() == final_value {
                return final_value;
            }
        }
        Err(code) => {
            println!("ERROR!!! in code {}", code[0]);
        }
    }
    -1
}

fn process_data_b(data: &str) -> i64 {
    let mut ints: Vec<i64> = data.split(',').map(|i| i.parse::<i64>().unwrap()).collect();
    let inputs = VecDeque::from(vec![5]);
    match run_tape(&mut ints, inputs) {
        Ok(outputs) => {
            if outputs.len() == 1 {
                return outputs[0];
            }
        }
        Err(code) => {
            println!("ERROR!!! in code {}", code[0]);
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
