//-----------------------------------------------------
// Setup.

// use std::io;

use crate::intcode::{Intcode, State};

use itertools::Itertools;

static INPUT: &str = include_str!("data/q25.data");

fn process_data_a(data: &str) -> i128 {
    let ints: Vec<i128> = data
        .split(',')
        .map(|i| i.parse::<i128>().unwrap())
        .collect();
    let program = "south
take fixed point
north
north
take spool of cat 6
north
take monolith
west
take planetoid
east
north
take hypercube
south
south
east
north
take candy cane
south
east
take easter egg
east
south
take ornament
west
south
";

    let mut machine = Intcode::new(ints, program.chars().map(|x| x as i128).collect());
    let outputs = &mut machine.outputs;
    while let Some(output) = outputs.pop_back() {
        if output < 255 {
            print!("{}", output as u8 as char);
        } else {
            print!("{}", output);
        }
    }
    let items = [
        "fixed point",
        "spool of cat 6",
        "monolith",
        "planetoid",
        "hypercube",
        "candy cane",
        "easter egg",
        "ornament",
    ];
    for i in 1..items.len() {
        for drops in items.iter().combinations(i) {
            let mut curr = machine.clone();
            println!("Testing {:?}", drops);
            for drop in drops {
                curr.inputs
                    .extend(format!("drop {}\nwest\n", drop).chars().map(|x| x as i128));
                match curr.run_tape() {
                    Ok(State::Halted) => {
                        // break;
                    }
                    Ok(State::WaitingForInput) => {
                        println!("waiting for input…");
                    }
                    Err(code) => {
                        println!("ERROR!!! {}", code);
                        // break;
                    }
                    Ok(state) => {
                        println!("ERROR, machine not halted! {:?}", state);
                        // break;
                    }
                }
                let outputs = &mut curr.outputs;
                while let Some(output) = outputs.pop_back() {
                    if output < 255 {
                        print!("{}", output as u8 as char);
                    } else {
                        print!("{}", output);
                    }
                }
            }
        }
        println!();
    }
    // loop {
    //     match machine.run_tape() {
    //         Ok(State::Halted) => {
    //             break;
    //         }
    //         Ok(State::WaitingForInput) => {
    //             println!("waiting for input…");
    //         }
    //         Err(code) => {
    //             println!("ERROR!!! {}", code);
    //             break;
    //         }
    //         Ok(state) => {
    //             println!("ERROR, machine not halted! {:?}", state);
    //             break;
    //         }
    //     }
    //     let outputs = &mut machine.outputs;
    //     while let Some(output) = outputs.pop_back() {
    //         if output < 255 {
    //             print!("{}", output as u8 as char);
    //         } else {
    //             print!("{}", output);
    //         }
    //     }
    //     let mut input = String::new();
    //     if io::stdin().read_line(&mut input).is_ok() {
    //         // println!("{:?}", &input);
    //         // input.push('\n');
    //         machine.inputs.extend(input.chars().map(|x| x as i128));
    //         for output in machine.inputs.iter() {
    //             // if *output < 255 {
    //                 // print!("{}", *output as u8 as char);
    //             // } else {
    //                 print!("{}, ", output);
    //             // }
    //         }

    //     }
    // }
    0
}

fn process_data_b(_data: &str) -> i32 {
    0
}

//-----------------------------------------------------
// Questions.

q_impl!("25");

#[test]
fn a() {
    assert_eq!(process_data_a(""), 0);
}

#[test]
fn b() {
    assert_eq!(process_data_b(""), 0);
}
