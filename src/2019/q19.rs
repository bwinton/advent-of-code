//-----------------------------------------------------
// Setup.

use crate::intcode::{Intcode, State};

static INPUT: &str = include_str!("data/q19.data");

fn process_data_a(data: &str) -> i128 {
    let ints: Vec<i128> = data
        .split(',')
        .map(|i| i.parse::<i128>().unwrap())
        .collect();

    let mut affected = 0;

    for y in 0..50 {
        for x in 0..50 {
            let mut machine = Intcode::new(ints.clone(), vec![x, y]);
            match machine.run_tape() {
                Ok(State::Halted) => {}
                Err(code) => {
                    println!("ERROR!!! {}", code);
                }
                Ok(state) => {
                    println!("ERROR, machine not halted! {:?}", state);
                }
            }
            let output = machine.outputs.pop_front().unwrap();
            // print!("{}", output);
            affected += output
        }
        // println!();
    }
    affected
}

fn check_pos(x: i128, y: i128, ints: Vec<i128>) -> bool {
    let mut machine = Intcode::new(ints, vec![x, y]);
    match machine.run_tape() {
        Ok(State::Halted) => {}
        Err(code) => {
            println!("ERROR!!! {}", code);
        }
        Ok(state) => {
            println!("ERROR, machine not halted! {:?}", state);
        }
    }
    let output = machine.outputs.pop_front().unwrap();
    output == 1
}

fn process_data_b(data: &str) -> i128 {
    let ints: Vec<i128> = data
        .split(',')
        .map(|i| i.parse::<i128>().unwrap())
        .collect();

    // Use math to get small bounds!
    for y in 700..800 {
        for x in 900..1000 {
            if !check_pos(x, y + 99, ints.clone()) || !check_pos(x + 99, y, ints.clone()) {
                continue;
            }
            return x * 10_000 + y;
        }
    }
    0
    // Not 940076.
    // Not 10911000.
    // Not 4290393
}

//-----------------------------------------------------
// Questions.

q_impl!("19");

#[test]
fn a() {
    assert_eq!(process_data_a(""), 0);
}

#[test]
fn b() {
    assert_eq!(process_data_b(""), 0);
}
