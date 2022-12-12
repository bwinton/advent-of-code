use std::collections::HashMap;

//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q15.data");

fn run(data: &str, iterations: usize) -> usize {
    let numbers: Vec<usize> = data.split(',').map(|x| x.parse().unwrap()).collect();
    let mut timestamps = HashMap::new();
    let mut gap = 0;
    for i in 0..iterations - 1 {
        // print!("{}: ", i);
        if i < numbers.len() {
            timestamps.insert(numbers[i], i);
            // println!("{}", numbers[i]);
            // println!("  {:?}", timestamps);
            continue;
        }

        // print!("{} => ", gap);
        let next = if let Some(prev) = timestamps.get(&gap) {
            i - prev
        } else {
            0
        };
        // println!("{}", next);
        timestamps.insert(gap, i);
        // println!("  {:?}", timestamps);
        gap = next;
    }
    gap
}

fn process_data_a(data: &str) -> usize {
    run(data, 2020)
}

fn process_data_b(data: &str) -> usize {
    run(data, 30_000_000)
}

//-----------------------------------------------------
// Questions.

q_impl!("15");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_a("0,3,6"), 436);
}

#[test]
#[ignore]
fn b() {
    // I'm brute forcing it, so these take way too long!
    assert_eq!(process_data_b("0,3,6"), 175_594);
    assert_eq!(process_data_b("1,3,2"), 2_578);
    assert_eq!(process_data_b("2,1,3"), 3_544_142);
    assert_eq!(process_data_b("1,2,3"), 261_214);
    assert_eq!(process_data_b("2,3,1"), 6_895_259);
    assert_eq!(process_data_b("3,2,1"), 18);
    assert_eq!(process_data_b("3,1,2"), 362);
}
