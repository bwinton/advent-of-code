//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q02.data");

fn run_tape(ints: &mut Vec<usize>) -> usize {
    let mut position = 0;
    loop {
        let opcode = &ints[position];
        match opcode {
            1 => {
                if let [a, b, dest] = ints[position + 1..position + 4] {
                    ints[dest] = ints[a] + ints[b];
                }
            }
            2 => {
                if let [a, b, dest] = ints[position + 1..position + 4] {
                    ints[dest] = ints[a] * ints[b];
                }
            }
            99 => {
                break;
            }
            _ => {
                println!("ERROR!!!");
                break;
            }
        }
        position += 4
    }

    ints[0]
}

fn process_data_a(data: &str) -> usize {
    let mut ints: Vec<usize> = data
        .split(',')
        .map(|i| i.parse::<usize>().unwrap())
        .collect();
    ints[1] = 12;
    ints[2] = 2;
    run_tape(&mut ints)
}

fn process_data_b(data: &str) -> usize {
    let base: Vec<usize> = data
        .split(',')
        .map(|i| i.parse::<usize>().unwrap())
        .collect();
    for verb in 0..100 {
        for noun in 0..100 {
            let mut ints = base.clone();
            ints[1] = noun;
            ints[2] = verb;
            if run_tape(&mut ints) == 19_690_720 {
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
    assert_eq!(
        run_tape(&mut vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
        3500
    );
    assert_eq!(run_tape(&mut vec![1, 0, 0, 0, 99]), 2);
    assert_eq!(run_tape(&mut vec![2, 3, 0, 3, 99]), 2);
    assert_eq!(run_tape(&mut vec![1, 1, 1, 4, 99, 5, 6, 0, 99]), 30);
}

#[test]
fn b() {
    // assert_eq!(process_data_b(""), 0);
}
