//-----------------------------------------------------
// Setup.

use day;

use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

static INPUT : &'static str = "set i 31
set a 1
mul p 17
jgz p p
mul a 2
add i -1
jgz i -2
add a -1
set i 127
set p 316
mul p 8505
mod p a
mul p 129749
add p 12345
mod p a
set b p
mod b 10000
snd b
add i -1
jgz i -9
jgz a 3
rcv b
jgz b -1
set f 0
set i 126
rcv a
rcv b
set p a
mul p -1
add p b
jgz p 4
snd a
set a b
jgz 1 3
snd b
set f 1
add i -1
jgz i -11
snd a
jgz f -16
jgz a -19";

#[derive(Clone)]
#[derive(Debug)]
enum Instruction {
  Sound(char),
  SetReg(char, char),
  SetLit(char, i64),
  AddReg(char, char),
  AddLit(char, i64),
  MulReg(char, char),
  MulLit(char, i64),
  ModReg(char, char),
  ModLit(char, i64),
  RecoverReg(char),
  RecoverLit(i64),
  JumpRegReg(char, char),
  JumpRegLit(char, i64),
  JumpLitReg(i64, char),
  JumpLitLit(i64, i64)
}

impl Instruction {
  fn execute(&self, state: &State) -> (State, Option<i64>) {
    let mut rv = state.clone();
    let mut value = None;
    // println!("Executing {:?} on {:?}\n", self, state);
    match (*self).clone() {
      Instruction::Sound(reg) => {
        rv.pc += 1;
        rv.sound(state.registers[&reg]);
      }
      Instruction::SetReg(dst, src) => {
        rv.pc += 1;
        rv.registers.insert(dst, state.registers[&src]);
      }
      Instruction::SetLit(reg, lit) => {
        rv.pc += 1;
        rv.registers.insert(reg, lit);
      }
      Instruction::AddReg(dst, src) => {
        rv.pc += 1;
        rv.registers.insert(dst, state.registers[&dst] + state.registers[&src]);
      }
      Instruction::AddLit(reg, lit) => {
        rv.pc += 1;
        rv.registers.insert(reg, state.registers[&reg] + lit);
      }
      Instruction::MulReg(dst, src) => {
        rv.pc += 1;
        rv.registers.insert(dst, state.registers[&dst] * state.registers[&src]);
      }
      Instruction::MulLit(reg, lit) => {
        rv.pc += 1;
        rv.registers.insert(reg, state.registers[&reg] * lit);
      }
      Instruction::ModReg(dst, src) => {
        rv.pc += 1;
        rv.registers.insert(dst, state.registers[&dst] % state.registers[&src]);
      }
      Instruction::ModLit(reg, lit) => {
        rv.pc += 1;
        rv.registers.insert(reg, state.registers[&reg] % lit);
      }
      Instruction::RecoverReg(reg) => {
        rv.pc += 1;
        if state.registers[&reg] != 0 {
          value = Some(*state.out.last().unwrap());
        }
      }
      Instruction::RecoverLit(lit) => {
        rv.pc += 1;
        if lit != 0 {
          value = Some(*state.out.last().unwrap());
        }
      }
      Instruction::JumpRegReg(reg_test, reg_offset) => {
        if rv.registers[&reg_test] > 0 {
          rv.pc += rv.registers[&reg_offset];
        } else {
          rv.pc += 1;
        }
      }
      Instruction::JumpRegLit(reg, offset) => {
        if rv.registers[&reg] > 0 {
          rv.pc += offset;
        } else {
          rv.pc += 1;
        }
      }
      Instruction::JumpLitReg(test, reg) => {
        if test > 0 {
          rv.pc += rv.registers[&reg];
        } else {
          rv.pc += 1;
        }
      }
      Instruction::JumpLitLit(test, offset) => {
        if test > 0 {
          rv.pc += offset;
        } else {
          rv.pc += 1;
        }
      }
    }
    (rv, value)
  }

  fn registers(&self) -> Vec<char> {
    match (*self).clone() {
      Instruction::Sound(reg) => { vec![reg] }
      Instruction::SetReg(dst, src) => { vec![dst, src] }
      Instruction::SetLit(reg, _) => { vec![reg] }
      Instruction::AddReg(dst, src) => { vec![dst, src] }
      Instruction::AddLit(reg, _) => { vec![reg] }
      Instruction::MulReg(dst, src) => { vec![dst, src] }
      Instruction::MulLit(reg, _) => { vec![reg] }
      Instruction::ModReg(dst, src) => { vec![dst, src] }
      Instruction::ModLit(reg, _) => { vec![reg] }
      Instruction::RecoverReg(reg) => { vec![reg] }
      Instruction::RecoverLit(_) => { vec![] }
      Instruction::JumpRegReg(test, offset) => { vec![test, offset] }
      Instruction::JumpRegLit(reg, _) => { vec![reg] }
      Instruction::JumpLitReg(_, reg) => { vec![reg] }
      Instruction::JumpLitLit(_, _) => { vec![] }
    }
  }
}

impl FromStr for Instruction {
  type Err = ();

  fn from_str(s: &str) -> Result<Instruction, ()> {
    lazy_static! {
      static ref SOUND_RE: Regex = Regex::new(r"^snd ([a-z])$").unwrap();
      static ref SET_REG_RE: Regex = Regex::new(r"^set ([a-z]) ([a-z])$").unwrap();
      static ref SET_LIT_RE: Regex = Regex::new(r"^set ([a-z]) (-?\d+)$").unwrap();
      static ref ADD_REG_RE: Regex = Regex::new(r"^add ([a-z]) ([a-z])$").unwrap();
      static ref ADD_LIT_RE: Regex = Regex::new(r"^add ([a-z]) (-?\d+)$").unwrap();
      static ref MUL_REG_RE: Regex = Regex::new(r"^mul ([a-z]) ([a-z])$").unwrap();
      static ref MUL_LIT_RE: Regex = Regex::new(r"^mul ([a-z]) (-?\d+)$").unwrap();
      static ref MOD_REG_RE: Regex = Regex::new(r"^mod ([a-z]) ([a-z])$").unwrap();
      static ref MOD_LIT_RE: Regex = Regex::new(r"^mod ([a-z]) (-?\d+)$").unwrap();
      static ref RECOVER_REG_RE: Regex = Regex::new(r"^rcv ([a-z])$").unwrap();
      static ref RECOVER_LIT_RE: Regex = Regex::new(r"^rcv (-?\d+)$").unwrap();
      static ref JUMP_LITLIT_RE: Regex = Regex::new(r"^jgz (-?[0-9]+) (-?[0-9]+)$").unwrap();
      static ref JUMP_LITREG_RE: Regex = Regex::new(r"^jgz (-?[0-9]+) ([a-z])$").unwrap();
      static ref JUMP_REGLIT_RE: Regex = Regex::new(r"^jgz ([a-z]) (-?[0-9]+)$").unwrap();
      static ref JUMP_REGREG_RE: Regex = Regex::new(r"^jgz ([a-z]) ([a-z])$").unwrap();
    }

    if let Some(cap) = SOUND_RE.captures(s) {
      return Ok(Instruction::Sound(
        cap.at(1).unwrap().parse().unwrap()
      ));
    }

    if let Some(cap) = SET_REG_RE.captures(s) {
      return Ok(Instruction::SetReg(
        cap.at(1).unwrap().parse().unwrap(),
        cap.at(2).unwrap().parse().unwrap()
      ));
    }

    if let Some(cap) = SET_LIT_RE.captures(s) {
      return Ok(Instruction::SetLit(
        cap.at(1).unwrap().parse().unwrap(),
        cap.at(2).unwrap().parse().unwrap()
      ));
    }

    if let Some(cap) = ADD_REG_RE.captures(s) {
      return Ok(Instruction::AddReg(
        cap.at(1).unwrap().parse().unwrap(),
        cap.at(2).unwrap().parse().unwrap()
      ));
    }

    if let Some(cap) = ADD_LIT_RE.captures(s) {
      return Ok(Instruction::AddLit(
        cap.at(1).unwrap().parse().unwrap(),
        cap.at(2).unwrap().parse().unwrap()
      ));
    }

    if let Some(cap) = MUL_REG_RE.captures(s) {
      return Ok(Instruction::MulReg(
        cap.at(1).unwrap().parse().unwrap(),
        cap.at(2).unwrap().parse().unwrap()
      ));
    }

    if let Some(cap) = MUL_LIT_RE.captures(s) {
      return Ok(Instruction::MulLit(
        cap.at(1).unwrap().parse().unwrap(),
        cap.at(2).unwrap().parse().unwrap()
      ));
    }

    if let Some(cap) = MOD_REG_RE.captures(s) {
      return Ok(Instruction::ModReg(
        cap.at(1).unwrap().parse().unwrap(),
        cap.at(2).unwrap().parse().unwrap()
      ));
    }

    if let Some(cap) = MOD_LIT_RE.captures(s) {
      return Ok(Instruction::ModLit(
        cap.at(1).unwrap().parse().unwrap(),
        cap.at(2).unwrap().parse().unwrap()
      ));
    }

    if let Some(cap) = RECOVER_REG_RE.captures(s) {
      return Ok(Instruction::RecoverReg(
        cap.at(1).unwrap().parse().unwrap()
      ));
    }

    if let Some(cap) = RECOVER_LIT_RE.captures(s) {
      return Ok(Instruction::RecoverLit(
        cap.at(1).unwrap().parse().unwrap()
      ));
    }

    if let Some(cap) = JUMP_REGREG_RE.captures(s) {
      return Ok(Instruction::JumpRegReg(
        cap.at(1).unwrap().parse().unwrap(),
        cap.at(2).unwrap().parse().unwrap()
      ));
    }

    if let Some(cap) = JUMP_REGLIT_RE.captures(s) {
      return Ok(Instruction::JumpRegLit(
        cap.at(1).unwrap().parse().unwrap(),
        cap.at(2).unwrap().parse().unwrap()
      ));
    }

    if let Some(cap) = JUMP_LITREG_RE.captures(s) {
      return Ok(Instruction::JumpLitReg(
        cap.at(1).unwrap().parse().unwrap(),
        cap.at(2).unwrap().parse().unwrap()
      ));
    }

    if let Some(cap) = JUMP_LITLIT_RE.captures(s) {
      return Ok(Instruction::JumpLitLit(
        cap.at(1).unwrap().parse().unwrap(),
        cap.at(2).unwrap().parse().unwrap()
      ));
    }

    println!("Unknown instruction! '{}'", s);
    Err(())
  }
}

#[derive(Clone)]
#[derive(Debug)]
struct State {
  registers: HashMap<char, i64>,
  pc: i64,
  out: Vec<i64>,
  instructions: Vec<Instruction>
}

// impl PartialEq for State {
//   fn eq(&self, other: &State) -> bool {
//     self.pc == other.pc && self.registers == other.registers
//   }
// }
//
// impl Eq for State {}
//
// impl Hash for State {
//   fn hash<H: Hasher>(&self, state: &mut H) {
//     self.pc.hash(state);
//     self.registers.hash(state);
//   }
// }
//
impl State {
  pub fn new(instructions: Vec<Instruction>, registers: HashMap<char, i64>) -> State {
    State {
      registers: registers,
      pc: 0,
      out: Vec::new(),
      instructions:instructions
    }
  }

  fn sound(&mut self, data: i64) {
    self.out.push(data);
  }

  fn execute(&self) -> (State, Option<i64>) {
    let instruction = &self.instructions[self.pc as usize];
    instruction.execute(&self)
  }
}



fn process_data_a(data: &str) -> i64 {
  let mut instructions: Vec<Instruction> = Vec::new();
  let mut registers: HashMap<char, i64> = HashMap::new();
  for line in data.lines() {
    let instruction: Instruction = line.parse().unwrap();
    instructions.push(instruction.clone());
    for reg in instruction.registers() {
      registers.insert(reg, 0);
    }
  }
  // println!("{:?}", instructions);
  let mut state = State::new(instructions, registers);
  let mut value = None;
  while value == None && (state.pc as usize) < state.instructions.len() {
    let temp = state.execute();
    state = temp.0; value = temp.1;
  }
  println!("{:?}", state);
  value.unwrap_or(0)
}

fn process_data_b(data: &str) -> i64 {
  let mut instructions: Vec<Instruction> = Vec::new();
  let mut registers: HashMap<char, i64> = HashMap::new();
  for line in data.lines() {
    let instruction: Instruction = line.parse().unwrap();
    instructions.push(instruction.clone());
    for reg in instruction.registers() {
      registers.insert(reg, 0);
    }
  }
  // println!("{:?}", instructions);
  let mut regs_a = registers.clone();
  regs_a.insert('p', 0);
  let mut state_a = State::new(instructions, regs_a);
  // let regs_b = registers.clone();
  // regs_b.insert('p', 1);
  // let mut state_b = State::new(instructions, regs_b);

  let mut value = None;
  while value == None && (state_a.pc as usize) < state_a.instructions.len() {
    let temp = state_a.execute();
    state_a = temp.0; value = temp.1;
  }
  println!("{:?}", state_a);
  value.unwrap_or(0)
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl day::Day for Q {
  fn number(&self) -> String {
    String::from("18")
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
  assert_eq!(process_data_a("set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2"), 4);
}

#[test]
fn b() {
  assert_eq!(process_data_b("snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d"), 0);
}
