//-----------------------------------------------------
// Setup.

use std::collections::HashSet;

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending, one_of},
    multi::separated_list1,
    sequence::separated_pair,
};

static INPUT: &str = include_str!("data/q09.data");

#[derive(Debug)]
struct Move {
    dir: String,
    distance: u32,
}

impl Move {
    fn move_head(&self, head: &mut (i32, i32)) {
        match self.dir.as_str() {
            "L" => {
                head.0 -= 1;
            }
            "R" => {
                head.0 += 1;
            }
            "U" => {
                head.1 -= 1;
            }
            "D" => {
                head.1 += 1;
            }
            _ => panic!("Invalid direction: {:?}", self),
        }
    }

    fn move_tail(head: &(i32, i32), tail: &mut (i32, i32)) {
        // check for too far inline.
        if tail.1 == head.1 {
            if tail.0 - head.0 > 1 {
                tail.0 -= 1;
            }
            if head.0 - tail.0 > 1 {
                tail.0 += 1;
            }
        }
        if tail.0 == head.0 {
            if tail.1 - head.1 > 1 {
                tail.1 -= 1;
            }
            if head.1 - tail.1 > 1 {
                tail.1 += 1;
            }
        }
        // check for diagonals.
        if tail == &(head.0 - 1, head.1 - 2) {
            tail.0 += 1;
            tail.1 += 1;
        }
        if tail == &(head.0 - 1, head.1 + 2) {
            tail.0 += 1;
            tail.1 -= 1;
        }
        if tail == &(head.0 + 1, head.1 - 2) {
            tail.0 -= 1;
            tail.1 += 1;
        }
        if tail == &(head.0 + 1, head.1 + 2) {
            tail.0 -= 1;
            tail.1 -= 1;
        }
        if tail == &(head.0 - 2, head.1 - 1) {
            tail.0 += 1;
            tail.1 += 1;
        }
        if tail == &(head.0 + 2, head.1 - 1) {
            tail.0 -= 1;
            tail.1 += 1;
        }
        if tail == &(head.0 - 2, head.1 + 1) {
            tail.0 += 1;
            tail.1 -= 1;
        }
        if tail == &(head.0 + 2, head.1 + 1) {
            tail.0 -= 1;
            tail.1 -= 1;
        }
        if tail == &(head.0 - 2, head.1 - 2) {
            tail.0 += 1;
            tail.1 += 1;
        }
        if tail == &(head.0 - 2, head.1 + 2) {
            tail.0 += 1;
            tail.1 -= 1;
        }
        if tail == &(head.0 + 2, head.1 - 2) {
            tail.0 -= 1;
            tail.1 += 1;
        }
        if tail == &(head.0 + 2, head.1 + 2) {
            tail.0 -= 1;
            tail.1 -= 1;
        }
    }
}

fn moove(i: &str) -> IResult<&str, Move> {
    let (input, (dir, distance)) =
        separated_pair(one_of("LRUD"), tag(" "), complete::u32).parse(i)?;
    Ok((
        input,
        Move {
            dir: String::from(dir),
            distance,
        },
    ))
}

fn parser(i: &str) -> IResult<&str, Vec<Move>> {
    let (input, list) = separated_list1(line_ending, moove).parse(i)?;
    Ok((input, list))
}

fn process_data_a(data: &str) -> usize {
    let moves = parser(data).unwrap().1;
    let mut segments = [(0, 0), (0, 0)];
    let mut tail_positions = HashSet::new();
    let last = segments.len() - 1;
    tail_positions.insert(segments[last]);
    for moove in moves {
        for _ in 0..moove.distance {
            moove.move_head(&mut segments[0]);
            for i in 0..segments.len() - 1 {
                let curr = segments[i];
                Move::move_tail(&curr, &mut segments[i + 1]);
            }
            tail_positions.insert(segments[last]);
        }
    }
    tail_positions.len()
}

fn process_data_b(data: &str) -> usize {
    let moves = parser(data).unwrap().1;
    let mut segments = [
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];
    let mut tail_positions = HashSet::new();
    let last = segments.len() - 1;
    tail_positions.insert(segments[last]);
    for moove in moves {
        for _ in 0..moove.distance {
            moove.move_head(&mut segments[0]);
            for i in 0..segments.len() - 1 {
                let curr = segments[i];
                Move::move_tail(&curr, &mut segments[i + 1]);
            }
            tail_positions.insert(segments[last]);
        }
    }
    tail_positions.len()
}

//-----------------------------------------------------
// Questions.

q_impl!("9");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "R 4
    U 4
    L 3
    D 1
    R 4
    D 1
    L 5
    R 2
    "
        )),
        13
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "R 4
    U 4
    L 3
    D 1
    R 4
    D 1
    L 5
    R 2
    "
        )),
        1
    );
    assert_eq!(
        process_data_b(indoc!(
            "R 5
    U 8
    L 8
    D 3
    R 17
    D 10
    L 25
    U 20    
    "
        )),
        36
    );
}
