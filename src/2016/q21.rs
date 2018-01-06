//-----------------------------------------------------
// Setup.

use day;
use regex::Regex;
use std::str::FromStr;

// static PASSCODE : &'static str = "abcde";
// static INPUT : &'static str = "swap position 4 with position 0
// swap letter d with letter b
// reverse positions 0 through 4
// rotate left 1 step
// move position 1 to position 4
// move position 3 to position 0
// rotate based on position of letter b
// rotate based on position of letter d";
static PASSCODE : &'static str = "abcdefgh";
static INPUT : &'static str = "reverse positions 1 through 6
rotate based on position of letter a
swap position 4 with position 1
reverse positions 1 through 5
move position 5 to position 7
swap position 4 with position 0
swap position 4 with position 6
rotate based on position of letter a
swap position 0 with position 2
move position 5 to position 2
move position 7 to position 1
swap letter d with letter c
swap position 5 with position 3
reverse positions 3 through 7
rotate based on position of letter d
swap position 7 with position 5
rotate based on position of letter f
swap position 4 with position 1
swap position 3 with position 6
reverse positions 4 through 7
rotate based on position of letter c
move position 0 to position 5
swap position 7 with position 4
rotate based on position of letter f
reverse positions 1 through 3
move position 5 to position 3
rotate based on position of letter g
reverse positions 2 through 5
rotate right 0 steps
rotate left 0 steps
swap letter f with letter b
rotate based on position of letter h
move position 1 to position 3
reverse positions 3 through 6
rotate based on position of letter h
swap position 4 with position 3
swap letter b with letter h
swap letter a with letter h
reverse positions 1 through 6
swap position 3 with position 6
swap letter e with letter d
swap letter e with letter h
swap position 1 with position 5
rotate based on position of letter a
reverse positions 4 through 5
swap position 0 with position 4
reverse positions 0 through 3
move position 7 to position 2
swap letter e with letter c
swap position 3 with position 4
rotate left 3 steps
rotate left 7 steps
rotate based on position of letter e
reverse positions 5 through 6
move position 1 to position 5
move position 1 to position 2
rotate left 1 step
move position 7 to position 6
rotate left 0 steps
reverse positions 5 through 6
reverse positions 3 through 7
swap letter d with letter e
rotate right 3 steps
swap position 2 with position 1
swap position 5 with position 7
swap letter h with letter d
swap letter c with letter d
rotate based on position of letter d
swap letter d with letter g
reverse positions 0 through 1
rotate right 0 steps
swap position 2 with position 3
rotate left 4 steps
rotate left 5 steps
move position 7 to position 0
rotate right 1 step
swap letter g with letter f
rotate based on position of letter a
rotate based on position of letter b
swap letter g with letter e
rotate right 4 steps
rotate based on position of letter h
reverse positions 3 through 5
swap letter h with letter e
swap letter g with letter a
rotate based on position of letter c
reverse positions 0 through 4
rotate based on position of letter e
reverse positions 4 through 7
rotate left 4 steps
swap position 0 with position 6
reverse positions 1 through 6
rotate left 2 steps
swap position 5 with position 3
swap letter b with letter d
swap letter b with letter d
rotate based on position of letter d
rotate based on position of letter c
rotate based on position of letter h
move position 4 to position 7";
static SCRAMBLED : &'static str = "fbgdceah";


#[derive(Clone)]
#[derive(Debug)]
enum Instruction {
  SwapPosition(usize, usize),
  SwapLetter(String, String),
  RotateLeft(usize),
  RotateRight(usize),
  RotateLetter(String),
  Reverse(usize, usize),
  Move(usize, usize)
}

impl Instruction {
  fn execute(&self, state: &str) -> String {
    let mut rv = state.to_string();
    match (*self).clone() {
      Instruction::SwapPosition(x, y) => {
        let mut temp: Vec<char> = rv.chars().collect();
        temp.swap(x, y);
        rv = temp.into_iter().collect();
      }
      Instruction::SwapLetter(a, b) => {
        rv = rv.replace(&a, "?");
        rv = rv.replace(&b, &a);
        rv = rv.replace("?", &b);
      }
      Instruction::RotateLeft(n) => {
        let data: Vec<char> = rv.chars().collect();
        let mut temp = Vec::new();
        temp.extend_from_slice(&data[n..]);
        temp.extend_from_slice(&data[..n]);
        rv = temp.into_iter().collect();
      }
      Instruction::RotateRight(n) => {
        let data: Vec<char> = rv.chars().collect();
        let mut temp = Vec::new();
        let index = data.len() - n;
        temp.extend_from_slice(&data[index..]);
        temp.extend_from_slice(&data[..index]);
        rv = temp.into_iter().collect();
      }
      Instruction::RotateLetter(a) => {
        let data: Vec<char> = rv.chars().collect();
        let mut temp = Vec::new();
        let mut index = data.iter().position(|e| e.to_string() == a).unwrap();
        if index >= 4 {
          index += 1;
        }
        index += 1;
        index %= data.len();
        index = data.len() - index;
        temp.extend_from_slice(&data[index..]);
        temp.extend_from_slice(&data[..index]);
        rv = temp.into_iter().collect();
      }
      Instruction::Reverse(x, y) => {
        let mut temp: Vec<char> = rv.chars().collect();
        temp[x..y + 1].reverse();
        rv = temp.into_iter().collect();
      }
      Instruction::Move(x, y) => {
        let mut temp: Vec<char> = rv.chars().collect();
        let element = temp.remove(x);
        temp.insert(y, element);
        rv = temp.into_iter().collect();
      }
    }
    rv
  }

  fn unexecute(&self, state: &str) -> String {
    let mut rv = state.to_string();
    match (*self).clone() {
      Instruction::SwapPosition(x, y) => {
        let mut temp: Vec<char> = rv.chars().collect();
        temp.swap(x, y);
        rv = temp.into_iter().collect();
      }
      Instruction::SwapLetter(a, b) => {
        rv = rv.replace(&a, "?");
        rv = rv.replace(&b, &a);
        rv = rv.replace("?", &b);
      }
      Instruction::RotateLeft(n) => {
        let data: Vec<char> = rv.chars().collect();
        let mut temp = Vec::new();
        let index = data.len() - n;
        temp.extend_from_slice(&data[index..]);
        temp.extend_from_slice(&data[..index]);
        rv = temp.into_iter().collect();
      }
      Instruction::RotateRight(n) => {
        let data: Vec<char> = rv.chars().collect();
        let mut temp = Vec::new();
        temp.extend_from_slice(&data[n..]);
        temp.extend_from_slice(&data[..n]);
        rv = temp.into_iter().collect();
      }
      Instruction::RotateLetter(a) => {
        let data: Vec<char> = rv.chars().collect();
        let mut temp = Vec::new();
        let mut new_index = data.iter().position(|e| e.to_string() == a).unwrap();
        if new_index == 0 {
          new_index = 8;
        }
        let mut index = if new_index % 2 == 1 { (new_index + 1) / 2 } else { 5 + new_index / 2 };
        index %= data.len();
        temp.extend_from_slice(&data[index..]);
        temp.extend_from_slice(&data[..index]);
        rv = temp.into_iter().collect();
      }
      Instruction::Reverse(x, y) => {
        let mut temp: Vec<char> = rv.chars().collect();
        temp[x..y + 1].reverse();
        rv = temp.into_iter().collect();
      }
      Instruction::Move(x, y) => {
        let mut temp: Vec<char> = rv.chars().collect();
        let element = temp.remove(y);
        temp.insert(x, element);
        rv = temp.into_iter().collect();
      }
    }
    rv
  }
}

impl FromStr for Instruction {
  type Err = ();

  fn from_str(s: &str) -> Result<Instruction, ()> {
    lazy_static! {
      static ref SWAP_POSITION_RE: Regex = Regex::new("swap position ([0-9]+) with position ([0-9]+)").unwrap();
      static ref SWAP_LETTER_RE: Regex = Regex::new("swap letter ([a-z]) with letter ([a-z])").unwrap();
      static ref ROTATE_LEFT_RE: Regex = Regex::new("rotate left ([0-9]+) steps?").unwrap();
      static ref ROTATE_RIGHT_RE: Regex = Regex::new("rotate right ([0-9]+) steps?").unwrap();
      static ref ROTATE_LETTER_RE: Regex = Regex::new("rotate based on position of letter ([a-z])").unwrap();
      static ref REVERSE_RE: Regex = Regex::new("reverse positions ([0-9]+) through ([0-9]+)").unwrap();
      static ref MOVE_RE: Regex = Regex::new("move position ([0-9]+) to position ([0-9]+)").unwrap();
    }

    if let Some(cap) = SWAP_POSITION_RE.captures(s) {
      return Ok(Instruction::SwapPosition(
        cap.at(1).unwrap().parse().unwrap(),
        cap.at(2).unwrap().parse().unwrap()
      ));
    }

    if let Some(cap) = SWAP_LETTER_RE.captures(s) {
      return Ok(Instruction::SwapLetter(
        cap.at(1).unwrap().to_string(),
        cap.at(2).unwrap().to_string()
      ));
    }

    if let Some(cap) = ROTATE_LEFT_RE.captures(s) {
      return Ok(Instruction::RotateLeft(
        cap.at(1).unwrap().parse().unwrap()
      ));
    }

    if let Some(cap) = ROTATE_RIGHT_RE.captures(s) {
      return Ok(Instruction::RotateRight(
        cap.at(1).unwrap().parse().unwrap()
      ));
    }

    if let Some(cap) = ROTATE_LETTER_RE.captures(s) {
      return Ok(Instruction::RotateLetter(
        cap.at(1).unwrap().to_string()
      ));
    }

    if let Some(cap) = REVERSE_RE.captures(s) {
      return Ok(Instruction::Reverse(
        cap.at(1).unwrap().parse().unwrap(),
        cap.at(2).unwrap().parse().unwrap()
      ));
    }

    if let Some(cap) = MOVE_RE.captures(s) {
      return Ok(Instruction::Move(
        cap.at(1).unwrap().parse().unwrap(),
        cap.at(2).unwrap().parse().unwrap()
      ));
    }

    Err(())
  }
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
    let mut instructions : Vec<Instruction> = Vec::new();
    for line in INPUT.lines() {
      let instruction = line.parse().unwrap();
      instructions.push(instruction);
    }

    let mut rv = PASSCODE.to_string();
    for instruction in instructions {
      rv = instruction.execute(&rv);
    }
    println!("Result = {}", rv);
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    let mut instructions : Vec<Instruction> = Vec::new();
    for line in INPUT.lines() {
      let instruction = line.parse().unwrap();
      instructions.push(instruction);
    }

    instructions.reverse();
    let mut rv = SCRAMBLED.to_string();
    for instruction in &instructions {
      rv = instruction.unexecute(&rv);
    }
    println!("Result = {}", rv);
  }
}
