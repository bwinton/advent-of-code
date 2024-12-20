//-----------------------------------------------------
// Setup.

use std::{cmp::Ordering, collections::HashMap};

use aoc::util::{Direction, Point2, in_bounds};

static INPUT: &str = include_str!("data/q20.data");

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Wall,
    Empty,
}

fn parse_data(data: &str) -> (Vec<Vec<Cell>>, HashMap<Point2, i64>) {
    let mut map = vec![];
    let mut start = None;
    let mut end = None;
    for (y, line) in data.lines().enumerate() {
        let mut row = vec![];
        for (x, cell) in line.chars().enumerate() {
            match cell {
                '#' => row.push(Cell::Wall),
                '.' => row.push(Cell::Empty),
                'S' => {
                    row.push(Cell::Empty);
                    start = Some((x as i64, y as i64));
                }
                'E' => {
                    row.push(Cell::Empty);
                    end = Some((x as i64, y as i64));
                }
                _ => panic!("Unknown char {} at ({},{})", cell, x, y),
            }
        }
        map.push(row);
    }
    let mut path = vec![];
    let mut curr = start.unwrap();
    let mut prev = curr;
    let end = end.unwrap();
    let min = Some((0, 0));
    let max = Some((map[0].len() as i64, map.len() as i64));
    while curr != end {
        path.push(curr);
        for direction in Direction::all() {
            if let Some(next) = direction.move_pos(curr, 1, min, max) {
                if next != prev && map[next.1 as usize][next.0 as usize] == Cell::Empty {
                    prev = curr;
                    curr = next;
                    break;
                }
            }
        }
    }
    path.push(end);
    let path = path
        .iter()
        .enumerate()
        .map(|(x, &y)| (y, x as i64))
        .collect();

    (map, path)
}

fn find_cheats(map: &[Vec<Cell>], path: &HashMap<Point2, i64>, size: i64) -> HashMap<usize, usize> {
    let origin = (0, 0);
    let bounds = (map[0].len() as i64, map.len() as i64);
    let mut rv: HashMap<usize, usize> = HashMap::new();
    for start_y in 0..bounds.1 {
        for start_x in 0..bounds.0 {
            let start = (start_x, start_y);
            if map[start_y as usize][start_x as usize] == Cell::Empty {
                for delta_y in -size..=size {
                    for delta_x in -size + delta_y.abs()..=size - delta_y.abs() {
                        let end = (start_x + delta_x, start_y + delta_y);
                        if start.cmp(&end) != Ordering::Less {
                            continue;
                        }
                        if in_bounds(end, origin, bounds)
                            && map[end.1 as usize][end.0 as usize] == Cell::Empty
                        {
                            let distance = (path[&start] - path[&end]).abs();
                            let saved = distance - (delta_x.abs() + delta_y.abs());
                            if saved > 0 {
                                *rv.entry(saved as usize).or_default() += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    rv
}

fn process_data(data: &str, size: i64) -> usize {
    let mut rv = 0;
    let (map, path) = parse_data(data);

    let cheats = find_cheats(&map, &path, size);
    for len in cheats.keys() {
        if *len < 100 {
            continue;
        }
        rv += cheats[len];
    }
    rv
}

fn process_data_a(data: &str) -> usize {
    process_data(data, 2)
}

fn process_data_b(data: &str) -> usize {
    process_data(data, 20)
}

//-----------------------------------------------------
// Questions.

q_impl!("20");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    let data = indoc!(
        "
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############
        "
    );

    let expected = HashMap::from([
        (2, 14),
        (4, 14),
        (6, 2),
        (8, 4),
        (10, 2),
        (12, 3),
        (20, 1),
        (36, 1),
        (38, 1),
        (40, 1),
        (64, 1),
    ]);

    assert_eq!(process_data_a(data), 0);

    let (map, distances) = parse_data(data);
    let cheats = find_cheats(&map, &distances, 2);
    assert_eq!(cheats, expected);
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    let data = indoc!(
        "
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############
        "
    );

    let expected = HashMap::from([
        (50, 32),
        (52, 31),
        (54, 29),
        (56, 39),
        (58, 25),
        (60, 23),
        (62, 20),
        (64, 19),
        (66, 12),
        (68, 14),
        (70, 12),
        (72, 22),
        (74, 4),
        (76, 3),
    ]);

    let (map, distances) = parse_data(data);
    let mut cheats = find_cheats(&map, &distances, 20);
    let mut removals = vec![];
    for &key in cheats.keys() {
        if key < 50 {
            removals.push(key);
        }
    }
    for key in removals {
        cheats.remove(&key);
    }

    assert_eq!(cheats, expected);

    assert_eq!(process_data_b(data), 0);
}
