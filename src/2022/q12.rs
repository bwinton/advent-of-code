//-----------------------------------------------------
// Setup.

use std::collections::{BinaryHeap, HashSet};

use enumset::{EnumSet, EnumSetType};

static INPUT: &str = include_str!("data/q12.data");

#[derive(Debug, EnumSetType)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn next(&self, curr: &Location, grid: &[Vec<u8>]) -> Option<Location> {
        match self {
            Direction::North => {
                if curr.1 > 0 {
                    Some((curr.0, curr.1 - 1))
                } else {
                    None
                }
            }
            Direction::East => {
                if curr.0 < grid[curr.1].len() - 1 {
                    Some((curr.0 + 1, curr.1))
                } else {
                    None
                }
            }
            Direction::South => {
                if curr.1 < grid.len() - 1 {
                    Some((curr.0, curr.1 + 1))
                } else {
                    None
                }
            }
            Direction::West => {
                if curr.0 > 0 {
                    Some((curr.0 - 1, curr.1))
                } else {
                    None
                }
            }
        }
    }
}

type Location = (usize, usize);

#[derive(Debug, Default, PartialEq, Eq)]
struct State {
    moves: usize,
    location: Location,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_moves = -(self.moves as isize);
        let other_moves = -(other.moves as isize);
        match self_moves.cmp(&other_moves) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.location.cmp(&other.location)
    }
}

fn get_grid(data: &str) -> (Vec<Vec<u8>>, Location, Location) {
    let mut grid = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (y, line) in data.lines().enumerate() {
        let mut row = vec![];
        for (x, cell) in line.chars().enumerate() {
            if cell == 'S' {
                start = (x, y);
                row.push(b'a');
            } else if cell == 'E' {
                end = (x, y);
                row.push(b'z');
            } else {
                row.push(cell as u8);
            }
        }
        grid.push(row);
    }
    (grid, start, end)
}

fn find_shortest_path(end: Location, starts: &[Location], grid: &[Vec<u8>]) -> usize {
    let directions: EnumSet<Direction> = EnumSet::all();
    let mut stack: BinaryHeap<State> = BinaryHeap::new();
    let mut next = Some(State {
        moves: 0,
        location: end,
    });
    let mut seen = HashSet::new();
    seen.insert(end);
    let mut best = usize::MAX;
    'outer: while next.is_some() {
        let curr = next.unwrap();
        seen.insert(curr.location);
        let cell = grid[curr.location.1][curr.location.0];
        for direction in directions {
            if let Some(prev_location) = direction.next(&curr.location, grid) {
                let next_state = State {
                    moves: curr.moves + 1,
                    location: prev_location,
                };
                let prev_cell = grid[prev_location.1][prev_location.0];
                if prev_cell + 1 >= cell {
                    if seen.contains(&prev_location) {
                        continue;
                    }
                    seen.insert(prev_location);
                    if starts.contains(&prev_location) {
                        best = curr.moves + 1;
                        break 'outer;
                    }
                    stack.push(next_state);
                }
            }
        }

        next = stack.pop();
    }
    best
}

fn process_data_a(data: &str) -> usize {
    let (grid, start, end) = get_grid(data);

    find_shortest_path(end, &[start], &grid)
}

fn process_data_b(data: &str) -> usize {
    let (grid, _, end) = get_grid(data);

    let mut starts = vec![];
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == b'a' {
                starts.push((x, y));
            }
        }
    }

    // Run this backwards instead…
    find_shortest_path(end, &starts, &grid)
}

//-----------------------------------------------------
// Questions.

q_impl!("12");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi"
        )),
        31
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi"
        )),
        29
    );
}
