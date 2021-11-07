//-----------------------------------------------------
// Setup.

use std::{fmt, str::FromStr};

static INPUT: &str = include_str!("data/q18.data");

#[derive(Clone, Debug)]
struct Board {
    cells: Vec<Vec<bool>>,
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

        Ok(Board { cells: grid })
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.cells {
            for cell in line.iter() {
                write!(f, "{}", if *cell { '#' } else { '.' }).expect("could not write cell");
            }
            writeln!(f).expect("could not write newline");
        }
        Ok(())
    }
}

impl Board {
    fn get_cell(&self, x: usize, y: usize) -> bool {
        let mut neighbours = 0;
        for i in 0..3 {
            for j in 0..3 {
                let new_x = (x as i32) + i - 1;
                let new_y = (y as i32) + j - 1;
                if new_x >= 0
                    && new_x < self.cells.len() as i32
                    && new_y >= 0
                    && new_y < self.cells[new_x as usize].len() as i32
                    && self.cells[new_x as usize][new_y as usize]
                {
                    neighbours += 1;
                }
            }
        }
        if self.cells[x][y] {
            neighbours == 3 || neighbours == 4
        } else {
            neighbours == 3
        }
    }

    fn step(&self) -> Board {
        let mut rv = self.clone();
        for (x, line) in self.cells.iter().enumerate() {
            for (y, _) in line.iter().enumerate() {
                rv.cells[x][y] = self.get_cell(x, y);
            }
        }
        rv
    }

    fn count_on_cells(&self) -> u32 {
        self.cells
            .iter()
            .flat_map(|line| line.iter())
            .fold(0, |acc, &x| acc + (x as u32))
    }

    fn light_corners(&mut self) {
        let len = &self.cells.len() - 1;
        self.cells[0][0] = true;
        self.cells[0][len] = true;
        self.cells[len][0] = true;
        self.cells[len][len] = true;
    }
}

fn process_data_a_impl(data: &str, iterations: usize) -> u32 {
    let mut board: Board = data.parse().unwrap();

    for _ in 0..iterations {
        board = board.step();
    }
    board.count_on_cells()
}

fn process_data_b_impl(data: &str, iterations: usize) -> u32 {
    let mut board: Board = data.parse().unwrap();

    for _ in 0..iterations {
        board.light_corners();
        // println!("{}", board);
        board = board.step();
    }
    board.light_corners();
    // println!("{}", board);
    board.count_on_cells()
}

fn process_data_a(data: &str) -> u32 {
    process_data_a_impl(data, 100)
}

fn process_data_b(data: &str) -> u32 {
    process_data_b_impl(data, 100)
}

//-----------------------------------------------------
// Questions.

q_impl!("18");

#[test]
fn a() {
    assert_eq!(
        process_data_a_impl(
            ".#.#.#
...##.
#....#
..#...
#.#..#
####..",
            4,
        ),
        4
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b_impl(
            ".#.#.#
...##.
#....#
..#...
#.#..#
####..",
            5,
        ),
        17
    );
}
