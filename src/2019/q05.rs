//-----------------------------------------------------
// Setup.

use crate::intcode::run_tape;

static INPUT: &str = include_str!("data/q05.data");

fn process_data_a(data: &str) -> i32 {
    let mut ints: Vec<i32> = data.split(',').map(|i| i.parse::<i32>().unwrap()).collect();
    let inputs = vec![1];
    match run_tape(&mut ints, inputs) {
        Ok(outputs) => {
            let final_value = *outputs.last().unwrap();
            if outputs.iter().sum::<i32>() == final_value {
                return final_value;
            }
        }
        Err(code) => {
            println!("ERROR!!! in code {}", code);
        }
    }
    -1
}

fn process_data_b(data: &str) -> i32 {
    let mut ints: Vec<i32> = data.split(',').map(|i| i.parse::<i32>().unwrap()).collect();
    let inputs = vec![5];
    match run_tape(&mut ints, inputs) {
        Ok(outputs) => {
            if outputs.len() == 1 {
                return outputs[0];
            }
        }
        Err(code) => {
            println!("ERROR!!! in code {}", code);
        }
    }
    -1
}

//-----------------------------------------------------
// Questions.

q_impl!("5");

#[test]
fn a() {
    assert_eq!(process_data_a("1002,4,3,0,99"), 297);
    assert_eq!(process_data_a("1101,100,-1,0,99"), 99);
}

#[test]
fn b() {
    assert_eq!(process_data_b(""), 0);
}
