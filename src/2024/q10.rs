//-----------------------------------------------------
// Setup.

use std::collections::HashSet;

use aoc::util::{Direction, Point2};

static INPUT: &str = include_str!("data/q10.data");

fn parse(data: &str) -> (Vec<Vec<u8>>, Vec<Point2>) {
    let mut map = vec![];
    let mut trailheads = vec![];
    for (y, line) in data.lines().enumerate() {
        let mut row = vec![];
        for (x, cell) in line.chars().enumerate() {
            if cell.is_ascii_digit() {
                row.push(cell as u8 - b'0');
            } else {
                row.push(99);
            }
            if cell == '0' {
                trailheads.push((x as i64, y as i64));
            }
        }
        map.push(row);
    }
    (map, trailheads)
}

fn find_trails(trailhead: Point2, map: &[Vec<u8>]) -> usize {
    let mut rv = HashSet::new();
    let mut stack = vec![];
    let origin = Some((0, 0));
    let bounds = Some((map[0].len() as i64, map.len() as i64));
    stack.push(trailhead);
    while let Some(curr) = stack.pop() {
        let curr_value = map[curr.1 as usize][curr.0 as usize];
        if curr_value == 9 {
            rv.insert(curr);
            continue;
        }
        for dir in Direction::all() {
            if let Some(next) = dir.move_pos(curr, 1, origin, bounds) {
                let next_value = map[next.1 as usize][next.0 as usize];
                if next_value == curr_value + 1 {
                    stack.push(next);
                }
            }
        }
    }
    rv.len()
}

fn find_rating(trailhead: Point2, map: &[Vec<u8>]) -> usize {
    let mut rv = 0;
    let mut stack = vec![];
    let origin = Some((0, 0));
    let bounds = Some((map[0].len() as i64, map.len() as i64));
    stack.push(trailhead);
    while let Some(curr) = stack.pop() {
        let curr_value = map[curr.1 as usize][curr.0 as usize];
        if curr_value == 9 {
            rv += 1;
            continue;
        }
        for dir in Direction::all() {
            if let Some(next) = dir.move_pos(curr, 1, origin, bounds) {
                let next_value = map[next.1 as usize][next.0 as usize];
                if next_value == curr_value + 1 {
                    stack.push(next);
                }
            }
        }
    }
    rv
}

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    let (map, trailheads) = parse(data);
    for trailhead in trailheads {
        rv += find_trails(trailhead, &map);
    }

    rv
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    let (map, trailheads) = parse(data);
    for trailhead in trailheads {
        rv += find_rating(trailhead, &map);
    }

    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("10");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "
    ...0...
    ...1...
    ...2...
    6543456
    7.....7
    8.....8
    9.....9
    "
        )),
        2
    );
    assert_eq!(
        process_data_a(indoc!(
            "
    ..90..9
    ...1.98
    ...2..7
    6543456
    765.987
    876....
    987....
    "
        )),
        4
    );
    assert_eq!(
        process_data_a(indoc!(
            "
    89010123
    78121874
    87430965
    96549874
    45678903
    32019012
    01329801
    10456732
    "
        )),
        36
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "
    89010123
    78121874
    87430965
    96549874
    45678903
    32019012
    01329801
    10456732
    "
        )),
        81
    );
}
