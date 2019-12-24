//-----------------------------------------------------
// Setup.

use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("data/q24.data");

static BOARD_SIZE: i32 = 5;

fn set(board: &mut i32, x: i32, y: i32) {
    *board |= 1 << (y * BOARD_SIZE + x);
}

// fn clear(board: &mut i32, x: i32, y: i32) {
//     *board &= !(1 << (y * BOARD_SIZE + x));
// }

fn get(board: i32, x: i32, y: i32) -> bool {
    if x < 0 || y < 0 || x >= BOARD_SIZE || y >= BOARD_SIZE {
        return false;
    }
    board & (1 << (y * BOARD_SIZE + x)) != 0
}

fn get_adjacent(board: i32, x: i32, y: i32) -> i32 {
    let mut rv = 0;
    for (i, j) in &[(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)] {
        if get(board, x + i, y + j) {
            rv += 1;
        }
    }
    rv
}

fn step(board: i32) -> i32 {
    let mut next = 0;
    for y in 0..5 {
        for x in 0..5 {
            // Get the number of adjacent bugs.
            let adjacent = get_adjacent(board, x, y);
            if adjacent == 2 {
                set(&mut next, x, y);
            }
            if !get(board, x, y) && adjacent == 1 {
                set(&mut next, x, y);
            }
        }
    }
    next
}

#[allow(dead_code)]
fn print_board(board: i32) {
    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            print!("{}", if get(board, x, y) { '#' } else { '.' });
        }
        println!();
    }
}

fn parse_board(data: &str) -> i32 {
    let mut board: i32 = 0;
    for (y, line) in data.lines().enumerate() {
        for (x, space) in line.chars().enumerate() {
            if space == '#' {
                set(&mut board, x as i32, y as i32);
            }
        }
    }
    board
}

fn process_data_a(data: &str) -> i32 {
    let mut board = parse_board(data);
    let mut seen = HashSet::new();

    while seen.insert(board) {
        board = step(board);
    }

    board
    // Not 25952256.
    // Not 35904.
}

fn get_adjacent_recursive(boards: &HashMap<i32, i32>, level: i32, x: i32, y: i32) -> i32 {
    let mut rv = 0;
    let board = boards[&level];
    for (i, j) in &[(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)] {
        let x2 = x + i;
        let y2 = y + j;
        if (x2, y2) == (2, 2) {
            // If we're hitting the center, grab the sub-elements.
            if let Some(board) = boards.get(&(level - 1)) {
                match (x, y) {
                    (2, 1) => {
                        // coming from above, get the top row.
                        for i in 0..BOARD_SIZE {
                            if get(*board, i, 0) {
                                rv += 1;
                            }
                        }
                    }
                    (2, 3) => {
                        // coming from below, get the bottom row.
                        for i in 0..BOARD_SIZE {
                            if get(*board, i, BOARD_SIZE - 1) {
                                rv += 1;
                            }
                        }
                    }
                    (1, 2) => {
                        // coming from the left, get the left row.
                        for i in 0..BOARD_SIZE {
                            if get(*board, 0, i) {
                                rv += 1;
                            }
                        }
                    }
                    (3, 2) => {
                        // coming from the right, get the right row.
                        for i in 0..BOARD_SIZE {
                            if get(*board, BOARD_SIZE - 1, i) {
                                rv += 1;
                            }
                        }
                    }
                    _ => {
                        panic!("Trying to get (2,2) from {:?}", (x, y));
                    }
                }
            }
        } else if x2 < 0 || y2 < 0 || x2 >= BOARD_SIZE || y2 >= BOARD_SIZE {
            if let Some(board) = boards.get(&(level + 1)) {
                if x2 < 0 && get(*board, 1, 2) {
                    rv += 1;
                }
                if y2 < 0 && get(*board, 2, 1) {
                    rv += 1;
                }
                if x2 >= BOARD_SIZE && get(*board, 3, 2) {
                    rv += 1;
                }
                if y2 >= BOARD_SIZE && get(*board, 2, 3) {
                    rv += 1;
                }
            }
        } else if get(board, x2, y2) {
            rv += 1;
        }
    }
    rv
}

fn step_recursive(boards: &HashMap<i32, i32>) -> HashMap<i32, i32> {
    let mut rv = HashMap::new();
    for (level, board) in boards.iter() {
        let mut next = 0;
        for y in 0..5 {
            for x in 0..5 {
                if (x, y) == (2, 2) {
                    // Don't process the middle spot.
                    continue;
                }
                // Get the number of adjacent bugs.
                let adjacent = get_adjacent_recursive(boards, *level, x, y);
                if adjacent == 2 {
                    set(&mut next, x, y);
                    if !boards.contains_key(&(level + 1))
                        && (x == 0 || x == BOARD_SIZE - 1 || y == 0 || y == BOARD_SIZE - 1)
                    {
                        // If we're setting something on the outside, add an outer level.
                        rv.insert(level + 1, 0);
                    } else if !boards.contains_key(&(level - 1))
                        && ((x, y) == (2, 1)
                            || (x, y) == (2, 3)
                            || (x, y) == (1, 2)
                            || (x, y) == (3, 2))
                    {
                        // If we're setting something on the inside, add an inner level.
                        rv.insert(level - 1, 0);
                    }
                }
                if !get(*board, x, y) && adjacent == 1 {
                    set(&mut next, x, y);
                    if !boards.contains_key(&(level + 1))
                        && (x == 0 || x == BOARD_SIZE - 1 || y == 0 || y == BOARD_SIZE - 1)
                    {
                        // If we're setting something on the outside, add an outer level.
                        rv.insert(level + 1, 0);
                    } else if !boards.contains_key(&(level - 1))
                        && ((x, y) == (2, 1)
                            || (x, y) == (2, 3)
                            || (x, y) == (1, 2)
                            || (x, y) == (3, 2))
                    {
                        // If we're setting something on the inside, add an inner level.
                        rv.insert(level - 1, 0);
                    }
                }
            }
        }
        rv.insert(*level, next);
    }
    rv
}
fn run_steps(data: &str, iterations: usize) -> u32 {
    let mut boards = HashMap::new();
    boards.insert(-1, 0);
    boards.insert(0, parse_board(data));
    boards.insert(1, 0);

    for _ in 0..iterations {
        boards = step_recursive(&boards);
    }

    boards.values().map(|board| board.count_ones()).sum()
}

fn process_data_b(data: &str) -> u32 {
    run_steps(data, 200)
}

//-----------------------------------------------------
// Questions.

q_impl!("24");

#[test]
fn a() {
    let start = parse_board(
        "....#
#..#.
#..##
..#..
#....",
    );

    let one = parse_board(
        "#..#.
####.
###.#
##.##
.##..",
    );

    let two = parse_board(
        "#####
....#
....#
...#.
#.###",
    );

    let three = parse_board(
        "#....
####.
...##
#.##.
.##.#",
    );

    let four = parse_board(
        "####.
....#
##..#
.....
##...",
    );
    assert_eq!(step(start), one);
    assert_eq!(step(one), two);
    assert_eq!(step(two), three);
    assert_eq!(step(three), four);
}

#[test]
fn b() {
    assert_eq!(
        run_steps(
            "....#
#..#.
#.?##
..#..
#....",
            10
        ),
        99
    );
}
