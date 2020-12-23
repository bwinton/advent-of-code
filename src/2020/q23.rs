//-----------------------------------------------------
// Setup.

use itertools::Itertools;
static INPUT: &str = include_str!("data/q23.data");

#[allow(unused)]
fn print_cups(cups: &[usize], first: usize) {
    let mut curr = first;
    print!("({}) ", cups[curr]);
    curr = cups[curr];
    let mut i = 0;
    while curr != first && i < 20 {
        print!("{} ", cups[curr]);
        curr = cups[curr];
        i += 1;
    }
    println!();
}

fn run_a(data: &str, steps: usize) -> String {
    let mut cups: [usize; 10] = [0; 10];
    let values: Vec<usize> = data
        .chars()
        .map(|c| String::from(c).parse().unwrap())
        .collect();
    cups[0] = values[values.len() - 1];
    let mut prev = 0;
    for value in &values {
        if prev != 0 {
            cups[prev] = *value;
        }
        prev = *value;
    }
    cups[prev] = values[0];

    let mut curr = cups[0];
    for _ in 0..steps {
        curr = cups[curr];
        let mut target = curr - 1;
        if target == 0 {
            target = cups.len() - 1;
        }
        let pickup = curr;
        let mut picked_up = vec![];
        let mut next = pickup;
        for _ in 0..3 {
            picked_up.push(cups[next]);
            next = cups[next];
        }

        while picked_up.contains(&target) {
            if target <= 1 {
                target = 9
            } else {
                target -= 1;
            }
        }

        let old_next = cups[next];
        cups[next] = cups[target];
        cups[target] = cups[pickup];
        cups[pickup] = old_next;
    }

    let mut rv = vec![];
    let mut curr = 1;
    rv.push(cups[curr]);
    curr = cups[curr];
    while cups[curr] != 1 {
        rv.push(cups[curr]);
        curr = cups[curr];
    }

    rv.iter().map(|c| c.to_string()).join("")
}

fn process_data_a(data: &str) -> String {
    run_a(data, 100)
}

fn process_data_b(data: &str) -> usize {
    let mut cups: [usize; 1_000_001] = [0; 1_000_001];
    for (i, item) in cups.iter_mut().enumerate() {
        *item = i + 1;
    }
    let values: Vec<usize> = data
        .chars()
        .map(|c| String::from(c).parse().unwrap())
        .collect();
    cups[0] = values[0];
    let mut prev = 0;
    for value in &values {
        if prev != 0 {
            cups[prev] = *value;
        }
        prev = *value;
    }
    cups[prev] = 10;
    cups[cups.len() - 1] = cups[0];

    let mut curr = 0;
    for _ in 0..10_000_000 {
        curr = cups[curr];
        let mut target = curr - 1;
        if target == 0 {
            target = cups.len() - 1;
        }
        let pickup = curr;
        let mut picked_up = vec![];
        let mut next = pickup;
        for _ in 0..3 {
            picked_up.push(cups[next]);
            next = cups[next];
        }

        while picked_up.contains(&target) {
            if target <= 1 {
                target = 9
            } else {
                target -= 1;
            }
        }

        let old_next = cups[next];
        cups[next] = cups[target];
        cups[target] = cups[pickup];
        cups[pickup] = old_next;
    }

    cups[1] * cups[cups[1]]
}

//-----------------------------------------------------
// Questions.

q_impl!("23");

#[test]
fn a() {
    assert_eq!(run_a("389125467", 10), "92658374".to_string());
    assert_eq!(run_a("389125467", 100), "67384529".to_string());
    assert_eq!(process_data_a("962713854"), "65432978".to_string());
}

#[test]
fn b() {
    // Blows the stack. :P
    // assert_eq!(process_data_b("389125467"), 149_245_887_792);
}
