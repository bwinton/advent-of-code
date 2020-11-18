//-----------------------------------------------------
// Setup.

use crate::intcode::{Intcode, State};
use std::collections::HashSet;

static INPUT: &str = include_str!("data/q23.data");

fn process_data_a(data: &str) -> i128 {
    let ints: Vec<i128> = data
        .split(',')
        .map(|i| i.parse::<i128>().unwrap())
        .collect();
    let mut machines = vec![];
    for i in 0..50 {
        let machine = Intcode::new(ints.clone(), vec![i, -1]);
        machines.push(machine)
    }
    loop {
        let mut values = vec![];
        for machine in &mut machines {
            match machine.run_tape() {
                Ok(State::Halted) | Ok(State::Ready) | Ok(State::WaitingForInput) => {}
                Err(code) => {
                    println!("ERROR!!! {}", code);
                }
            }
            let outputs = &mut machine.outputs;
            while outputs.len() >= 3 {
                let address = outputs.pop_back().unwrap();
                let x = outputs.pop_back().unwrap();
                let y = outputs.pop_back().unwrap();
                if address == 255 {
                    return y;
                }
                values.push((address as usize, x, y));
            }
        }
        if !values.is_empty() {
            for value in values {
                machines[value.0].inputs.push_back(value.1);
                machines[value.0].inputs.push_back(value.2);
            }
        }
        for machine in &mut machines {
            if machine.inputs.is_empty() {
                machine.inputs.push_back(-1);
            }
        }
    }
}

fn process_data_b(data: &str) -> i128 {
    let mut nat = None;
    let mut delivered = HashSet::new();

    let ints: Vec<i128> = data
        .split(',')
        .map(|i| i.parse::<i128>().unwrap())
        .collect();
    let mut machines = vec![];
    for i in 0..50 {
        let machine = Intcode::new(ints.clone(), vec![i, -1]);
        machines.push(machine)
    }
    let mut empty_count = 0;
    loop {
        let mut values = vec![];
        for machine in &mut machines {
            match machine.run_tape() {
                Ok(State::Halted) | Ok(State::Ready) | Ok(State::WaitingForInput) => {}
                Err(code) => {
                    println!("ERROR!!! {}", code);
                }
            }
            let outputs = &mut machine.outputs;
            while outputs.len() >= 3 {
                let address = outputs.pop_back().unwrap();
                let x = outputs.pop_back().unwrap();
                let y = outputs.pop_back().unwrap();
                if address == 255 {
                    nat = Some((x, y));
                } else {
                    values.push((address as usize, x, y));
                }
            }
        }
        if !values.is_empty() {
            for value in values {
                machines[value.0].inputs.push_back(value.1);
                machines[value.0].inputs.push_back(value.2);
            }
            empty_count = 0;
        } else if machines
            .iter()
            .all(|m| m.inputs.is_empty() || m.inputs.iter().all(|x| *x == -1))
        {
            // There are no outputs, and no inputs…
            empty_count += 1;
            // Derived by experimentation. Max empty_count was 136.
            if empty_count >= 140 {
                if let Some(nat) = nat {
                    // println!("Idle detected . Sending {:?} to 0.", nat);
                    machines[0].inputs.push_back(nat.0);
                    machines[0].inputs.push_back(nat.1);
                    if !delivered.insert(nat.1) {
                        return nat.1;
                    }
                }
                empty_count = 0;
            }
        }
        for machine in &mut machines {
            if machine.inputs.is_empty() {
                machine.inputs.push_back(-1);
            }
        }
    }
    // 17541 is too high.
}

//-----------------------------------------------------
// Questions.

q_impl!("23");

#[test]
fn a() {
    // assert_eq!(process_data_a(""), 0);
}

#[test]
fn b() {
    // assert_eq!(process_data_b(""), 0);
}
