//-----------------------------------------------------
// Setup.

use crate::intcode::{Intcode, State};

static INPUT: &str = include_str!("data/q21.data");

fn process_data_a(data: &str) -> i128 {
    let ints: Vec<i128> = data
        .split(',')
        .map(|i| i.parse::<i128>().unwrap())
        .collect();
    let program = "NOT A T
NOT B J
OR J T
NOT C J
OR J T
NOT D J
NOT J J
AND T J
WALK
";
    let mut machine = Intcode::new(ints, program.chars().map(|x| x as i128).collect());
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
    outputs[0]
}

fn process_data_b(data: &str) -> i128 {
    let ints: Vec<i128> = data
        .split(',')
        .map(|i| i.parse::<i128>().unwrap())
        .collect();
    let program = "NOT A T
NOT B J
OR J T
NOT C J
OR J T
NOT D J
NOT J J
AND J T
NOT H J
NOT J J
OR E J
AND T J
RUN
";
    let mut machine = Intcode::new(ints, program.chars().map(|x| x as i128).collect());
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
    outputs[0]
}

//-----------------------------------------------------
// Questions.

q_impl!("21");

#[test]
fn a() {
    // assert_eq!(process_data_a(""), 0);
}

#[test]
fn b() {
    // assert_eq!(process_data_b(""), 0);
}
