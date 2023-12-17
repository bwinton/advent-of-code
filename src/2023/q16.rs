//-----------------------------------------------------
// Setup.

use std::collections::{HashMap, HashSet};

use aoc::util::Direction;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

static INPUT: &str = include_str!("data/q16.data");

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
enum Cell {
    MirrorRight,
    MirrorLeft,
    SplitterHorizontal,
    SplitterVertical,
}

fn parse(data: &str) -> (HashMap<(usize, usize), Cell>, (usize, usize)) {
    let mut board = HashMap::new();
    let mut max = (0, data.lines().count());
    for (y, line) in data.lines().enumerate() {
        if max.0 == 0 {
            max.0 = line.chars().count();
        }
        for (x, cell) in line.chars().enumerate() {
            if let Some(cell) = match cell {
                '/' => Some(Cell::MirrorRight),
                '\\' => Some(Cell::MirrorLeft),
                '-' => Some(Cell::SplitterHorizontal),
                '|' => Some(Cell::SplitterVertical),
                '.' => None,
                _ => {
                    panic!("Invalid character! {}", cell);
                }
            } {
                board.insert((x, y), cell);
            }
        }
    }
    (board, max)
}

fn get_energized_count(
    mut curr: Vec<((usize, usize), Direction)>,
    board: &HashMap<(usize, usize), Cell>,
    max: (usize, usize),
) -> usize {
    let mut energized = HashSet::new();
    let mut seen = HashSet::new();
    while !curr.is_empty() {
        let mut next = vec![];
        for pos in curr {
            if seen.contains(&pos) {
                // We've been here before!
                continue;
            }
            seen.insert(pos);
            energized.insert(pos.0);
            match (board.get(&pos.0), pos.1) {
                (None, _)
                | (Some(Cell::SplitterVertical), Direction::North)
                | (Some(Cell::SplitterVertical), Direction::South)
                | (Some(Cell::SplitterHorizontal), Direction::East)
                | (Some(Cell::SplitterHorizontal), Direction::West) => {
                    if let Some(cell) = pos.1.move_pos(pos.0, max) {
                        next.push((cell, pos.1));
                    }
                }
                (Some(Cell::SplitterHorizontal), _) => {
                    if let Some(cell) = Direction::East.move_pos(pos.0, max) {
                        next.push((cell, Direction::East));
                    };
                    if let Some(cell) = Direction::West.move_pos(pos.0, max) {
                        next.push((cell, Direction::West));
                    };
                }
                (Some(Cell::SplitterVertical), _) => {
                    if let Some(cell) = Direction::North.move_pos(pos.0, max) {
                        next.push((cell, Direction::North));
                    };
                    if let Some(cell) = Direction::South.move_pos(pos.0, max) {
                        next.push((cell, Direction::South));
                    };
                }
                (Some(Cell::MirrorRight), Direction::East)
                | (Some(Cell::MirrorLeft), Direction::West) => {
                    if let Some(cell) = Direction::North.move_pos(pos.0, max) {
                        next.push((cell, Direction::North));
                    };
                }
                (Some(Cell::MirrorRight), Direction::North)
                | (Some(Cell::MirrorLeft), Direction::South) => {
                    if let Some(cell) = Direction::East.move_pos(pos.0, max) {
                        next.push((cell, Direction::East));
                    };
                }
                (Some(Cell::MirrorRight), Direction::West)
                | (Some(Cell::MirrorLeft), Direction::East) => {
                    if let Some(cell) = Direction::South.move_pos(pos.0, max) {
                        next.push((cell, Direction::South));
                    };
                }
                (Some(Cell::MirrorRight), Direction::South)
                | (Some(Cell::MirrorLeft), Direction::North) => {
                    if let Some(cell) = Direction::West.move_pos(pos.0, max) {
                        next.push((cell, Direction::West));
                    };
                }
            };
        }
        curr = next;
    }
    energized.len()
}

fn process_data_a(data: &str) -> usize {
    let (board, max) = parse(data);

    get_energized_count(vec![((0, 0), Direction::East)], &board, max)
}

fn process_data_b(data: &str) -> usize {
    let (board, max) = parse(data);

    let mut starts = vec![];
    for x in 0..max.0 {
        starts.push(vec![((x, 0), Direction::South)]);
        starts.push(vec![((x, max.1 - 1), Direction::North)]);
    }
    for y in 0..max.1 {
        starts.push(vec![((0, y), Direction::East)]);
        starts.push(vec![((max.0 - 1, y), Direction::West)]);
    }
    starts
        .into_par_iter()
        .map(|curr| get_energized_count(curr, &board, max))
        .max()
        .unwrap()
}

//-----------------------------------------------------
// Questions.

q_impl!("16");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            r"
    .|...\....
    |.-.\.....
    .....|-...
    ........|.
    ..........
    .........\
    ..../.\\..
    .-.-/..|..
    .|....-|.\
    ..//.|....
    "
        )),
        46
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            r"
    .|...\....
    |.-.\.....
    .....|-...
    ........|.
    ..........
    .........\
    ..../.\\..
    .-.-/..|..
    .|....-|.\
    ..//.|....
"
        )),
        51
    );
}
