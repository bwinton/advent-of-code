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
    // Find left and right at y=1000 to get slopes.
    let mut min_x = 0;
    let mut max_x = 0;
    let mut x = 0;
    const Y: i128 = 1000;
    while min_x == 0 {
        x += 1;
        if check_pos(x, Y, ints.clone()) {
            min_x = x;
        }
    }
    while max_x == 0 {
        x += 1;
        if !check_pos(x, Y, ints.clone()) {
            max_x = x;
        }
    }
    // Get the slopes.
    let low_slope = min_x as f64 / Y as f64;
    let high_slope = max_x as f64 / Y as f64;

    // Solve low_slope*(y+100) == high_slope*y - 100
    //  => 0 = high_slope*y - 100 - (low_slope*y + 100*low_slope)
    //  => high_slope*y - 100 - (low_slope*y + 100*low_slope) = 0
    //  => high_slope*y - 100 - low_slope*y - 100*low_slope) = 0
    //  => high_slope*y - low_slope*y - 100*low_slope - 100 = 0
    //  => (high_slope - low_slope)*y - (100*low_slope + 100) = 0
    //  => (high_slope - low_slope)*y - 101*low_slope = 0
    //  => (high_slope - low_slope)*y = 101*low_slope
    //  => y = 101*low_slope / (high_slope - low_slope)
    let y = (100.0 * (low_slope + 1.0)) / (high_slope - low_slope);

    // Solve x == high_slope * y - 100
    let x = high_slope * y - 100.0;
    let (x, y) = (x as i128, y as i128);

    for y in y - 10..y + 10 {
        for x in x - 10..x + 10 {
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
