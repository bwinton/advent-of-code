//-----------------------------------------------------
// Setup.

use day;
use std::str::FromStr;

static INPUT : &'static str = "";

#[derive(Debug)]
struct Rule {
  size: usize,
  input: String,
  output: String
}

impl FromStr for Rule {
  type Err = ();

  fn from_str(s: &str) -> Result<Rule, ()> {
    let parts: Vec<_> = s.split(" => ").collect();
    let input = parts[0].to_string();
    Ok(Rule {
      size: if input.len() == 5 { 2 } else { 3 },
      input: input,
      output: parts[1].to_string()
    })
  }
}

// impl Rule {
//   fn matches(&self, s: &str) -> Option<String> {
//     if s == self.input {
//       Some(self.output.clone())
//     } else {
//       None
//     }
//   }
// }
//

fn process_data_a(data: &str, iterations: usize) -> i32 {
  let rules: Vec<Rule> = data.lines().map(|line| line.parse().unwrap()).collect();
  let _initial_state = ".#./..#/###";
  println!("{:?}", rules);
  for _i in 0..iterations {
    // step!
  }
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
    String::from("21")
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    let result = process_data_a(INPUT, 5);
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
  assert_eq!(process_data_a("../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#", 2), 12);
}

#[test]
fn b() {
  assert_eq!(process_data_b(""), 0);
}
