//-----------------------------------------------------
// Setup.

use aoc::Day;

use itertools::Itertools;

static INPUT: &'static str = "3113322113";

fn process_data(data: &str, iterations: usize) -> String {
  let mut curr = data.to_string();
  for _ in 0..iterations {
    // let mut next =
    let input = curr.clone();
    let iter = input
      .chars()
      .map(|x| x.to_digit(10).unwrap())
      .zip([1 as u32].iter().cycle())
      .map(|x| vec![*x.1, x.0])
      .coalesce(|x, y| {
        if x[1] == y[1] {
          Ok(vec![x[0] + y[0], x[1]])
        } else {
          Err((x, y))
        }
      });
    let values: Vec<String> = Itertools::flatten(iter).map(|x| x.to_string()).collect();
    curr = values.join("");
    // println!("  {}: {:?}", i, curr);
  }
  curr
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
  fn number(&self) -> String {
    String::from("10")
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    let result = process_data(INPUT, 40);
    println!("Result = {}", result.len());
  }

  fn b(&self) {
    print!("{}A: ", self.number());
    let result = process_data(INPUT, 50);
    println!("Result = {}", result.len());
  }
}

#[test]
fn a() {
  assert_eq!(process_data("1", 1), "11");
  assert_eq!(process_data("1", 2), "21");
  assert_eq!(process_data("1", 3), "1211");
  assert_eq!(process_data("1", 4), "111221");
  assert_eq!(process_data("1", 5), "312211");
}

#[test]
fn b() {}
