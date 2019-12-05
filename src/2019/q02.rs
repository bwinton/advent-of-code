//-----------------------------------------------------
// Setup.

use crate::intcode::run_tape;

static INPUT: &str = include_str!("data/q02.data");

fn process_data_a(data: &str) -> i32 {
    let mut ints: Vec<i32> = data.split(',').map(|i| i.parse::<i32>().unwrap()).collect();
    ints[1] = 12;
    ints[2] = 2;
    match run_tape(&mut ints, vec![]) {
        Ok(_) => ints[0],
        Err(code) => {
            println!("ERROR!!! in code {}", code);
            0
        }
    }
}

fn process_data_b(data: &str) -> i32 {
    let base: Vec<i32> = data.split(',').map(|i| i.parse::<i32>().unwrap()).collect();
    for verb in 0..100 {
        for noun in 0..100 {
            let mut ints = base.clone();
            ints[1] = noun;
            ints[2] = verb;
            if run_tape(&mut ints, vec![]).is_ok() && ints[0] == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}

//-----------------------------------------------------
// Questions.

q_impl!("2");

#[test]
fn a() {
    // assert_eq!(process_data_b(""), 0);
}

#[test]
fn b() {
    // assert_eq!(process_data_b(""), 0);
}
