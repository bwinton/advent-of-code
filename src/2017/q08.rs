//-----------------------------------------------------
// Setup.

use aoc::Day;

use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

static INPUT: &'static str = include_str!("data/q08.data");

#[derive(Clone, Debug)]
enum Operation {
  Inc(i32),
  Dec(i32),
}

impl FromStr for Operation {
  type Err = ();

  fn from_str(s: &str) -> Result<Operation, ()> {
    lazy_static! {
      static ref MAIN_RE: Regex = Regex::new(r"^(inc|dec) (-?\d+)?$").unwrap();
    }
    let cap = MAIN_RE.captures(s);
    match cap {
      None => Err(()),
      Some(x) => {
        let value = x[2].parse().unwrap();
        match &x[1] {
          "inc" => Ok(Operation::Inc(value)),
          "dec" => Ok(Operation::Dec(value)),
          _ => {
            println!("Unknown Operation {}", s);
            Err(())
          }
        }
      }
    }
  }
}

#[derive(Clone, Debug)]
enum Test {
  Greater,
  GreaterEqual,
  Equal,
  NotEqual,
  LessEqual,
  Less,
}

impl FromStr for Test {
  type Err = ();

  fn from_str(s: &str) -> Result<Test, ()> {
    match s {
      ">" => Ok(Test::Greater),
      ">=" => Ok(Test::GreaterEqual),
      "==" => Ok(Test::Equal),
      "!=" => Ok(Test::NotEqual),
      "<=" => Ok(Test::LessEqual),
      "<" => Ok(Test::Less),
      _ => {
        println!("Unknown Test {}", s);
        Err(())
      }
    }
  }
}

#[derive(Clone, Debug)]
struct Condition {
  source: String,
  test: Test,
  value: i32,
}

impl FromStr for Condition {
  type Err = ();

  fn from_str(s: &str) -> Result<Condition, ()> {
    lazy_static! {
      static ref MAIN_RE: Regex = Regex::new(r"^([a-z]+) ([!<>=]=?) (-?\d+)$").unwrap();
    }
    let cap = MAIN_RE.captures(s);
    match cap {
      None => {
        println!("Unknown Condition {}", s);
        Err(())
      }
      Some(x) => Ok(Condition {
        source: x[1].to_string(),
        test: x[2].parse().unwrap(),
        value: x[3].parse().unwrap(),
      }),
    }
  }
}

impl Condition {
  fn evaluate(&self, regs: &mut HashMap<String, i32>) -> bool {
    let reg = regs.entry(self.source.clone()).or_insert(0);
    match self.test {
      Test::Greater => *reg > self.value,
      Test::GreaterEqual => *reg >= self.value,
      Test::Equal => *reg == self.value,
      Test::NotEqual => *reg != self.value,
      Test::LessEqual => *reg <= self.value,
      Test::Less => *reg < self.value,
    }
  }
}

#[derive(Clone, Debug)]
struct Instruction {
  dest: String,
  op: Operation,
  cond: Condition,
}

impl FromStr for Instruction {
  type Err = ();

  fn from_str(s: &str) -> Result<Instruction, ()> {
    lazy_static! {
      static ref MAIN_RE: Regex =
        Regex::new(r"^([a-z]+) ([a-z]+ -?\d+) if (([a-z]+) ..? -?\d+)$").unwrap();
    }
    let cap = MAIN_RE.captures(s);
    match cap {
      None => {
        println!("Unknown Instruction {}", s);
        Err(())
      }
      Some(x) => Ok(Instruction {
        dest: x[1].to_string(),
        op: x[2].parse().unwrap(),
        cond: x[3].parse().unwrap(),
      }),
    }
  }
}

impl Instruction {
  fn execute(&self, regs: &mut HashMap<String, i32>) -> Option<(String, i32)> {
    if self.cond.evaluate(regs) {
      let reg = regs.entry(self.dest.clone()).or_insert(0);
      match self.op {
        Operation::Inc(value) => *reg += value,
        Operation::Dec(value) => *reg -= value,
      }
      Some((self.dest.clone(), *reg))
    } else {
      None
    }
  }
}

fn process_data_a(data: &str) -> HashMap<String, i32> {
  let mut instructions = Vec::new();
  let mut regs: HashMap<String, i32> = HashMap::new();
  for line in data.lines() {
    let instruction: Instruction = line.parse().unwrap();
    instruction.execute(&mut regs);
    instructions.push(instruction);
  }
  regs
}

fn process_data_b(data: &str) -> (String, i32) {
  let mut instructions = Vec::new();
  let mut regs: HashMap<String, i32> = HashMap::new();
  let mut max = ("a".to_string(), 0);
  for line in data.lines() {
    let instruction: Instruction = line.parse().unwrap();
    if let Some(new) = instruction.execute(&mut regs) {
      if new.1 > max.1 {
        max = new;
      }
    }
    instructions.push(instruction);
  }
  max
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
  fn number(&self) -> String {
    String::from("8")
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    let result = process_data_a(INPUT);
    println!("Result = {}", result.values().max().unwrap());
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    let result = process_data_b(INPUT);
    println!("Result = {}", result.1);
  }
}

#[test]
fn a() {
  let expected = hashmap!{
    "a".to_string() => 1,
    "b".to_string() => 0,
    "c".to_string() => -10,
  };
  assert_eq!(
    process_data_a(
      "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10",
    ),
    expected
  );
}

#[test]
fn b() {
  assert_eq!(
    process_data_b(
      "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10",
    ),
    ("c".to_string(), 10)
  );
}
