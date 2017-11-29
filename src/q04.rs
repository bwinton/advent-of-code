//-----------------------------------------------------
// Setup.

use day;

static TEST_INPUT : &'static str = "";
static INPUT : &'static str = "";

fn process_data_a(_data: &str) -> i32 {
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
    String::from("4")
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
