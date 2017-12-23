//-----------------------------------------------------
// Setup.

use day;

static INPUT : &'static str = "";

// enum Direction {
//   Left,
//   Up,
//   Right,
//   Down
// }

struct Carrier {
  // position: (i32, i32),
  // direction: Direction
}

impl Carrier {
  pub fn new(_grid: &[Vec<bool>]) -> Carrier {
    Carrier {
      // position: (
      //   ((grid.len() + 1) / 2) as i32,
      //   ((grid[0].len() + 1) / 2) as i32
      // ),
      // direction: Direction::Up
    }
  }

  fn step(&self, grid: &[Vec<bool>]) -> Vec<Vec<bool>>{
    // pass
    let rv: Vec<Vec<bool>> = grid.to_vec();
    rv
  }
}

fn process_data_a(data: &str, iterations: usize) -> i32 {
  let mut grid: Vec<Vec<bool>> = Vec::new();
  for line in data.lines() {
    let mut row = Vec::new();
    for cell in line.chars() {
      row.push(cell == '#');
    }
    grid.push(row);
  }
  let carrier = Carrier::new(&grid);
  println!("{:?}", grid);
  for _ in 0..iterations {
    grid = carrier.step(&grid);
  }
  0
}

fn process_data_b(_data: &str) -> i32 {
  0
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
