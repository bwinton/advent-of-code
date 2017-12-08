//-----------------------------------------------------
// Setup.

use day;

static INPUT : &'static str = "3113322113";

fn process_data_a(data: &str, iterations: usize) -> &str {
  "0"
}

fn process_data_b(_data: &str) -> i32 {
  0
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl day::Day for Q {
  fn number(&self) -> String {
    String::from("10")
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    let result = process_data_a(INPUT, 40);
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
  assert_eq!(process_data_a("1", 1), "11");
  assert_eq!(process_data_a("1", 2), "21");
  assert_eq!(process_data_a("1", 3), "1211");
  assert_eq!(process_data_a("1", 4), "111221");
  assert_eq!(process_data_a("1", 5), "312211");
}

#[test]
fn b() {
  assert_eq!(process_data_b(""), 0);
}
