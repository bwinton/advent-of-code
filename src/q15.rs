//-----------------------------------------------------
// Setup.

use day;

static INPUT : &[u64;2] = &[591, 393];

struct Generator {
  value: u64,
  factor: u64,
  multiple: u64
}

impl Generator {
  pub fn new(start: u64, factor: u64, multiple: u64) -> Generator {
    Generator {
      value: start,
      factor: factor,
      multiple: multiple
    }
  }

  fn next(&mut self) -> u16 {
    self.value = self.value.wrapping_mul(self.factor) % 2_147_483_647;
    while self.value % self.multiple != 0 {
      self.value = self.value.wrapping_mul(self.factor) % 2_147_483_647;
    }
    self.value as u16
  }
}

fn process_data_a(data: &[u64;2]) -> u64 {
  let mut a = Generator::new(data[0], 16_807, 1);
  let mut b = Generator::new(data[1], 48_271, 1);
  let mut rv = 0;

  for _ in 0..40_000_000 {
    let next_a = a.next();
    let next_b = b.next();
    if next_a == next_b {
      rv += 1;
    }
  }
  rv
}

fn process_data_b(data: &[u64;2]) -> u64 {
  let mut a = Generator::new(data[0], 16_807, 4);
  let mut b = Generator::new(data[1], 48_271, 8);
  let mut rv = 0;

  for _ in 0..5_000_000 {
    let next_a = a.next();
    let next_b = b.next();
    if next_a == next_b {
      rv += 1;
    }
  }
  rv
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl day::Day for Q {
  fn number(&self) -> String {
    String::from("15")
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
  assert_eq!(process_data_a(&[65, 8921]), 588);
}

#[test]
fn b() {
  assert_eq!(process_data_b(&[65, 8921]), 309);
}
