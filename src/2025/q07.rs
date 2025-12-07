//-----------------------------------------------------
// Setup.

use std::collections::{HashMap, HashSet};

use aoc::util::Point2;

static INPUT: &str = include_str!("data/q07.data");

fn parse(data: &str) -> (Point2, Point2, HashSet<Point2>) {
    let mut start = (0, 0);
    let mut map = HashSet::new();
    let bounds = (
        data.lines().next().unwrap().len() as i64,
        data.lines().count() as i64,
    );
    for (y, line) in data.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            match cell {
                'S' => {
                    start = (x as i64, y as i64);
                }
                '^' => {
                    map.insert((x as i64, y as i64));
                }
                '.' => {} // pass
                _ => {
                    panic!("Unknown cell! {cell}");
                }
            }
        }
    }
    (start, bounds, map)
}

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    let (start, bounds, map) = parse(data);
    let mut beams = HashSet::from([start.0]);
    for y in start.1..=bounds.1 {
        let mut next = HashSet::new();
        for beam in beams {
            if map.contains(&(beam, y)) {
                rv += 1;
                next.insert(beam - 1);
                next.insert(beam + 1);
            } else {
                next.insert(beam);
            }
        }
        beams = next;
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let (start, bounds, map) = parse(data);
    let mut beams = HashMap::new();
    beams.insert(start.0, 1);
    for y in start.1..=bounds.1 {
        let mut next = HashMap::new();
        for (&beam, &count) in beams.iter() {
            if map.contains(&(beam, y)) {
                *next.entry(beam - 1).or_default() += count;
                *next.entry(beam + 1).or_default() += count;
            } else {
                *next.entry(beam).or_default() += count;
            }
        }
        beams = next;
    }
    beams.values().sum()
}

//-----------------------------------------------------
// Questions.

q_impl!("7");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"
        )),
        21
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"
        )),
        40
    );
}
