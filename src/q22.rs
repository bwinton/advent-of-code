//-----------------------------------------------------
// Setup.

use day;

use std::collections::HashSet;

static INPUT : &'static str = "....##.#.#.#...#.##.##.#.
##.####..###..#.#.#.###.#
.#.#...#.##....#......###
...#.....##.###....##.###
#.########.#.#####..##.#.
.#..#..#.#..#....##.#...#
.....#.##..#.#.....##..##
....###....###....###.#..
..#..#..#..#.##.#.#..##.#
.##......#...##.#.#.##.#.
.#####.#.#.##...###...#..
#..###..#....#....##..#..
###..#....#.##.##.....#..
##.##..#.##.#..#####.#.#.
#....#.######.#.#.#.##.#.
###.##.#.######.#..###.#.
#...###.#.#..##..####....
###...##.###..###..##..#.
..##.###...#.....##.##.##
..##..#.###.###.....#.###
#..###.##.#.###......####
#.#...#..##.###.....##.#.
#..#.##...##.##....#...#.
..#.#..#..#...##.#..###..
......###....#.....#....#";

enum Direction {
  Left,
  Up,
  Right,
  Down
}

struct Carrier {
  position: (i32, i32),
  direction: Direction
}

impl Carrier {
  pub fn new() -> Carrier {
    Carrier {
      position: (0, 0),
      direction: Direction::Up
    }
  }

  fn step(&mut self, grid: &HashSet<(i32, i32)>) -> (HashSet<(i32, i32)>, bool) {
    let mut rv = grid.clone();
    let mut infected = false;
    if grid.contains(&self.position) {
      rv.remove(&self.position);
      self.direction = match self.direction {
        Direction::Left => Direction::Up,
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left
      }
    } else {
      infected = true;
      rv.insert(self.position.clone());
      self.direction = match self.direction {
        Direction::Left => Direction::Down,
        Direction::Up => Direction::Left,
        Direction::Right => Direction::Up,
        Direction::Down => Direction::Right
      }
    }
    self.position = match self.direction {
      Direction::Left => (self.position.0 - 1, self.position.1),
      Direction::Up => (self.position.0, self.position.1 - 1),
      Direction::Right => (self.position.0 + 1, self.position.1),
      Direction::Down => (self.position.0, self.position.1 + 1)
    };
    (rv, infected)
  }
}

fn get_cells(data: &str) -> HashSet<(i32, i32)> {
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

  let mut rv = HashSet::new();
  for (y, line) in grid.iter().enumerate() {
    for (x, cell) in line.iter().enumerate() {
      if *cell {
        rv.insert((x as i32 - mid_x, y as i32 - mid_y));
      }
    }
  }
  rv
}

fn process_data_a(data: &str, iterations: usize) -> usize {
  let mut cells = get_cells(data);
  let mut carrier = Carrier::new();
  // println!("{:?}", cells);
  let mut rv = 0;
  for _ in 0..iterations {
    let temp = carrier.step(&cells);
    cells = temp.0;
    if temp.1 {
      rv += 1;
    }
    // println!("\n{:?}", cells);
  }
  rv
}

fn process_data_b(_data: &str) -> i32 {
  let mut cells = get_cells(data);
  let mut carrier = Carrier::new();
  // println!("{:?}", cells);
  let mut rv = 0;
  for _ in 0..iterations {
    let temp = carrier.step(&cells);
    cells = temp.0;
    if temp.1 {
      rv += 1;
    }
    // println!("\n{:?}", cells);
  }
  rv
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl day::Day for Q {
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
    let result = process_data_b(INPUT);
    println!("Result = {}", result);
  }
}

#[test]
fn a() {
  assert_eq!(process_data_a("..#
#..
...", 7), 5);
assert_eq!(process_data_a("..#
#..
...", 70), 41);
assert_eq!(process_data_a("..#
#..
...", 10_000), 5587);
}

#[test]
fn b() {
  assert_eq!(process_data_b(""), 0);
}
