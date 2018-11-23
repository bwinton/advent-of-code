//-----------------------------------------------------
// Setup.

use aoc::Day;

use regex::Regex;
use std::iter::repeat;
use std::str::FromStr;

// use itertools::Itertools;

static INPUT: &'static str = include_str!("data/q06.data");

type State = Vec<[i32; 1000]>;

#[derive(Debug)]
enum Operation {
  TurnOn,
  TurnOff,
  Toggle,
}

impl FromStr for Operation {
  type Err = ();

  fn from_str(s: &str) -> Result<Operation, ()> {
    match s {
      "turn on" => Ok(Operation::TurnOn),
      "turn off" => Ok(Operation::TurnOff),
      "toggle" => Ok(Operation::Toggle),
      _ => Err(()),
    }
  }
}


#[derive(Debug)]
struct Instruction {
  op: Operation,
  start_x: usize,
  start_y: usize,
  end_x: usize,
  end_y: usize,
}

impl FromStr for Instruction {
  type Err = ();

  fn from_str(s: &str) -> Result<Instruction, ()> {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    }
    let captures = RE.captures(s);
    match captures {
      Some(cap) => {
        Ok(Instruction {
          op: cap[1].parse().unwrap(),
          start_x: cap[2].parse().unwrap(),
          start_y: cap[3].parse().unwrap(),
          end_x: cap[4].parse().unwrap(),
          end_y: cap[5].parse().unwrap(),
        })
      },
      None => Err(()),
    }
  }
}

impl Instruction {
  fn execute_a(&self, state: &mut State) {
    for mut row in state.iter_mut().take(self.end_x + 1).skip(self.start_x) {
      for mut cell in row.iter_mut().take(self.end_y + 1).skip(self.start_y) {
        match self.op {
          Operation::TurnOn => *cell = 1,
          Operation::TurnOff => *cell = 0,
          Operation::Toggle => *cell = 1 - *cell,
        }
      }
    }
  }

  fn execute_b(&self, state: &mut State) {
    for mut row in state.iter_mut().take(self.end_x + 1).skip(self.start_x) {
      for mut cell in row.iter_mut().take(self.end_y + 1).skip(self.start_y) {
        match self.op {
          Operation::TurnOn => *cell += 1,
          Operation::TurnOff => {
            if *cell > 0 {
              *cell -= 1;
            }
          },
          Operation::Toggle => *cell += 2,
        }
      }
    }
  }
}

fn process_data_a(data: &str) -> i32 {
  let mut state: State = repeat([0 as i32; 1000]).take(1000).collect();

  for line in data.lines() {
    let inst: Instruction = line.parse().unwrap();
    inst.execute_a(&mut state);
  }

  state.iter().flat_map(|row| row.iter()).sum()
}

fn process_data_b(data: &str) -> i32 {
  let mut state: State = repeat([0 as i32; 1000]).take(1000).collect();

  for line in data.lines() {
    let inst: Instruction = line.parse().unwrap();
    inst.execute_b(&mut state);
  }
  state.iter().flat_map(|row| row.iter()).sum()
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
  fn number(&self) -> String {
    String::from("6")
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
  assert_eq!(
    process_data_a(
      "turn on 0,0 through 999,999
  toggle 0,0 through 999,0
  turn off 499,499 through 500,500",
    ),
    998996
  );
}

#[test]
fn b() {
  assert_eq!(process_data_b("turn on 0,0 through 0,0"), 1);
  assert_eq!(process_data_b("toggle 0,0 through 999,999"), 2000000);
}
