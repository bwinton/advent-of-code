//-----------------------------------------------------
// Setup.

use aoc::Day;

static INPUT: &'static str = include_str!("data/q01.data");

fn process_chars_a(data: &str) -> i32 {
  let mut rv = 0;
  for character in data.chars() {
    match character {
      '(' => rv += 1,
      ')' => rv -= 1,
      _ => panic!("Invalid Character \"{}\"", character),
    }
  }
  rv
}

fn process_chars_b(data: &str) -> Option<usize> {
  let mut rv = 0;
  for (i, character) in data.chars().enumerate() {
    match character {
      '(' => rv += 1,
      ')' => rv -= 1,
      _ => panic!("Invalid Character \"{}\"", character),
    }
    if rv < 0 {
      return Some(i + 1);
    }
  }
  None
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
  fn number(&self) -> String {
    String::from("1")
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    let result = process_chars_a(INPUT);
    println!("Result = {}", result);
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    let result = process_chars_b(INPUT);
    println!("Result = {}", result.unwrap());
  }
}

#[test]
fn a() {
  assert_eq!(process_chars_a("(())"), 0);
  assert_eq!(process_chars_a("()()"), 0);
  assert_eq!(process_chars_a("((("), 3);
  assert_eq!(process_chars_a("(()(()("), 3);
  assert_eq!(process_chars_a("))((((("), 3);
  assert_eq!(process_chars_a("())"), -1);
  assert_eq!(process_chars_a("))("), -1);
  assert_eq!(process_chars_a(")())())"), -3);
}

#[test]
fn b() {
  assert_eq!(process_chars_b(")"), Some(1));
  assert_eq!(process_chars_b("()())"), Some(5));
}
