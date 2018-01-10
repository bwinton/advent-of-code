//-----------------------------------------------------
// Setup.

use aoc::Day;

static INPUT : usize = 369;

fn process_data_a(steps: usize) -> usize {
  let mut buffer: Vec<usize> = Vec::with_capacity(2019);
  let mut pos = 0;
  buffer.push(0);
  for i in 1..2018 {
    pos += steps;
    pos %= i;
    pos += 1;
    buffer.insert(pos, i);
  }
  buffer[(pos + 1) % buffer.len()]
}

fn process_data_b(steps: usize) -> usize {
  let mut rv: usize = 0;
  let mut pos = 0;
  for i in 1..50_000_001 {
    pos += steps;
    pos %= i;
    pos += 1;
    if pos == 1 {
      rv = i
    }
  }
  rv
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
  fn number(&self) -> String {
    String::from("17")
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    let result = process_data_a(INPUT);
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
  assert_eq!(process_data_a(3), 638);
}

#[test]
fn b() {
  assert_eq!(process_data_b(0), 1);
}
