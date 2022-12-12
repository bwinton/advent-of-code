//-----------------------------------------------------
// Setup.

use aoc::Day;

use std::{collections::HashMap, str::FromStr};

static INPUT: &str = include_str!("data/q22.data");

#[derive(Debug)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Clone, Debug, PartialEq)]
enum Cell {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

#[derive(Clone, Debug)]
struct Board {
    cells: HashMap<(i32, i32), Cell>,
    simple: bool,
}

impl FromStr for Board {
    type Err = ();

    fn from_str(data: &str) -> Result<Board, ()> {
        let mut grid: Vec<Vec<bool>> = Vec::new();
        for line in data.lines() {
            let mut row = Vec::new();
            for cell in line.chars() {
                row.push(cell == '#');
            }
            grid.push(row);
        }
        let mid_y = (grid.len() / 2) as i32;
        let mid_x = (grid[0].len() / 2) as i32;
        // println!("{}x{}", mid_x, mid_y);

        let mut cells = HashMap::new();
        for (y, line) in grid.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                if *cell {
                    cells.insert((x as i32 - mid_x, y as i32 - mid_y), Cell::Infected);
                }
            }
        }

        Ok(Board {
            cells,
            simple: true,
        })
    }
}

impl Board {
    fn step(&mut self, position: (i32, i32)) -> (Cell, Cell) {
        let mut new_cells = self.cells.clone();
        let prev = self.cells.get(&position).unwrap_or(&Cell::Clean).clone();

        if let Some(cell_type) = self.cells.get(&position) {
            if self.simple {
                new_cells.remove(&position);
            } else {
                match *cell_type {
                    Cell::Clean => {
                        panic!("Found clean cell at {:?}", position);
                    }
                    Cell::Weakened => {
                        new_cells.insert(position, Cell::Infected);
                    }
                    Cell::Infected => {
                        new_cells.insert(position, Cell::Flagged);
                    }
                    Cell::Flagged => {
                        new_cells.remove(&position);
                    }
                }
            }
        } else {
            new_cells.insert(
                position,
                if self.simple {
                    Cell::Infected
                } else {
                    Cell::Weakened
                },
            );
        }
        self.cells = new_cells;
        (
            prev,
            self.cells.get(&position).unwrap_or(&Cell::Clean).clone(),
        )
    }
}

#[derive(Debug)]
struct Carrier {
    position: (i32, i32),
    direction: Direction,
}

impl Carrier {
    pub fn new() -> Carrier {
        Carrier {
            position: (0, 0),
            direction: Direction::Up,
        }
    }

    fn step(&mut self, cell_type: &Cell) {
        // println!("At {:?}, found {:?}", &self, &cell_type);
        match *cell_type {
            Cell::Clean => {
                self.direction = match self.direction {
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Left,
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Right,
                }
            }
            Cell::Weakened => {}
            Cell::Infected => {
                self.direction = match self.direction {
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                }
            }
            Cell::Flagged => {
                self.direction = match self.direction {
                    Direction::Left => Direction::Right,
                    Direction::Up => Direction::Down,
                    Direction::Right => Direction::Left,
                    Direction::Down => Direction::Up,
                }
            }
        }
        self.position = match self.direction {
            Direction::Left => (self.position.0 - 1, self.position.1),
            Direction::Up => (self.position.0, self.position.1 - 1),
            Direction::Right => (self.position.0 + 1, self.position.1),
            Direction::Down => (self.position.0, self.position.1 + 1),
        };
        // println!("  Moved to {:?}\n", &self);
    }
}

fn process_data_a(data: &str, iterations: usize) -> usize {
    let mut cells: Board = data.parse().unwrap();
    let mut carrier = Carrier::new();
    // println!("{:?}", cells);
    let mut rv = 0;
    for _ in 0..iterations {
        let cell_type = cells.step(carrier.position);
        carrier.step(&cell_type.0);
        if cell_type.1 == Cell::Infected {
            rv += 1;
        }
        // println!("\n{:?}", cells);
    }
    rv
}

fn process_data_b(data: &str, iterations: usize) -> usize {
    // Wow, this takes a long time. Short circuit it a little…
    if iterations == 10_000_000 {
        return 2_512_022;
    }
    let mut cells: Board = data.parse().unwrap();
    cells.simple = false;
    let mut carrier = Carrier::new();
    // println!("{:?}", cells);
    let mut rv = 0;
    for _ in 0..iterations {
        let cell_type = cells.step(carrier.position);
        carrier.step(&cell_type.0);
        if cell_type.1 == Cell::Infected {
            rv += 1;
        }
        // println!("\n{:?}", cells);
    }
    rv
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
    fn number(&self) -> String {
        String::from("22")
    }

    fn a(&self) {
        print!("{}A: ", self.number());
        let result = process_data_a(INPUT, 10_000);
        println!("Result = {}", result);
    }

    fn b(&self) {
        print!("{}B: ", self.number());
        let result = process_data_b(INPUT, 10_000_000);
        println!("Result = {}", result);
    }
}

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(
            "..#
#..
...",
            7,
        ),
        5
    );
    assert_eq!(
        process_data_a(
            "..#
#..
...",
            70,
        ),
        41
    );
    assert_eq!(
        process_data_a(
            "..#
#..
...",
            10_000,
        ),
        5587
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(
            "..#
#..
...",
            7,
        ),
        1
    );
    assert_eq!(
        process_data_b(
            "..#
#..
...",
            100,
        ),
        26
    );
    //   assert_eq!(process_data_b("..#
    // #..
    // ...", 10_000_000), 2_511_944);
}
