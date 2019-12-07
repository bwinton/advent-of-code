//-----------------------------------------------------
// Setup.

use crate::intcode::{continue_tape, run_tape};
use std::collections::VecDeque;

use itertools::Itertools;

static INPUT: &str = include_str!("data/q07.data");

fn run_amp(mut ints: &mut Vec<i64>, phase: i64, input: i64) -> Result<Option<i64>, Vec<i64>> {
    let inputs = VecDeque::from(vec![phase, input]);
    let mut outputs = run_tape(&mut ints, inputs)?;
    Ok(outputs.pop())
}

fn run_amps(ints: &[i64], permutation: Vec<i64>) -> i64 {
    let first = run_amp(&mut ints.to_owned(), permutation[0], 0)
        .unwrap()
        .unwrap();
    let second = run_amp(&mut ints.to_owned(), permutation[1], first)
        .unwrap()
        .unwrap();
    let third = run_amp(&mut ints.to_owned(), permutation[2], second)
        .unwrap()
        .unwrap();
    let fourth = run_amp(&mut ints.to_owned(), permutation[3], third)
        .unwrap()
        .unwrap();
    run_amp(&mut ints.to_owned(), permutation[4], fourth)
        .unwrap()
        .unwrap()
}

fn run_multi_amps(ints: &[i64], permutation: Vec<i64>) -> i64 {
    let first_program = &mut ints.to_owned();
    let second_program = &mut ints.to_owned();
    let third_program = &mut ints.to_owned();
    let fourth_program = &mut ints.to_owned();
    let fifth_program = &mut ints.to_owned();

    let mut first_position = 0;
    let mut second_position = 0;
    let mut third_position = 0;
    let mut fourth_position = 0;
    let mut fifth_position = 0;

    let _ = continue_tape(
        &mut first_position,
        first_program,
        VecDeque::from(vec![permutation[0]]),
    );
    let _ = continue_tape(
        &mut second_position,
        second_program,
        VecDeque::from(vec![permutation[1]]),
    );
    let _ = continue_tape(
        &mut third_position,
        third_program,
        VecDeque::from(vec![permutation[2]]),
    );
    let _ = continue_tape(
        &mut fourth_position,
        fourth_program,
        VecDeque::from(vec![permutation[3]]),
    );
    let _ = continue_tape(
        &mut fifth_position,
        fifth_program,
        VecDeque::from(vec![permutation[4]]),
    );

    let mut max = 0;
    let mut input = 0;
    loop {
        let mut outputs = continue_tape(
            &mut first_position,
            first_program,
            VecDeque::from(vec![input]),
        );
        let first = match outputs {
            Ok(outputs) => outputs[0],
            Err(outputs) => outputs[0],
        };
        outputs = continue_tape(
            &mut second_position,
            second_program,
            VecDeque::from(vec![first]),
        );
        let second = match outputs {
            Ok(outputs) => outputs[0],
            Err(outputs) => outputs[0],
        };
        outputs = continue_tape(
            &mut third_position,
            third_program,
            VecDeque::from(vec![second]),
        );
        let third = match outputs {
            Ok(outputs) => outputs[0],
            Err(outputs) => outputs[0],
        };
        outputs = continue_tape(
            &mut fourth_position,
            fourth_program,
            VecDeque::from(vec![third]),
        );
        let fourth = match outputs {
            Ok(outputs) => outputs[0],
            Err(outputs) => outputs[0],
        };
        outputs = continue_tape(
            &mut fifth_position,
            fifth_program,
            VecDeque::from(vec![fourth]),
        );
        let fifth = match &outputs {
            Ok(outputs) => outputs[0],
            Err(outputs) => outputs[0],
        };
        input = fifth;
        if fifth > max {
            max = fifth;
        }
        if outputs.is_ok() {
            break;
        }
    }
    max
}

fn process_data_a(data: &str) -> i64 {
    let ints: Vec<i64> = data.split(',').map(|i| i.parse::<i64>().unwrap()).collect();

    let phases = 0..5;

    let mut max = 0;
    for permutation in phases.permutations(5) {
        let result = run_amps(&ints, permutation);
        if result > max {
            max = result;
        }
    }

    max
}

fn process_data_b(data: &str) -> i64 {
    let ints: Vec<i64> = data.split(',').map(|i| i.parse::<i64>().unwrap()).collect();

    let phases = 5..10;

    let mut max = 0;
    for permutation in phases.permutations(5) {
        let result = run_multi_amps(&ints, permutation);
        if result > max {
            max = result;
        }
    }

    max
}

//-----------------------------------------------------
// Questions.

q_impl!("7");

#[test]
fn a() {
    let program: Vec<i64> = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
        .split(',')
        .map(|i| i.parse::<i64>().unwrap())
        .collect();
    assert_eq!(run_amps(&program, vec![4, 3, 2, 1, 0]), 43_210);
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
    let program: Vec<i64> =
        "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
            .split(',')
            .map(|i| i.parse::<i64>().unwrap())
            .collect();
    assert_eq!(run_multi_amps(&program, vec![9, 8, 7, 6, 5]), 139_629_729);

    assert_eq!(
        process_data_b(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
        ),
        139_629_729
    );
    assert_eq!(process_data_b("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"), 18_216);
}
