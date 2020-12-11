//-----------------------------------------------------
// Setup.

use enumset::EnumSet;

static INPUT: &str = include_str!("data/q11.data");

#[derive(Clone, Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Occupied,
    Floor,
}

impl Cell {
    fn parse(c: char) -> Option<Cell> {
        match c {
            'L' => Some(Cell::Empty),
            '#' => Some(Cell::Occupied),
            '.' => Some(Cell::Floor),
            _ => None,
        }
    }
}

#[derive(EnumSetType, Debug)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

fn surrounding(field: &[Vec<Cell>], i: usize, j: usize, horizon: usize) -> usize {
    let mut rv = 0;
    let i = i as i32;
    let j = j as i32;
    for direction in EnumSet::all().iter() {
        let mut distance = 0;
        let (x_d, y_d) = match direction {
            Direction::North => (-1, 0),
            Direction::NorthEast => (-1, 1),
            Direction::East => (0, 1),
            Direction::SouthEast => (1, 1),
            Direction::South => (1, 0),
            Direction::SouthWest => (1, -1),
            Direction::West => (0, -1),
            Direction::NorthWest => (-1, -1),
        };
        let (mut x, mut y) = (i, j);
        while distance < horizon {
            distance += 1;
            x += x_d;
            y += y_d;
            if x < 0 || y < 0 || x as usize >= field.len() || y as usize >= field[0].len() {
                break;
            }
            if field[x as usize][y as usize] == Cell::Floor {
                continue;
            } else if field[x as usize][y as usize] == Cell::Occupied {
                rv += 1;
            }
            break;
        }
    }
    rv
}

fn run_one(field: &[Vec<Cell>], horizon: usize, occupied: usize) -> Vec<Vec<Cell>> {
    let mut rv = vec![];
    for (i, row) in field.iter().enumerate() {
        let mut curr = vec![];
        for (j, cell) in row.iter().enumerate() {
            curr.push(match *cell {
                Cell::Floor => Cell::Floor,
                Cell::Empty => {
                    // if there are no occupied seats adjacent to it, the seat becomes occupied.
                    if surrounding(&field, i, j, horizon) == 0 {
                        Cell::Occupied
                    } else {
                        Cell::Empty
                    }
                }
                Cell::Occupied => {
                    // if some seats adjacent to it are also occupied, the seat becomes empty.
                    if surrounding(&field, i, j, horizon) >= occupied {
                        Cell::Empty
                    } else {
                        Cell::Occupied
                    }
                }
            });
        }
        rv.push(curr);
    }
    rv
}
fn occupied(field: &[Vec<Cell>]) -> usize {
    let mut rv = 0;
    for row in field {
        for cell in row {
            if *cell == Cell::Occupied {
                rv += 1;
            }
        }
    }
    rv
}

fn process_data_a(data: &str) -> usize {
    let mut field = vec![];
    for line in data.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(Cell::parse(c).unwrap());
        }
        field.push(row);
    }
    loop {
        let next = run_one(&field, 1, 4);
        if next == field {
            return occupied(&next);
        }
        field = next;
    }
}

fn process_data_b(data: &str) -> usize {
    let mut field = vec![];
    for line in data.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(Cell::parse(c).unwrap());
        }
        field.push(row);
    }
    let horizon = field.len();
    loop {
        let next = run_one(&field, horizon, 5);
        if next == field {
            return occupied(&next);
        }
        field = next;
    }
}

//-----------------------------------------------------
// Questions.

q_impl!("11");

#[test]
fn a() {
    assert_eq!(
        process_data_a(
            "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
"
        ),
        37
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(
            "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
"
        ),
        26
    );
}
