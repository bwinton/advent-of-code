//-----------------------------------------------------
// Setup.

use crate::intcode::{Intcode, IntcodeError, State};

use itertools::Itertools;

static INPUT: &str = include_str!("data/q07.data");

fn run_amps(ints: &[i128], permutation: Vec<i128>) -> Result<i128, IntcodeError> {
    let mut first = Intcode::new(ints.to_owned(), vec![permutation[0], 0]);
    first.run_tape()?;
    let mut second = Intcode::new(
        ints.to_owned(),
        vec![permutation[1], first.outputs.pop_back().unwrap()],
    );
    second.run_tape()?;
    let mut third = Intcode::new(
        ints.to_owned(),
        vec![permutation[2], second.outputs.pop_back().unwrap()],
    );
    third.run_tape()?;
    let mut fourth = Intcode::new(
        ints.to_owned(),
        vec![permutation[3], third.outputs.pop_back().unwrap()],
    );
    fourth.run_tape()?;
    let mut fifth = Intcode::new(
        ints.to_owned(),
        vec![permutation[4], fourth.outputs.pop_back().unwrap()],
    );
    fifth.run_tape()?;
    fifth.outputs.pop_back().ok_or(IntcodeError::MissingValue)
}

fn run_multi_amps(ints: &[i128], permutation: Vec<i128>) -> Result<i128, IntcodeError> {
    let mut first = Intcode::new(ints.to_owned(), vec![permutation[0]]);
    let mut state = first.run_tape()?;
    if state != State::WaitingForInput {
        return Err(IntcodeError::MachineNotWaiting);
    }
    let mut second = Intcode::new(ints.to_owned(), vec![permutation[1]]);
    state = second.run_tape()?;
    if state != State::WaitingForInput {
        return Err(IntcodeError::MachineNotWaiting);
    }
    let mut third = Intcode::new(ints.to_owned(), vec![permutation[2]]);
    state = third.run_tape()?;
    if state != State::WaitingForInput {
        return Err(IntcodeError::MachineNotWaiting);
    }
    let mut fourth = Intcode::new(ints.to_owned(), vec![permutation[3]]);
    state = fourth.run_tape()?;
    if state != State::WaitingForInput {
        return Err(IntcodeError::MachineNotWaiting);
    }
    let mut fifth = Intcode::new(ints.to_owned(), vec![permutation[4]]);
    state = fifth.run_tape()?;
    if state != State::WaitingForInput {
        return Err(IntcodeError::MachineNotWaiting);
    }

    let mut max = 0;
    let mut input = 0;
    loop {
        first.inputs.push_back(input);
        first.run_tape()?;
        input = first.outputs.pop_back().ok_or(IntcodeError::MissingValue)?;

        second.inputs.push_back(input);
        second.run_tape()?;
        input = second.outputs.pop_back().ok_or(IntcodeError::MissingValue)?;

        third.inputs.push_back(input);
        third.run_tape()?;
        input = third.outputs.pop_back().ok_or(IntcodeError::MissingValue)?;

        fourth.inputs.push_back(input);
        fourth.run_tape()?;
        input = fourth.outputs.pop_back().ok_or(IntcodeError::MissingValue)?;

        fifth.inputs.push_back(input);
        state = fifth.run_tape()?;
        input = fifth.outputs.pop_back().ok_or(IntcodeError::MissingValue)?;

        if input > max {
            max = input;
        }
        if state == State::Halted {
            break;
        }
    }
    Ok(max)
}

fn process_data_a(data: &str) -> i128 {
    let ints: Vec<i128> = data
        .split(',')
        .map(|i| i.parse::<i128>().unwrap())
        .collect();

    let phases = 0..5;

    let mut max = 0;
    for permutation in phases.permutations(5) {
        match run_amps(&ints, permutation) {
            Ok(result) => {
                if result > max {
                    max = result;
                }
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }

    max
}

fn process_data_b(data: &str) -> i128 {
    let ints: Vec<i128> = data
        .split(',')
        .map(|i| i.parse::<i128>().unwrap())
        .collect();

    let phases = 5..10;

    let mut max = 0;
    for permutation in phases.permutations(5) {
        match run_multi_amps(&ints, permutation) {
            Ok(result) => {
                if result > max {
                    max = result;
                }
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }

    max
}

//-----------------------------------------------------
// Questions.

q_impl!("7");

#[test]
fn a() {
    let program: Vec<i128> = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
        .split(',')
        .map(|i| i.parse::<i128>().unwrap())
        .collect();
    assert_eq!(run_amps(&program, vec![4, 3, 2, 1, 0]), Ok(43_210));
    assert_eq!(
        process_data_a("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
        43_210
    );
    assert_eq!(
        process_data_a("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"),
        54_321
    );
    assert_eq!(process_data_a("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"), 65_210);
}

#[test]
fn b() {
    let program: Vec<i128> =
        "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
            .split(',')
            .map(|i| i.parse::<i128>().unwrap())
            .collect();
    assert_eq!(
        run_multi_amps(&program, vec![9, 8, 7, 6, 5]),
        Ok(139_629_729)
    );

    assert_eq!(
        process_data_b(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
        ),
        139_629_729
    );
    assert_eq!(process_data_b("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"), 18_216);
}
