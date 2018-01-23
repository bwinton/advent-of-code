//-----------------------------------------------------
// Setup.

use aoc::Day;
use regex::Regex;
use std::str::FromStr;

// static INPUT : &'static str = "Disc #1 has 5 positions; at time=0, it is at position 4.
// Disc #2 has 2 positions; at time=0, it is at position 1.";
static INPUT: &'static str = "Disc #1 has 17 positions; at time=0, it is at position 1.
Disc #2 has 7 positions; at time=0, it is at position 0.
Disc #3 has 19 positions; at time=0, it is at position 2.
Disc #4 has 5 positions; at time=0, it is at position 0.
Disc #5 has 3 positions; at time=0, it is at position 0.
Disc #6 has 13 positions; at time=0, it is at position 5.";

#[derive(Clone)]
#[derive(Debug)]
struct Disc {
  number: usize,
  positions: usize,
  current: usize,
}

impl Disc {
  fn is_valid(&self, time: usize) -> bool {
    (time + self.number + self.current) % self.positions == 0
  }
}

impl FromStr for Disc {
  type Err = ();

  fn from_str(s: &str) -> Result<Disc, ()> {
    let re: Regex = Regex::new(
      r"^Disc #([0-9]+) has ([0-9]+) positions; at time=0, it is at position ([0-9]+)\.$",
    ).unwrap();
    let captures = re.captures(s);
    match captures {
      None => Err(()),
      Some(cap) => {
        let number = cap.at(1).unwrap().parse().unwrap();
        let positions = cap.at(2).unwrap().parse().unwrap();
        let current = cap.at(3).unwrap().parse().unwrap();
        Ok(Disc {
          number: number,
          positions: positions,
          current: current,
        })
      },
    }
  }
}

fn get_result(input: &str) -> usize {
  let mut state = Vec::new();
  for line in input.lines() {
    let disc: Disc = line.parse().unwrap();
    state.push(disc);
  }
  let mut is_valid = false;
  let mut i: i32 = -1;
  while !is_valid {
    i += 1;
    is_valid = true;
    for disc in state.clone() {
      is_valid &= disc.is_valid(i as usize);
      if !is_valid {
        break;
      }
    }
  }
  i as usize
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
  fn number(&self) -> String {
    String::from("15")
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    println!("Result = {}", get_result(INPUT));
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    let input = INPUT.to_owned() + "\nDisc #7 has 11 positions; at time=0, it is at position 0.";
    println!("Result = {}", get_result(&input));
  }
}
