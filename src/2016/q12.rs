//-----------------------------------------------------
// Setup.

use aoc::Day;
use regex::Regex;
use std::str::FromStr;

// static INPUT : &'static str = "cpy 41 a
// inc a
// inc a
// dec a
// jnz a 2
// dec a";
static INPUT: &'static str = include_str!("data/q12.data");

#[derive(Clone, Debug)]
enum Instruction {
  CopyLiteral(i32, usize),
  CopyRegister(usize, usize),
  Increment(usize),
  Decrement(usize),
  JumpLiteral(i32, i32),
  JumpRegister(usize, i32),
}

impl Instruction {
  fn execute(&self, state: &State) -> State {
    let mut rv = state.clone();
    match (*self).clone() {
      Instruction::CopyLiteral(lit, reg) => {
        rv.pc += 1;
        rv.registers[reg] = lit;
      }
      Instruction::CopyRegister(reg_a, reg_b) => {
        rv.pc += 1;
        rv.registers[reg_b] = rv.registers[reg_a];
      }
      Instruction::Increment(reg) => {
        rv.pc += 1;
        rv.registers[reg] += 1;
      }
      Instruction::Decrement(reg) => {
        rv.pc += 1;
        rv.registers[reg] -= 1;
      }
      Instruction::JumpLiteral(test, offset) => {
        if test != 0 {
          rv.pc += offset;
        } else {
          rv.pc += 1;
        }
      }
      Instruction::JumpRegister(reg, offset) => {
        if rv.registers[reg] != 0 {
          rv.pc += offset;
        } else {
          rv.pc += 1;
        }
      }
    }
    // println!("{:?} {:?}", self, rv);
    rv
  }
}

impl FromStr for Instruction {
  type Err = ();

  fn from_str(s: &str) -> Result<Instruction, ()> {
    let copy_literal_re: Regex = Regex::new(r"^cpy (-?[0-9]+) ([a-z])$").unwrap();
    if let Some(cap) = copy_literal_re.captures(s) {
      return Ok(Instruction::CopyLiteral(
        cap[1].parse().unwrap(),
        reg_index(&cap[2]).unwrap(),
      ));
    }

    let copy_register_re: Regex = Regex::new(r"^cpy ([a-z]) ([a-z])$").unwrap();
    if let Some(cap) = copy_register_re.captures(s) {
      return Ok(Instruction::CopyRegister(
        reg_index(&cap[1]).unwrap(),
        reg_index(&cap[2]).unwrap(),
      ));
    }

    let increment_re: Regex = Regex::new(r"^inc ([a-z])$").unwrap();
    if let Some(cap) = increment_re.captures(s) {
      return Ok(Instruction::Increment(reg_index(&cap[1]).unwrap()));
    }

    let decrement_re: Regex = Regex::new(r"^dec ([a-z])$").unwrap();
    if let Some(cap) = decrement_re.captures(s) {
      return Ok(Instruction::Decrement(reg_index(&cap[1]).unwrap()));
    }

    let jump_literal_re: Regex = Regex::new(r"^jnz (-?[0-9]+) (-?[0-9]+)$").unwrap();
    if let Some(cap) = jump_literal_re.captures(s) {
      return Ok(Instruction::JumpLiteral(
        cap[1].parse().unwrap(),
        cap[2].parse().unwrap(),
      ));
    }

    let jump_register_re: Regex = Regex::new(r"^jnz ([a-z]) (-?[0-9]+)$").unwrap();
    if let Some(cap) = jump_register_re.captures(s) {
      return Ok(Instruction::JumpRegister(
        reg_index(&cap[1]).unwrap(),
        cap[2].parse().unwrap(),
      ));
    }

    Err(())
  }
}

#[derive(Clone, Debug)]
struct State {
  registers: [i32; 4],
  pc: i32,
}

fn reg_index(s: &str) -> Option<usize> {
  match s {
    "a" => Some(0),
    "b" => Some(1),
    "c" => Some(2),
    "d" => Some(3),
    &_ => None,
  }
}

fn execute(state: &State, instructions: &[Instruction]) -> State {
  instructions[state.pc as usize].execute(state)
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
  fn number(&self) -> String {
    String::from("12")
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in INPUT.lines() {
      let instruction = line.parse().unwrap();
      instructions.push(instruction);
    }
    let mut state = State {
      registers: [0, 0, 0, 0],
      pc: 0,
    };

    while 0 <= state.pc && state.pc < instructions.len() as i32 {
      state = execute(&state, &instructions);
    }

    let result = state.registers[0];
    println!("Result = {}", result);
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in INPUT.lines() {
      let instruction = line.parse().unwrap();
      instructions.push(instruction);
    }
    let mut state = State {
      registers: [0, 0, 1, 0],
      pc: 0,
    };

    while 0 <= state.pc && state.pc < instructions.len() as i32 {
      state = execute(&state, &instructions);
    }

    let result = state.registers[0];
    println!("Result = {}", result);
  }
}
