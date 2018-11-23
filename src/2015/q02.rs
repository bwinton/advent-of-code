//-----------------------------------------------------
// Setup.

use aoc::Day;

static INPUT: &'static str = include_str!("data/q02.data");

fn get_pieces(line: &str) -> Vec<u32> {
  let mut rv: Vec<u32> = line.split('x').map(|i| i.parse().unwrap()).collect();
  rv.sort_unstable();
  rv
}

fn process_lines_a(data: &str) -> u32 {
  let mut rv = 0;
  for line in data.lines() {
    let pieces = get_pieces(line);
    let size = 3 * pieces[0] * pieces[1] + 2 * pieces[0] * pieces[2] + 2 * pieces[1] * pieces[2];
    // println!("{:?} => {}", pieces, size);
    rv += size;
  }
  rv
}

fn process_lines_b(data: &str) -> u32 {
  let mut rv = 0;
  for line in data.lines() {
    let pieces = get_pieces(line);
    let size = 2 * pieces[0] + 2 * pieces[1] + pieces[0] * pieces[1] * pieces[2];
    // println!("{:?} => {}", pieces, size);
    rv += size;
  }
  rv
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
  fn number(&self) -> String {
    String::from("2")
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    let result = process_lines_a(INPUT);
    println!("Result = {}", result);
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    let result = process_lines_b(INPUT);
    println!("Result = {}", result);
  }
}

#[test]
fn a() {
  assert_eq!(process_lines_a("2x3x4"), 58);
  assert_eq!(process_lines_a("1x1x10"), 43);
}

#[test]
fn b() {
  assert_eq!(process_lines_b("2x3x4"), 34);
  assert_eq!(process_lines_b("1x1x10"), 14);
}
