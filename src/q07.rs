//-----------------------------------------------------
// Setup.

use day;

use regex::Regex;
use std::str::FromStr;

static INPUT : &'static str = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

#[derive(Debug)]
struct Value {
  number: i32,
  wire: String
}

impl FromStr for Value {
  type Err = ();

  fn from_str(s: &str) -> Result<Value, ()> {
    lazy_static! {
      static ref RE: Regex = Regex::new("^([0-9]+) -> ([a-z]+)$").unwrap();
    }

    let captures = RE.captures(s);
    match captures {
      Some(cap) => {
        Ok(Value{
          number: cap.at(1).unwrap().parse().unwrap(),
          wire: cap.at(2).unwrap().parse().unwrap()
        })
      },
      _ => Err(())
    }
  }
}

// #[derive(Debug)]
// enum Gate {
//   And{a: String, b: String},
//   Or{a: String, b: String},
//   LShift{a: String, b: i32},
//   RShift{a: String, b: i32},
//   Not{a: String},
// }
//
// impl FromStr for Gate {
//   type Err = ();
//
//   fn from_str(s: &str) -> Result<Gate, ()> {
//     lazy_static! {
//       static ref VALUE_RE: Regex = Regex::new("([0-9]+) -> ([a-z]+)").unwrap();
//       static ref AND_RE: Regex = Regex::new("([a-z]+) AND ([a-z]+) -> ([a-z]+)").unwrap();
//       static ref OR_RE: Regex = Regex::new("([a-z]+) OR ([a-z]+) -> ([a-z]+)").unwrap();
//       static ref LSHIFT_RE: Regex = Regex::new("([a-z]+) LSHIFT ([0-9]+) -> ([a-z]+)").unwrap();
//       static ref RSHIFT_RE: Regex = Regex::new("([a-z]+) RSHIFT ([0-9]+) -> ([a-z]+)").unwrap();
//       static ref NOT_RE: Regex = Regex::new("NOT ([a-z]+) -> ([a-z]+)").unwrap();
//     }
//
//     let value_captures = VALUE_RE.captures(s);
//     match value_captures {
//       Some(cap) => {
//         return Ok(Cell{
//           id: cap.at(2).unwrap().parse().unwrap(),
//           value: Some(cap.at(1).unwrap().parse().unwrap())
//         });
//       },
//       None => {}
//     }
//
//     let and_captures = AND_RE.captures(s);
//     match and_captures {
//       Some(cap) => {
//         return Ok(Cell{
//           id: cap.at(3).unwrap().parse().unwrap(),
//           value: Gate::And{
//             a: cap.at(1).unwrap().parse().unwrap(),
//             b: cap.at(2).unwrap().parse().unwrap()
//           }
//         });
//       },
//       None => {}
//     }
//
//     return Err(());
//   }
// }


fn process_data_a(data: &str) -> i32 {
  for line in data.lines() {
    let maybe_value: Result<Value, ()> = line.parse();
    match maybe_value {
      Ok(value) => println!("{} => {:?}", line, value),
      Err(()) => {}
    }
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
    String::from("7")
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
