//-----------------------------------------------------
// Setup.

use aoc::Day;

use std::collections::HashMap;

static INPUT: &'static str = include_str!("data/q11.data");

#[derive(Debug)]
struct HexPosition {
  x: i32,
  y: i32,
  z: i32,
}

impl HexPosition {
  fn go(&self, direction: &HexPosition) -> HexPosition {
    HexPosition {
      x: self.x + direction.x,
      y: self.y + direction.y,
      z: self.z + direction.z,
    }
  }

  fn distance(&self) -> u32 {
    (self.x.max(0) + self.y.max(0) + self.z.max(0)) as u32
  }
}

fn process_data_a(data: &str) -> u32 {
  lazy_static! {
    static ref DIRECTIONS: HashMap<&'static str, HexPosition> = hashmap!{
      "nw" => HexPosition{ x: -1, y: 0, z: 1},
      "n" => HexPosition{ x: 0, y: -1, z: 1},
      "ne" => HexPosition{ x: 1, y: -1, z: 0},
      "se" => HexPosition{ x: 1, y: 0, z: -1},
      "s" => HexPosition{ x: 0, y: 1, z: -1},
      "sw" => HexPosition{ x: -1, y: 1, z: 0},
    };
  }
  let mut position = HexPosition { x: 0, y: 0, z: 0 };

  for hexmove in data.split(',') {
    position = position.go(&DIRECTIONS[hexmove]);
  }
  position.distance()
}

fn process_data_b(data: &str) -> u32 {
  lazy_static! {
    static ref DIRECTIONS: HashMap<&'static str, HexPosition> = hashmap!{
      "nw" => HexPosition{ x: -1, y: 0, z: 1},
      "n" => HexPosition{ x: 0, y: -1, z: 1},
      "ne" => HexPosition{ x: 1, y: -1, z: 0},
      "se" => HexPosition{ x: 1, y: 0, z: -1},
      "s" => HexPosition{ x: 0, y: 1, z: -1},
      "sw" => HexPosition{ x: -1, y: 1, z: 0},
    };
  }
  let mut position = HexPosition { x: 0, y: 0, z: 0 };
  let mut rv = position.distance();

  for hexmove in data.split(',') {
    position = position.go(&DIRECTIONS[hexmove]);
    if position.distance() > rv {
      rv = position.distance();
    }
  }
  rv
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
  fn number(&self) -> String {
    String::from("11")
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
  assert_eq!(process_data_a("ne,ne,ne"), 3);
  assert_eq!(process_data_a("ne,ne,sw,sw"), 0);
  assert_eq!(process_data_a("ne,ne,s,s"), 2);
  assert_eq!(process_data_a("se,sw,se,sw,sw"), 3);
}

#[test]
fn b() {}
