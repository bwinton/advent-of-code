//-----------------------------------------------------
// Setup.

use day;

// static INPUT : &'static str = "R2, L3";
// static INPUT : &'static str = "R8, R4, R4, R8";


//-----------------------------------------------------
// Questions.

pub struct Q;

impl day::Day for Q {
  fn number(&self) -> String {
    return String::from("12");
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
