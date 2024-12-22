//-----------------------------------------------------
// Setup.

use std::collections::HashMap;

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

static INPUT: &str = include_str!("data/q22.data");

fn process_data_a(data: &str) -> usize {
    let mut monkeys: Vec<usize> = data.lines().map(|x| x.parse().unwrap()).collect();
    // println!("monkeys: {:?}", monkeys);

    for _i in 0..2000 {
        for monkey in monkeys.iter_mut() {
            // println!("{}", monkey);
            let result = *monkey * 64;
            *monkey ^= result;
            *monkey %= 16777216;
            let result = *monkey / 32;
            *monkey ^= result;
            *monkey %= 16777216;
            let result = *monkey * 2048;
            *monkey ^= result;
            *monkey %= 16777216;
        }
    }
    monkeys.iter().sum()
}

fn process_data_b(data: &str) -> usize {
    let mut monkeys: Vec<usize> = data.lines().map(|x| x.parse().unwrap()).collect();
    // println!("monkeys: {:?}", monkeys);

    let mut prices = vec![vec![]; monkeys.len()];
    for _i in 0..2000 {
        for (i, monkey) in monkeys.iter_mut().enumerate() {
            // println!("{}", monkey);
            let result = *monkey * 64;
            *monkey ^= result;
            *monkey %= 16777216;
            let result = *monkey / 32;
            *monkey ^= result;
            *monkey %= 16777216;
            let result = *monkey * 2048;
            *monkey ^= result;
            *monkey %= 16777216;
            prices[i].push(*monkey % 10);
        }
    }
    let changes = prices
        .par_iter()
        .map(|x| {
            let mut change = HashMap::new();
            for (a, b, c, d, e) in x.iter().tuple_windows() {
                let a = *a as i8;
                let b = *b as i8;
                let c = *c as i8;
                let d = *d as i8;
                let key = (b - a, c - b, d - c, *e as i8 - d);
                change.entry(key).or_insert(e);
            }
            change
        })
        .collect::<Vec<_>>();

    let mut grouped_changes: HashMap<&(i8, i8, i8, i8), usize> = HashMap::new();
    for change in changes.iter() {
        for (k, &&v) in change.iter() {
            *grouped_changes.entry(k).or_default() += v;
        }
    }
    let mut best_change = ((0, 0, 0, 0), 0);
    for change in grouped_changes {
        if change.1 > best_change.1 {
            // println!("better change: {:?}", change);
            best_change = (*change.0, change.1);
        }
    }
    // println!("best_change: {:?}", best_change);
    best_change.1
}

//-----------------------------------------------------
// Questions.

q_impl!("22");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_a(indoc!("1")), 8685429);
    assert_eq!(process_data_a(indoc!("10")), 4700978);
    assert_eq!(process_data_a(indoc!("100")), 15273692);
    assert_eq!(process_data_a(indoc!("2024")), 8667524);

    assert_eq!(
        process_data_a(indoc!(
            "
        1
        10
        100
        2024
        "
        )),
        37327623
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    // assert_eq!(process_data_b(indoc!("123")), 7);
    // assert_eq!(process_data_b(indoc!("1")), 7);
    // assert_eq!(process_data_b(indoc!("10")), 7);
    // assert_eq!(process_data_b(indoc!("100")), 0);
    // assert_eq!(process_data_b(indoc!("2024")), 9);

    assert_eq!(
        process_data_b(indoc!(
            "
        1
        2
        3
        2024
        "
        )),
        23
    );
}
