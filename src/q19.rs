//-----------------------------------------------------
// Setup.

use day;

// static INPUT : usize = 5;
static INPUT : usize = 3014603;

#[derive(Debug)]
struct Elf {
  position: usize,
  presents: usize
}

fn run_turn(elves: &mut Vec<Elf>) {
  // println!("");
  // println!("start {:?}", elves);
  for i in 0..elves.len() {
    let next = (i + 1) % elves.len();
    if elves[i].presents > 0 {
      elves[i].presents += elves[next].presents;
      elves[next].presents = 0;
    }
  }
  // println!("stealing {:?}", elves);
  elves.retain(|elf| elf.presents > 0);
  // println!("retain {:?}", elves);
  // println!("");
}

fn get_result() -> usize {
  let mut elves = Vec::new();
  for i in 0..INPUT {
    elves.push(Elf{position: i + 1, presents: 1});
  }
  // println!("{:?}", elves);
  while elves.len() > 1 {
    run_turn(&mut elves);
  }
  elves[0].position
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
    println!("Result = {}", get_result());
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    let result = 0;
    println!("Result = {}", result);
  }
}
