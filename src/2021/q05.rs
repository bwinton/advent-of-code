use std::collections::HashMap;

use nom::{bytes::complete::tag, character::complete::u64, sequence::tuple, IResult};

//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q05.data");

fn parse(i: &str) -> IResult<&str, (u64, u64, u64, u64)> {
    let (input, (mut x1, _, mut y1, _, mut x2, _, mut y2)) =
        tuple((u64, tag(","), u64, tag(" -> "), u64, tag(","), u64))(i)?;
    let mut temp = [(x1, y1), (x2, y2)];
    temp.sort_unstable();
    [(x1, y1), (x2, y2)] = temp;
    Ok((input, (x1, y1, x2, y2)))
}

fn process_data_a(data: &str) -> usize {
    let mut board = HashMap::new();
    for line in data.lines() {
        // Do something
        let (x1, y1, x2, y2) = parse(line).unwrap().1;
        if x1 == x2 {
            for y in y1..=y2 {
                *board.entry((x1, y)).or_insert(0) += 1;
            }
        } else if y1 == y2 {
            for x in x1..=x2 {
                *board.entry((x, y1)).or_insert(0) += 1;
            }
        }
    }
    let mut rv = 0;
    for (_coord, occurrences) in board {
        if occurrences > 1 {
            rv += 1;
        }
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut board = HashMap::new();
    for line in data.lines() {
        let (x1, y1, x2, y2) = parse(line).unwrap().1;
        if x1 == x2 {
            for y in y1..=y2 {
                *board.entry((x1, y)).or_insert(0) += 1;
            }
        } else if y1 == y2 {
            for x in x1..=x2 {
                *board.entry((x, y1)).or_insert(0) += 1;
            }
        } else if y1 < y2 {
            for (x, y) in (x1..=x2).zip(y1..=y2) {
                *board.entry((x, y)).or_insert(0) += 1;
            }
        } else {
            for (x, y) in (x1..=x2).zip(0..=(y1 - y2)) {
                *board.entry((x, y1 - y)).or_insert(0) += 1;
            }
        }
    }
    let mut rv = 0;
    for (_coord, occurrences) in board {
        if occurrences > 1 {
            rv += 1;
        }
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("5");

#[test]
fn a() {
    assert_eq!(
        process_data_a(indoc!(
            "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2
    "
        )),
        5
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(indoc!(
            "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2
    "
        )),
        12
    );
}
