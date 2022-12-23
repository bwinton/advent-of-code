//-----------------------------------------------------
// Setup.

use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("data/q23.data");

fn get_adjacent(elf: &(i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (elf.0 - 1, elf.1 - 1),
        (elf.0 - 1, elf.1),
        (elf.0 - 1, elf.1 + 1),
        (elf.0, elf.1 - 1),
        (elf.0, elf.1 + 1),
        (elf.0 + 1, elf.1 - 1),
        (elf.0 + 1, elf.1),
        (elf.0 + 1, elf.1 + 1),
    ]
}

enum Move {
    North,
    South,
    East,
    West,
}

impl Move {
    fn try_move(&self, elf: &(i32, i32), elves: &HashSet<(i32, i32)>) -> Option<(i32, i32)> {
        match self {
            Move::North => {
                let north = vec![
                    (elf.0 - 1, elf.1 - 1),
                    (elf.0, elf.1 - 1),
                    (elf.0 + 1, elf.1 - 1),
                ];
                let mut found = true;
                for next in north {
                    if elves.contains(&next) {
                        found = false;
                        break;
                    }
                }
                if found {
                    Some((elf.0, elf.1 - 1))
                } else {
                    None
                }
            }
            Move::South => {
                let south = vec![
                    (elf.0 - 1, elf.1 + 1),
                    (elf.0, elf.1 + 1),
                    (elf.0 + 1, elf.1 + 1),
                ];
                let mut found = true;
                for next in south {
                    if elves.contains(&next) {
                        found = false;
                        break;
                    }
                }
                if found {
                    Some((elf.0, elf.1 + 1))
                } else {
                    None
                }
            }
            Move::West => {
                let west = vec![
                    (elf.0 - 1, elf.1 - 1),
                    (elf.0 - 1, elf.1),
                    (elf.0 - 1, elf.1 + 1),
                ];
                let mut found = true;
                for next in west {
                    if elves.contains(&next) {
                        found = false;
                        break;
                    }
                }
                if found {
                    Some((elf.0 - 1, elf.1))
                } else {
                    None
                }
            }
            Move::East => {
                let east = vec![
                    (elf.0 + 1, elf.1 - 1),
                    (elf.0 + 1, elf.1),
                    (elf.0 + 1, elf.1 + 1),
                ];
                let mut found = true;
                for next in east {
                    if elves.contains(&next) {
                        found = false;
                        break;
                    }
                }
                if found {
                    Some((elf.0 + 1, elf.1))
                } else {
                    None
                }
            }
        }
    }
}

fn propose(elf: &(i32, i32), elves: &HashSet<(i32, i32)>, moves: &[Move]) -> Option<(i32, i32)> {
    for next in moves {
        if let Some(value) = next.try_move(elf, elves) {
            return Some(value);
        }
    }
    None
}

#[allow(unused)]
// For debugging
fn print_board(elves: &HashSet<(i32, i32)>) {
    let mut min = (i32::MAX, i32::MAX);
    let mut max = (i32::MIN, i32::MIN);
    for elf in elves {
        min.0 = min.0.min(elf.0);
        min.1 = min.1.min(elf.1);
        max.0 = max.0.max(elf.0);
        max.1 = max.1.max(elf.1);
    }

    for y in min.1..=max.1 {
        for x in min.0..=max.0 {
            if elves.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn process_data_a(data: &str) -> i32 {
    let mut elves = HashSet::new();
    for (y, line) in data.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell == '#' {
                elves.insert((x as i32, y as i32));
            }
        }
    }
    // print_board(&elves);
    let elf_count = elves.len() as i32;

    let mut moves = vec![Move::North, Move::South, Move::West, Move::East];
    // let mut done = false;
    for _i in 0..10 {
        // done = true;
        let mut proposed: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
        for elf in &elves {
            for adjacent in get_adjacent(elf) {
                if elves.contains(&adjacent) {
                    // propose a move.
                    if let Some(next) = propose(elf, &elves, &moves) {
                        proposed.entry(next).or_default().push(*elf);
                    }
                    break;
                }
            }
        }

        // println!("{}: proposed: {:?}", i + 1, proposed);
        for (next, curr) in proposed {
            if curr.len() == 1 {
                // println!("  Moving {:?} to {:?}", curr[0], next);
                // done = false;
                elves.remove(&curr[0]);
                elves.insert(next);
            }
        }
        // print_board(&elves);

        moves.rotate_left(1);
    }

    let mut min = (i32::MAX, i32::MAX);
    let mut max = (i32::MIN, i32::MIN);
    for elf in elves {
        min.0 = min.0.min(elf.0);
        min.1 = min.1.min(elf.1);
        max.0 = max.0.max(elf.0);
        max.1 = max.1.max(elf.1);
    }

    (max.0 - min.0 + 1) * (max.1 - min.1 + 1) - elf_count
}

fn process_data_b(data: &str) -> usize {
    let mut elves = HashSet::new();
    for (y, line) in data.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell == '#' {
                elves.insert((x as i32, y as i32));
            }
        }
    }
    // print_board(&elves);

    let mut moves = vec![Move::North, Move::South, Move::West, Move::East];

    let mut i = 0;
    let mut done = false;
    while !done {
        done = true;
        let mut proposed: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
        for elf in &elves {
            for adjacent in get_adjacent(elf) {
                if elves.contains(&adjacent) {
                    // propose a move.
                    if let Some(next) = propose(elf, &elves, &moves) {
                        proposed.entry(next).or_default().push(*elf);
                    }
                    break;
                }
            }
        }

        // println!("{}: proposed: {:?}", i + 1, proposed);
        for (next, curr) in proposed {
            if curr.len() == 1 {
                // println!("  Moving {:?} to {:?}", curr[0], next);
                done = false;
                elves.remove(&curr[0]);
                elves.insert(next);
            }
        }
        // print_board(&elves);
        i += 1;
        moves.rotate_left(1);
    }

    i
}

//-----------------------------------------------------
// Questions.

q_impl!("23");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            ".....
    ..##.
    ..#..
    .....
    ..##.
    .....
    "
        )),
        25
    );

    assert_eq!(
        process_data_a(indoc!(
            "....#..
    ..###.#
    #...#.#
    .#...##
    #.###..
    ##.#.##
    .#..#..
    "
        )),
        110
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "....#..
    ..###.#
    #...#.#
    .#...##
    #.###..
    ##.#.##
    .#..#..
    "
        )),
        20
    );
}
