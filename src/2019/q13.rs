//-----------------------------------------------------
// Setup.

use crate::intcode::{Intcode, State};

static INPUT: &str = include_str!("data/q13.data");

fn process_data_a(data: &str) -> usize {
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
    let mut block_count = 0;
    while !outputs.is_empty() {
        let _ = outputs.pop_back().unwrap();
        let _ = outputs.pop_back().unwrap();
        let block = outputs.pop_back().unwrap();
        if block == 2 {
            block_count += 1;
        }
    }
    block_count

    // 53 is wrong.
    // 740 is wrong.
}

fn direction(a: i128, b: i128) -> i128 {
    match b - a {
        x if x < 0 => -1,
        x if x > 0 => 1,
        _ => 0,
    }
}

fn process_data_b(data: &str) -> i128 {
    let mut ints: Vec<i128> = data
        .split(',')
        .map(|i| i.parse::<i128>().unwrap())
        .collect();
    ints[0] = 2;
    let mut machine = Intcode::new(ints, vec![]);
    let mut ball_x: i128 = 0;
    let mut paddle_x: i128 = 0;
    let mut score: i128 = 0;

    loop {
        match machine.run_tape() {
            Ok(State::WaitingForInput) => {
                let outputs = &mut machine.outputs;
                while !outputs.is_empty() {
                    let x = outputs.pop_back().unwrap();
                    let y = outputs.pop_back().unwrap();
                    let block = outputs.pop_back().unwrap();
                    // println!("{:?}", (x, y, block));
                    match block {
                        4 => {
                            println!("Ball is at {}", x);
                            ball_x = x;
                        }
                        3 => {
                            println!("Paddle is at {}", x);
                            paddle_x = x;
                        }
                        _ => {}
                    }
                    if x == -1 && y == 0 {
                        println!("Score is {}", block);
                        score = block;
                    }
                }
                println!("Moving {}", direction(paddle_x, ball_x));
                machine.inputs.push_back(direction(paddle_x, ball_x));
            }
            Ok(State::Halted) => {
                let outputs = &mut machine.outputs;
                while !outputs.is_empty() {
                    let x = outputs.pop_back().unwrap();
                    let y = outputs.pop_back().unwrap();
                    let block = outputs.pop_back().unwrap();
                    if x == -1 && y == 0 {
                        score = block;
                    }
                }
                break;
            }
            Err(code) => {
                println!("ERROR!!! {}", code);
            }
            Ok(state) => {
                println!("ERROR, machine not halted! {:?}", state);
            }
        }
    }

    score
}

//-----------------------------------------------------
// Questions.

q_impl!("13");

#[test]
fn a() {
    assert_eq!(process_data_a(""), 0);
}

#[test]
fn b() {
    assert_eq!(process_data_b(""), 0);
}
