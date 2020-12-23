//-----------------------------------------------------
// Setup.

use itertools::Itertools;
static INPUT: &str = include_str!("data/q23.data");

fn run_a(cups: Vec<usize>, steps: usize) -> Vec<usize> {
    let mut cups = cups;

    for _ in 0..steps {
        let pickup: Vec<usize> = cups.splice(1..4, vec![]).collect();
        let mut next = cups[0] - 1;
        while cups.iter().position(|&x| x == next).is_none() {
            if next <= 1 {
                next = 9
            } else {
                next -= 1;
            }
        }
        let dest = cups.iter().position(|&x| x == next).unwrap() + 1;
        cups.splice(dest..dest, pickup);
        cups.rotate_left(1);

        // let next = cups[dest];
    }

    let dest = cups.iter().position(|&x| x == 1).unwrap() + 1;
    cups.rotate_left(dest);
    cups[..cups.len()-1].to_vec()
}


fn run_b(cups: &Box<[usize; 1_000_001]>, start: usize, steps: usize) -> usize {
    // Cups is an array of usize pointers to the next value;
    let mut cups = cups;

    let mut curr = start as usize;
    for i in 0..steps {
        let target = curr - 1;
        let mut pickup = cups[curr];
        let mut picked_up = vec![];
        let mut next = pickup;
        for _ in 0..3 {
            picked_up.push(cups[next]);
            next = cups[next];
        }
        println!("picked_up: {:?}", picked_up);
        println!("target: {:?}", target);
        println!("next: {:?}", next);

        while picked_up.contains(&target) {
            if target <= 1 {
                target = 1_000_000
            } else {
                target -= 1;
            }
        }

        cups[curr] = target;
        cups[picked_up[picked_up.len() - 1]] = cups[target];
        cups[target] = pickup;

    }

    let dest = cups.iter().position(|&x| x == 1).unwrap() + 1;
    cups.iter().cycle().skip(dest).take(2).map(|&x| x as u64).product()
}

fn process_data_a(data: &str) -> String {
    let mut cups= data.chars().map(|c| String::from(c).parse().unwrap()).collect();
    let cups = run_a(cups, 100);
    cups.iter().map(|c| c.to_string()).join("")
}

fn process_data_b(data: &str) -> u64 {
    let mut cups: Box<[usize; 1_000_001]> = Box::new([0;1_000_001]);
    for (i, value) in cups.iter_mut().enumerate() {
        *value = (i as usize) + 1;
    }
    let values: Vec<usize> = data.chars().map(|c| String::from(c).parse().unwrap()).collect();
    let start = values[0];
    for (i, value) in values.into_iter().enumerate() {
        if i == 0 {
            continue;
        }
        cups[i-1] = value;
    }
    cups[999_999] = 0;

    println!("{:?}", &cups[..12]);

    run_b(&cups, start, 10_000_000)
}

//-----------------------------------------------------
// Questions.

q_impl!("23");

#[test]
fn a() {
    assert_eq!(run_a(vec![3,8,9,1,2,5,4,6,7], 10), vec![9,2,6,5,8,3,7,4]);
    // assert_eq!(process_data_a("32415"), "67384529".to_string());
}

#[test]
fn b() {
    assert_eq!(process_data_b("389125467"), 149245887792);
}
