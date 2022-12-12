//-----------------------------------------------------
// Setup.

use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

static INPUT: &str = include_str!("data/q18.data");

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Board {
    cells: Vec<Vec<char>>,
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Board, ()> {
        let mut cells = vec![];
        for line in s.lines() {
            let mut row = vec![];
            for c in line.chars() {
                row.push(c);
            }
            cells.push(row);
        }
        Ok(Board { cells })
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        for row in &self.cells {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Board {
    fn score(&self) -> i32 {
        let mut trees = 0;
        let mut yards = 0;
        for row in &self.cells {
            for c in row {
                match c {
                    '|' => trees += 1,
                    '#' => yards += 1,
                    _ => {}
                }
            }
        }
        trees * yards
    }

    fn step(&mut self) {
        let mut cells = self.cells.clone();
        for y in 0..self.cells.len() as i32 {
            for x in 0..self.cells[y as usize].len() as i32 {
                let c = self.cells[y as usize][x as usize];
                match c {
                    '.' => {
                        // An open acre will become filled with trees if three or more adjacent acres contained trees.
                        let mut trees = 0;
                        for b in y - 1..=y + 1 {
                            for a in x - 1..=x + 1 {
                                if b >= 0
                                    && b < self.cells.len() as i32
                                    && a >= 0
                                    && a < self.cells[b as usize].len() as i32
                                    && self.cells[b as usize][a as usize] == '|'
                                {
                                    trees += 1;
                                }
                            }
                        }
                        if trees >= 3 {
                            cells[y as usize][x as usize] = '|';
                        }
                    }
                    '|' => {
                        // An acre filled with trees will become a lumberyard if three or more adjacent acres were lumberyards.
                        let mut lumber = 0;
                        for b in y - 1..=y + 1 {
                            for a in x - 1..=x + 1 {
                                if b >= 0
                                    && b < self.cells.len() as i32
                                    && a >= 0
                                    && a < self.cells[b as usize].len() as i32
                                    && self.cells[b as usize][a as usize] == '#'
                                {
                                    lumber += 1;
                                }
                            }
                        }
                        // println!("{},{} l:{}", x, y, lumber);
                        if lumber >= 3 {
                            cells[y as usize][x as usize] = '#';
                        }
                    }
                    '#' => {
                        // An acre containing a lumberyard will remain a lumberyard if it was adjacent to at least
                        // one other lumberyard and at least one acre containing trees. Otherwise, it becomes open.
                        let mut trees = 0;
                        let mut lumber = 0;
                        for b in y - 1..=y + 1 {
                            for a in x - 1..=x + 1 {
                                if b >= 0
                                    && b < self.cells.len() as i32
                                    && a >= 0
                                    && a < self.cells[b as usize].len() as i32
                                {
                                    if a == x && b == y {
                                        continue;
                                    }
                                    if self.cells[b as usize][a as usize] == '#' {
                                        lumber += 1;
                                    } else if self.cells[b as usize][a as usize] == '|' {
                                        trees += 1;
                                    }
                                }
                            }
                        }
                        if lumber < 1 || trees < 1 {
                            cells[y as usize][x as usize] = '.';
                        }
                    }
                    _ => {}
                }
            }
        }
        self.cells = cells;
    }
}

fn process_data_a(data: &str) -> i32 {
    let mut board: Board = data.parse().unwrap();
    for _ in 0..10 {
        board.step();
    }
    board.score()
}

fn process_data_b(data: &str) -> i32 {
    let mut board: Board = data.parse().unwrap();
    let mut seen = HashMap::new();
    let mut i = 1;
    let mut target = 1_000_000_000;
    loop {
        if let Some(prev) = seen.insert(board.clone(), i) {
            let period = i - prev;
            target = ((target - i) % period) + prev;
            for (key, val) in seen.iter() {
                if val == &target {
                    board = key.clone();
                    break;
                }
            }
            break;
        };
        i += 1;
        board.step();
    }
    // Not 210015.
    // Not 194598.
    // 207998
    board.step();
    board.score()
}

//-----------------------------------------------------
// Questions.

q_impl!("18");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(
            ".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|."
        ),
        1147
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b(""), 0);
}
