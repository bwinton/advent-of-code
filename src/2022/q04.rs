//-----------------------------------------------------
// Setup.

use std::ops::RangeInclusive;

static INPUT: &str = include_str!("data/q04.data");

fn get_segment(elves: &str) -> RangeInclusive<usize> {
    let (start, end) = elves.split_once('-').unwrap();
    start.parse().unwrap()..=end.parse().unwrap()
}

fn get_elves(line: &str) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    let (first, second) = line.split_once(',').unwrap();
    let first = get_segment(first);
    let second = get_segment(second);
    (first, second)
}

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    for line in data.lines() {
        let (first, second) = get_elves(line);
        if first.start() >= second.start() && first.end() <= second.end()
            || second.start() >= first.start() && second.end() <= first.end()
        {
            rv += 1;
        }
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    for line in data.lines() {
        let (first, second) = get_elves(line);
        if first.contains(second.start()) || second.contains(first.start()) {
            rv += 1;
        }
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("4");

#[test]
fn a() {
    assert_eq!(
        process_data_a(indoc!(
            "2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8
    "
        )),
        2
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(indoc!(
            "2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8
    "
        )),
        4
    );
}
