//-----------------------------------------------------
// Setup.

use day;

// use itertools::Itertools;

static TEST_INPUT : &'static str = "turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500";
static INPUT : &'static str = "";

fn process_data_a(data: &str) -> i32 {
  let mut state = [[0 as i32; 1000]; 1000];

  for line in data.lines() {
    println!("{}", line);
  }

  state.iter()
    .flat_map(|row| row.iter())
    .fold(0, |sum, i| sum + i)
}

fn process_data_b(_data: &str) -> i32 {
  0
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl day::Day for Q {
  fn number(&self) -> String {
    String::from("6")
  }

  fn a(&self, use_test_data: bool) {
    print!("{}A: ", self.number());
    let result = if use_test_data {
      process_data_a(TEST_INPUT)
    } else {
      process_data_a(INPUT)
    };
    println!("Result = {}", result);
  }

  fn b(&self, use_test_data: bool) {
    print!("{}B: ", self.number());
    let result = if use_test_data {
      process_data_b(TEST_INPUT)
    } else {
      process_data_b(INPUT)
    };
    println!("Result = {}", result);
  }
}
