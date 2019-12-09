//-----------------------------------------------------
// Setup.

use crate::intcode::{Intcode, State};

static INPUT: &str = include_str!("data/q09.data");

fn process_data_a(data: &str) -> i128 {
    let ints: Vec<i128> = data
        .split(',')
        .map(|i| i.parse::<i128>().unwrap())
        .collect();
    let mut machine = Intcode::new(ints, vec![1]);
    match machine.run_tape() {
        Ok(State::Halted) => {
            return *machine.outputs.last().unwrap();
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
    let mut machine = Intcode::new(ints, vec![2]);
    match machine.run_tape() {
        Ok(State::Halted) => {
            return *machine.outputs.last().unwrap();
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

q_impl!("9");

#[test]
fn a() {
    // assert_eq!(process_data_a("1102,34463338,34463338,63,1007,63,34463338,63,1005,63,53,1101,3,0,1000,109,988,209,12,9,1000,209,6,209,3,203,0,1008,1000,1,63,99"), 0);
    // assert_eq!(process_data_a("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"), 0);
    assert_eq!(
        process_data_a("1102,34915192,34915192,7,4,7,99,0"),
        1_219_070_632_396_864
    );
    assert_eq!(
        process_data_a("104,1125899906842624,99"),
        1_125_899_906_842_624
    );
}

#[test]
fn b() {
    // assert_eq!(process_data_b(""), 0);
}
