//-----------------------------------------------------
// Setup.

use aoc::Day;

use crypto::digest::Digest;
use crypto::md5::Md5;
use std::i32::MAX;

static INPUT: &'static str = "bgvyzdsv";

fn process_data(data: &str, zeroes: usize) -> i32 {
  let mut hasher = Md5::new();
  let input = data.as_bytes();

  for i in 0..MAX {
    hasher.input(input);
    hasher.input(i.to_string().as_bytes());

    let mut output = [0; 16]; // An MD5 is 16 bytes
    hasher.result(&mut output);
    let mut first = 0;
    for value in output.iter().take(zeroes / 2) {
      first += i32::from(*value);
    }
    if zeroes % 2 == 1 {
      first += i32::from(output[zeroes / 2] >> 4);
    }
    if first == 0 {
      return i;
    }
    hasher.reset();
  }
  -1
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
  fn number(&self) -> String {
    String::from("4")
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    let result = process_data(INPUT, 5);
    println!("Result = {}", result);
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    let result = process_data(INPUT, 6);
    println!("Result = {}", result);
  }
}

#[test]
fn a() {
  assert_eq!(process_data("abcdef", 5), 609043);
  assert_eq!(process_data("pqrstuv", 5), 1048970);
}

#[test]
fn b() {
  assert_eq!(process_data("abcdef", 6), 6742839);
  assert_eq!(process_data("pqrstuv", 6), 5714438);
}
