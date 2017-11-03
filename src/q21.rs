//-----------------------------------------------------
// Setup.

use day;

static INPUT : &'static str = "";
// static INPUT : &'static str = "";

//-----------------------------------------------------
// Questions.

pub struct Q;

impl day::Day for Q {
  fn number(&self) -> String {
    return String::from("21");
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    let result = 0;
    println!("Result = {}", result);
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    let result = 0;
    println!("Result = {}", result);
  }
}
