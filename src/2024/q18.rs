//-----------------------------------------------------
// Setup.

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use aoc::util::{Direction, Point2};
use itertools::Itertools;

static INPUT: &str = include_str!("data/q18.data");

fn parse_data(data: &str) -> Vec<Point2> {
    let mut rv = vec![];
    for line in data.lines() {
        let (x, y) = line.split_once(",").unwrap();
        let x = x.parse::<i64>().unwrap();
        let y = y.parse::<i64>().unwrap();
        rv.push((x, y));
    }
    rv
}

fn get_map(rocks: &[Point2], range: usize) -> Vec<Vec<bool>> {
    let mut rv = vec![vec![false; range]; range];
    for &(x, y) in rocks {
        rv[y as usize][x as usize] = true;
    }
    rv
}

fn find_path(map: &[Vec<bool>]) -> Option<usize> {
    let origin = (0, 0);
    let bounds = (map[0].len() as i64, map.len() as i64);
    let end = (bounds.0 - 1, bounds.1 - 1);
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), origin));
    let mut seen = HashSet::new();
    while let Some((score, curr)) = heap.pop() {
        if curr == end {
            return Some(score.0);
        }
        if seen.contains(&curr) {
            continue;
        }
        seen.insert(curr);
        for direction in Direction::all() {
            if let Some(next) = direction.move_pos(curr, 1, Some(origin), Some(bounds))
                && !map[next.1 as usize][next.0 as usize] {
                    heap.push((Reverse(score.0 + 1), next));
                }
        }
    }
    None
}

fn find_rock(rocks: &[Point2], range: usize) -> Point2 {
    let rv = (0..rocks.len()).collect_vec().partition_point(|&time| {
        let map = get_map(&rocks[..time], range);
        find_path(&map).is_some()
    }) - 1;
    rocks[rv]
}

fn process_data_a(data: &str) -> usize {
    let rocks = parse_data(data);
    let map = get_map(&rocks[..1024], 71);
    find_path(&map).unwrap()
}

fn process_data_b(data: &str) -> String {
    let rocks = parse_data(data);
    let rv = find_rock(&rocks, 71);
    format!("{},{}", rv.0, rv.1)
}

//-----------------------------------------------------
// Questions.

q_impl!("18");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    let rocks = parse_data(indoc!(
        "
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0
        "
    ));
    let map = get_map(&rocks[..12], 7);
    assert_eq!(find_path(&map).unwrap(), 22);
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    let rocks = parse_data(indoc!(
        "
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0
        "
    ));
    assert_eq!(find_rock(&rocks, 7), (6, 1));
}
