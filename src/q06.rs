//-----------------------------------------------------
// Setup.

use day;

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
    String::from("6")
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
  assert_eq!(process_data_a(""), 0);
}

#[test]
fn b() {
  assert_eq!(process_data_b(""), 0);
}
