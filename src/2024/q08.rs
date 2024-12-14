//-----------------------------------------------------
// Setup.

use std::collections::{HashMap, HashSet};

use aoc::util::{Point2, in_bounds};
use itertools::Itertools;

static INPUT: &str = include_str!("data/q08.data");

fn process_data_a(data: &str) -> usize {
    let origin = (0, 0);
    let bounds = (
        data.lines().next().unwrap().chars().count() as i64,
        data.lines().count() as i64,
    );
    let mut antennas: HashMap<char, Vec<Point2>> = HashMap::new();
    for (y, line) in data.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell != '.' {
                antennas.entry(cell).or_default().push((x as i64, y as i64));
            }
        }
    }
    let mut antinodes = HashSet::new();
    for (_, v) in antennas.clone() {
        for v in v.into_iter().combinations(2) {
            let (a, b) = (v[0], v[1]);
            let diff_x = a.0 - b.0;
            let diff_y = a.1 - b.1;
            let one: (i64, i64) = (a.0 + diff_x, a.1 + diff_y);
            let two = (b.0 - diff_x, b.1 - diff_y);

            if in_bounds(one, origin, bounds) {
                antinodes.insert(one);
            }
            if in_bounds(two, origin, bounds) {
                antinodes.insert(two);
            }
        }
    }
    antinodes.len()
}

fn process_data_b(data: &str) -> usize {
    let origin = (0, 0);
    let bounds = (
        data.lines().next().unwrap().chars().count() as i64,
        data.lines().count() as i64,
    );
    let mut antennas: HashMap<char, Vec<Point2>> = HashMap::new();
    for (y, line) in data.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell != '.' {
                antennas.entry(cell).or_default().push((x as i64, y as i64));
            }
        }
    }
    let mut antinodes = HashSet::new();
    for (_, v) in antennas.clone() {
        for v in v.into_iter().combinations(2) {
            let (a, b) = (v[0], v[1]);
            let diff_x = a.0 - b.0;
            let diff_y = a.1 - b.1;

            let mut next = a;
            while in_bounds(next, origin, bounds) {
                antinodes.insert(next);
                next = (next.0 + diff_x, next.1 + diff_y);
            }

            next = a;
            while in_bounds(next, origin, bounds) {
                antinodes.insert(next);
                next = (next.0 - diff_x, next.1 - diff_y);
            }
        }
    }
    antinodes.len()
}

//-----------------------------------------------------
// Questions.

q_impl!("8");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "
    ..........
    ..........
    ..........
    ....a.....
    ..........
    .....a....
    ..........
    ..........
    ..........
    ..........
    "
        )),
        2
    );

    assert_eq!(
        process_data_a(indoc!(
            "
    ..........
    ..........
    ..........
    ....a.....
    ........a.
    .....a....
    ..........
    ..........
    ..........
    .......... 
    "
        )),
        4
    );

    assert_eq!(
        process_data_a(indoc!(
            "
    ............
    ........0...
    .....0......
    .......0....
    ....0.......
    ......A.....
    ............
    ............
    ........A...
    .........A..
    ............
    ............
    "
        )),
        14
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "
    T.........
    ...T......
    .T........
    ..........
    ..........
    ..........
    ..........
    ..........
    ..........
    ..........
    "
        )),
        9
    );

    assert_eq!(
        process_data_b(indoc!(
            "
    ............
    ........0...
    .....0......
    .......0....
    ....0.......
    ......A.....
    ............
    ............
    ........A...
    .........A..
    ............
    ............
    "
        )),
        34
    );
}
