//-----------------------------------------------------
// Setup.

use day;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::i32::MAX;

static TEST_INPUT : &'static str = "abcdef";
// static TEST_INPUT : &'static str = "pqrstuv";
static INPUT : &'static str = "bgvyzdsv";

fn process_data(data: &str, zeroes: usize) -> i32 {
  let mut hasher = Md5::new();
  let input = data.as_bytes();

  for i in 0..MAX {
    hasher.input(input);
    hasher.input(i.to_string().as_bytes());

    let mut output = [0; 16]; // An MD5 is 16 bytes
    hasher.result(&mut output);
    let mut first = 0;
    for value in output.iter().take(zeroes/2) {
      first += i32::from(*value);
    }
    if zeroes % 2 == 1 {
      first += i32::from(output[zeroes/2] >> 4);
    }
    if first == 0 {
      return i
    }
    hasher.reset();
  }
  -1
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
      process_data(TEST_INPUT, 5)
    } else {
      process_data(INPUT, 5)
    };
    println!("Result = {}", result);
  }

  fn b(&self, use_test_data: bool) {
    print!("{}B: ", self.number());
    let result = if use_test_data {
      process_data(TEST_INPUT, 6)
    } else {
      process_data(INPUT, 6)
    };
    println!("Result = {}", result);
  }
}
