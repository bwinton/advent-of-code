//-----------------------------------------------------
// Setup.

use day;

// static INPUT_NUMBER : i32 = 10;
// static INPUT_TARGET_X : i32 = 7;
// static INPUT_TARGET_Y : i32 = 4;
static INPUT_NUMBER : i32 = 1362;
static INPUT_TARGET_X : i32 = 31;
static INPUT_TARGET_Y : i32 = 39;

fn get_cell(x: i32, y: i32) -> bool {
  let mut number = x*x + 3*x + 2*x*y + y + y*y + INPUT_NUMBER;
  // print!("{} => ", number);
  let mut count = 0;
  while number != 0 {
    number = number & (number-1);
    // print!("{}/{} => ", number, count);
    count += 1;
  }
  // println!("{}, {}", count, count % 2 == 1);
  return count % 2 == 0;
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl day::Day for Q {
  fn number(&self) -> String {
    return String::from("13");
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    println!("");
    let result = 0;

    for y in 0..INPUT_TARGET_Y {
      for x in 0..INPUT_TARGET_X {
        if get_cell(x,y) {
          print!(".");
        } else {
          print!("#");
        }
      }
      println!("");
    }

    println!("Result = {}", result);
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    let result = 0;
    println!("Result = {}", result);
  }
}
