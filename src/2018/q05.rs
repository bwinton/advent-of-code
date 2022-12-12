//-----------------------------------------------------
// Setup.

use rayon::prelude::*;
use std::iter::Iterator;

static INPUT: &str = include_str!("data/q05.data");

fn remove_pairs(data: &str) -> String {
    let mut data: Vec<_> = data.chars().collect();
    let mut lower_data = data
        .iter()
        .map(char::to_ascii_lowercase)
        .collect::<Vec<_>>();
    let mut found = true;
    while found {
        // add a terminator
        data.push(' ');
        lower_data.push(' ');
        // println!("{:?}", data);
        found = false;
        let mut rv = vec![];
        let mut lower_rv = vec![];

        let mut skip = false;
        for i in 0..data.len() - 1 {
            // println!("{}, {}:{}, {}", i, data[i], data[i+1], lower_data[i]);
            if skip {
                skip = false;
                continue;
            }
            if data[i] == data[i + 1] || lower_data[i] != lower_data[i + 1] {
                rv.push(data[i]);
                lower_rv.push(lower_data[i]);
            } else {
                // Skip the next one.
                skip = true;
                found = true;
            }
        }
        data = rv;
        lower_data = lower_rv;
    }

    data.iter().collect()
}

fn process_data_a(data: &str) -> usize {
    // 9705 is too high…
    remove_pairs(data.trim()).len()
}

// @todo: Needs optimization!!!
fn process_data_b(data: &str) -> usize {
    let data = data.trim();
    let chars = vec![
        vec!['a', 'A'],
        vec!['b', 'B'],
        vec!['c', 'C'],
        vec!['d', 'D'],
        vec!['e', 'E'],
        vec!['f', 'F'],
        vec!['g', 'G'],
        vec!['h', 'H'],
        vec!['i', 'I'],
        vec!['j', 'J'],
        vec!['k', 'K'],
        vec!['l', 'L'],
        vec!['m', 'M'],
        vec!['n', 'N'],
        vec!['o', 'O'],
        vec!['p', 'P'],
        vec!['q', 'Q'],
        vec!['r', 'R'],
        vec!['s', 'S'],
        vec!['t', 'T'],
        vec!['u', 'U'],
        vec!['v', 'V'],
        vec!['w', 'W'],
        vec!['x', 'X'],
        vec!['y', 'Y'],
        vec!['z', 'Z'],
    ];
    chars
        .par_iter()
        .map(|remove| {
            let curr: String = data.chars().filter(|x| !remove.contains(x)).collect();
            if curr.len() == data.len() {
                // Didn't find any characters, so no need to continue.
                usize::max_value()
            } else {
                remove_pairs(&curr).len()
            }
        })
        .min()
        .unwrap()
    // min
    // 6943 is too high…
}

//-----------------------------------------------------
// Questions.

q_impl!("5");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(remove_pairs("aA"), "");
    assert_eq!(remove_pairs("abBA"), "");
    assert_eq!(remove_pairs("abAB"), "abAB");
    assert_eq!(remove_pairs("aabAAB"), "aabAAB");
    assert_eq!(remove_pairs("dabAcCaCBAcCcaDA"), "dabCBAcaDA");
    assert_eq!(process_data_a("aA"), 0);
    assert_eq!(process_data_a("abBA"), 0);
    assert_eq!(process_data_a("abAB"), 4);
    assert_eq!(process_data_a("aabAAB"), 6);
    assert_eq!(process_data_a("dabAcCaCBAcCcaDA"), 10);
    assert_eq!(process_data_a("IfreEEeRkKFD"), 2);
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b("dabAcCaCBAcCcaDA"), 4);
}
