//-----------------------------------------------------
// Setup.

use std::collections::HashMap;

use aoc::util::Point2;
use itertools::Itertools;

static INPUT: &str = include_str!("data/q10.data");

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
    Unknown,
}

fn parse_data(data: &str) -> (HashMap<Point2, (Direction, Direction)>, Point2) {
    let mut map = HashMap::new();
    let mut start = None;
    for (y, line) in data.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            match cell {
                // a vertical pipe connecting north and south.
                '|' => {
                    map.insert((x, y), (Direction::North, Direction::South));
                }
                // a horizontal pipe connecting east and west.
                '-' => {
                    map.insert((x, y), (Direction::East, Direction::West));
                }
                // a 90-degree bend connecting north and east.
                'L' => {
                    map.insert((x, y), (Direction::North, Direction::East));
                }
                // a 90-degree bend connecting north and west.
                'J' => {
                    map.insert((x, y), (Direction::North, Direction::West));
                }
                // a 90-degree bend connecting south and west.
                '7' => {
                    map.insert((x, y), (Direction::South, Direction::West));
                }
                // a 90-degree bend connecting south and east.
                'F' => {
                    map.insert((x, y), (Direction::South, Direction::East));
                }
                // ground; there is no pipe in this tile.
                '.' | 'O' | 'I' => {}
                // the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
                'S' => {
                    map.insert((x, y), (Direction::Unknown, Direction::Unknown));
                    start = Some((x, y));
                }
                _ => {
                    panic!("Unknown character! {}", cell);
                }
            }
        }
    }
    let start = start.unwrap();
    (map, start)
}

fn get(start: Point2, map: &mut HashMap<Point2, (Direction, Direction)>) {
    let mut start_cell = vec![];
    if let Some((a, b)) = map.get(&(start.0, start.1.overflowing_sub(1).0)) {
        if *a == Direction::South || *b == Direction::South {
            start_cell.push(Direction::North);
        }
    }
    if let Some((a, b)) = map.get(&(start.0, start.1 + 1)) {
        if *a == Direction::North || *b == Direction::North {
            start_cell.push(Direction::South);
        }
    }
    if let Some((a, b)) = map.get(&(start.0.overflowing_sub(1).0, start.1)) {
        if *a == Direction::East || *b == Direction::East {
            start_cell.push(Direction::West);
        }
    }
    if let Some((a, b)) = map.get(&(start.0 + 1, start.1)) {
        if *a == Direction::West || *b == Direction::West {
            start_cell.push(Direction::East);
        }
    }
    *map.get_mut(&start).unwrap() = start_cell.into_iter().collect_tuple().unwrap();
}

fn process_data_a(data: &str) -> usize {
    let (mut map, start) = parse_data(data);
    get(start, &mut map);
    let mut curr = (start, map.get(&start).unwrap().0);
    let mut moves = 0;
    loop {
        moves += 1;
        match curr.1 {
            Direction::North => {
                curr.0 .1 -= 1;
                let directions = map.get(&curr.0).unwrap();
                curr.1 = if directions.0 == Direction::South {
                    directions.1
                } else {
                    directions.0
                };
            }
            Direction::East => {
                curr.0 .0 += 1;
                let directions = map.get(&curr.0).unwrap();
                curr.1 = if directions.0 == Direction::West {
                    directions.1
                } else {
                    directions.0
                };
            }
            Direction::South => {
                curr.0 .1 += 1;
                let directions = map.get(&curr.0).unwrap();
                curr.1 = if directions.0 == Direction::North {
                    directions.1
                } else {
                    directions.0
                };
            }
            Direction::West => {
                curr.0 .0 -= 1;
                let directions = map.get(&curr.0).unwrap();
                curr.1 = if directions.0 == Direction::East {
                    directions.1
                } else {
                    directions.0
                };
            }
            _ => panic!("We should know everything for {:?}", curr),
        }
        if curr.0 == start {
            break;
        }
    }
    moves / 2
}

fn process_data_b(data: &str) -> usize {
    let (mut map, start) = parse_data(data);
    get(start, &mut map);
    let mut curr = (start, map.get(&start).unwrap().0);
    let mut tiles = HashMap::new();
    loop {
        tiles.insert(curr.0, map.get(&curr.0).unwrap());
        match curr.1 {
            Direction::North => {
                curr.0 .1 -= 1;
                let directions = map.get(&curr.0).unwrap();
                curr.1 = if directions.0 == Direction::South {
                    directions.1
                } else {
                    directions.0
                };
            }
            Direction::East => {
                curr.0 .0 += 1;
                let directions = map.get(&curr.0).unwrap();
                curr.1 = if directions.0 == Direction::West {
                    directions.1
                } else {
                    directions.0
                };
            }
            Direction::South => {
                curr.0 .1 += 1;
                let directions = map.get(&curr.0).unwrap();
                curr.1 = if directions.0 == Direction::North {
                    directions.1
                } else {
                    directions.0
                };
            }
            Direction::West => {
                curr.0 .0 -= 1;
                let directions = map.get(&curr.0).unwrap();
                curr.1 = if directions.0 == Direction::East {
                    directions.1
                } else {
                    directions.0
                };
            }
            _ => panic!("We should know everything for {:?}", curr),
        }
        if curr.0 == start {
            break;
        }
    }

    let min_x = tiles.iter().map(|t| t.0 .0).min().unwrap();
    let min_y = tiles.iter().map(|t| t.0 .1).min().unwrap();
    let max_x = tiles.iter().map(|t| t.0 .0).max().unwrap();
    let max_y = tiles.iter().map(|t| t.0 .1).max().unwrap();

    let mut rv = 0;

    for y in min_y..=max_y {
        let mut inside = false;
        for x in min_x..=max_x {
            if let Some(&tile) = tiles.get(&(x, y)) {
                match *tile {
                    (Direction::North, _) | (_, Direction::North) => {
                        inside = !inside;
                    }
                    _ => {}
                };
            } else if inside {
                rv += 1;
            }
        }
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
            ".....
    .S-7.
    .|.|.
    .L-J.
    ....."
        )),
        4
    );
    assert_eq!(
        process_data_a(indoc!(
            "..F7.
    .FJ|.
    SJ.L7
    |F--J
    LJ..."
        )),
        8
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "...........
    .S-------7.
    .|F-----7|.
    .||.....||.
    .||.....||.
    .|L-7.F-J|.
    .|..|.|..|.
    .L--J.L--J.
    ..........."
        )),
        4
    );

    assert_eq!(
        process_data_b(indoc!(
            "..........
    .S------7.
    .|F----7|.
    .||OOOO||.
    .||OOOO||.
    .|L-7F-J|.
    .|II||II|.
    .L--JL--J.
    .........."
        )),
        4
    );

    assert_eq!(
        process_data_b(indoc!(
            ".F----7F7F7F7F-7....
    .|F--7||||||||FJ....
    .||.FJ||||||||L7....
    FJL7L7LJLJ||LJ.L-7..
    L--J.L7...LJS7F-7L7.
    ....F-J..F7FJ|L7L7L7
    ....L7.F7||L7|.L7L7|
    .....|FJLJ|FJ|F7|.LJ
    ....FJL-7.||.||||...
    ....L---J.LJ.LJLJ...
    "
        )),
        8
    );

    assert_eq!(
        process_data_b(indoc!(
            "FF7FSF7F7F7F7F7F---7
    L|LJ||||||||||||F--J
    FL-7LJLJ||||||LJL-77
    F--JF--7||LJLJ7F7FJ-
    L---JF-JLJ.||-FJLJJ7
    |F|F-JF---7F7-L7L|7|
    |FFJF7L7F-JF7|JL---7
    7-L-JL7||F7|L7F-7F7|
    L.L7LFJ|||||FJL7||LJ
    L7JLJL-JLJLJL--JLJ.L
    "
        )),
        10
    );
}
