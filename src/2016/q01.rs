//-----------------------------------------------------
// Setup.

use day;
use std::collections::HashSet;

static INPUT : &'static str = "L5, R1, R4, L5, L4, R3, R1, L1, R4, R5, L1, L3, R4, L2, L4, R2, L4, L1, R3, R1, R1, L1, R1, L5, R5, R2, L5, R2, R1, L2, L4, L4, R191, R2, R5, R1, L1, L2, R5, L2, L3, R4, L1, L1, R1, R50, L1, R1, R76, R5, R4, R2, L5, L3, L5, R2, R1, L1, R2, L3, R4, R2, L1, L1, R4, L1, L1, R185, R1, L5, L4, L5, L3, R2, R3, R1, L5, R1, L3, L2, L2, R5, L1, L1, L3, R1, R4, L2, L1, L1, L3, L4, R5, L2, R3, R5, R1, L4, R5, L3, R3, R3, R1, R1, R5, R2, L2, R5, L5, L4, R4, R3, R5, R1, L3, R1, L2, L2, R3, R4, L1, R4, L1, R4, R3, L1, L4, L1, L5, L2, R2, L1, R1, L5, L3, R4, L1, R5, L5, L5, L1, L3, R1, R5, L2, L4, L5, L1, L1, L2, R5, R5, L4, R3, L2, L1, L3, L4, L5, L5, L2, R4, R3, L5, R4, R2, R1, L5";
// static INPUT : &'static str = "R2, L3";
// static INPUT : &'static str = "R8, R4, R4, R8";

#[derive(Debug)]
enum Heading {
  North, East, South, West
}

impl Heading {
  fn turn(&self, dir: &str) -> Heading {
    match *self {
      Heading::North => if dir == "R" {
        Heading::East
      } else {
        Heading::West
      },
      Heading::East => if dir == "R" {
        Heading::South
      } else {
        Heading::North
      },
      Heading::South => if dir == "R" {
        Heading::West
      } else {
        Heading::East
      },
      Heading::West => if dir == "R" {
        Heading::North
      } else {
        Heading::South
      }
    }
  }
}

type Pos = [i32; 2];

fn handle_turn(turn: &str, heading: &mut Heading) -> i32 {
  let (dir, len_str) = turn.split_at(1);
  let length : i32 = len_str.parse().expect("Wanted a number");
  *heading = heading.turn(dir);
  length
}


//-----------------------------------------------------
// Questions.

pub struct Q;

impl day::Day for Q {
  fn number(&self) -> String {
    String::from("1")
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    let mut heading:Heading = Heading::North;
    let mut pos : Pos = [0, 0];

    fn run_turn(pos : &mut Pos, heading: & Heading, length: i32) {
      match *heading {
        Heading::North => pos[0] += length,
        Heading::East => pos[1] += length,
        Heading::South => pos[0] -= length,
        Heading::West => pos[1] -= length
      }
      // println!("{:?}, {} {:?}", pos, length, heading);
    }

    for data in INPUT.split(", ") {
      let length = handle_turn(data, &mut heading);
      run_turn(&mut pos, &heading, length);
    }
    println!("Result = {}", pos[0].abs() + pos[1].abs());
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    let mut heading:Heading = Heading::North;
    let mut pos : Pos = [0, 0];
    let mut seen = HashSet::new();

    fn run_turn(seen: &mut HashSet<Pos>, pos : &mut Pos, heading: & Heading, length: i32) -> bool {
      for _ in 0..length {
        match *heading {
          Heading::North => pos[0] += 1,
          Heading::East => pos[1] += 1,
          Heading::South => pos[0] -= 1,
          Heading::West => pos[1] -= 1
        }
        if seen.contains(pos) {
          return true;
        } else {
          seen.insert(*pos);
        }
      }
      false
    }

    for data in INPUT.split(", ") {
      let length = handle_turn(data, &mut heading);
      if run_turn(&mut seen, &mut pos, &heading, length) {
        break;
      }
    }
    println!("Result = {}", pos[0].abs() + pos[1].abs());
  }
}
