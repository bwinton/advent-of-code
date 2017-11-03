//-----------------------------------------------------
// Setup.

use day;
use std::collections::VecDeque;

// static INPUT : usize = 5;
static INPUT : usize = 3014603;

#[derive(Debug)]
struct Elf {
  position: usize,
  presents: usize
}

fn get_result_a() -> usize {
  let mut elves = VecDeque::new();
  for i in 0..INPUT {
    elves.push_back(Elf{position: i + 1, presents: 1});
  }
  // println!("{:?}", elves);
  while elves.len() > 1 {
    let front = elves.pop_front().unwrap();
    let next = elves.pop_front().unwrap();
    elves.push_back(Elf{position: front.position, presents: front.presents + next.presents});
  }
  elves[0].position
}

fn get_result_b() -> usize {
  return 0;
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl day::Day for Q {
  fn number(&self) -> String {
    return String::from("19");
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    println!("Result = {}", get_result_a());
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    println!("Result = {}", get_result_b());
  }
}
