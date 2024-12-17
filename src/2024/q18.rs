//-----------------------------------------------------
// Setup.

use core::pat;
use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}};

use aoc::util::{Direction, Point2};

static INPUT: &str = include_str!("data/q18.data");

fn parse_data(data: &str, range: usize) -> Vec<Vec<(bool, usize)>> {
    let mut rv = vec![vec![(false, 0); range]; range];
    for (i, line) in data.lines().enumerate() {
        let (x, y) = line.split_once(",").unwrap();
        let x = x.parse::<usize>().unwrap();
        let y = y.parse::<usize>().unwrap();
        rv[y][x] = (true, i);
    }
    rv
}

fn find_path(map: &[Vec<(bool, usize)>], steps: usize, falling: bool) -> Vec<Vec<Point2>> {
    let origin = (0, 0);
    let bounds = (map[0].len() as i64, map.len() as i64);
    let end = (bounds.0 - 1, bounds.1 - 1);
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), origin, vec![]));
    let mut seen: HashMap<(i64, i64), Vec<Vec<Point2>>> = HashMap::new();
    while let Some((score, curr, mut path)) = heap.pop() {
        println!("{}, {:?}", score.0, curr);
        if curr == end {
            return path;
        }
        path.push(curr);
        if let Some(paths) = seen.get_mut(&curr) {
            println!("  Seen it.");
            paths.push(path.clone());
            continue;
        } else {
            seen.insert(curr, vec![path.clone()]);
        }
        // seen.insert(curr, path.clone());
        for direction in Direction::all() {
            if let Some(next) = direction.move_pos(curr, 1, Some(origin), Some(bounds)) {
                print!("  {:?} / {:?} => ", next, map[next.1 as usize][next.0 as usize]);
                match map[next.1 as usize][next.0 as usize] {
                    (false, _) => {
                        println!("pushing.");
                        heap.push((Reverse(score.0 + 1), next, path.clone()));
                    }
                    (true, t) if t >= steps && (!falling || t > score.0) => {
                        println!("pushing.");
                        heap.push((Reverse(score.0 + 1), next, path.clone()));
                    }
                    _ => {
                        println!("skipping.");
                        // Can't go here, so skip it.   
                    }
                }
            }
        }
    }
    panic!("No path found…");
}


fn process_data_a(data: &str) -> usize {
    let map = parse_data(data, 71);
    println!("map: {:?}", map);
    let path = find_path(&map, 1024, false);
    path.len()
}

fn process_data_b(data: &str) -> usize {
    let rv = 0;
    for _line in data.lines() {
        // Do something
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("18");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    let map = parse_data(indoc!("
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
    "), 7);
    assert_eq!(find_path(&map, 12, false).len(), 22);
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    let map = parse_data(indoc!("
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
    "), 7);
    let path = find_path(&map, 12, true);
    let mut min = 900;
    for (x,y ) in path {
        let cell = map[y as usize][x as usize];
        println!("{:?} => {:?}", (x, y), cell);
    }
    // assert_eq!(path.len(), 22);
}
