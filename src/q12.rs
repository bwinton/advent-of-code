//-----------------------------------------------------
// Setup.

use day;
use regex::Regex;
use std::str::FromStr;

// static INPUT : &'static str = "cpy 41 a
// inc a
// inc a
// dec a
// jnz a 2
// dec a";
static INPUT : &'static str = "cpy 1 a
cpy 1 b
cpy 26 d
jnz c 2
jnz 1 5
cpy 7 c
inc d
dec c
jnz c -2
cpy a c
inc a
dec b
jnz b -2
cpy c b
dec d
jnz d -6
cpy 19 c
cpy 14 d
inc a
dec d
jnz d -2
dec c
jnz c -5";

#[derive(Clone)]
#[derive(Debug)]
enum Instruction {
  CopyLiteral(i32, usize),
  CopyRegister(usize, usize),
  Increment(usize),
  Decrement(usize),
  JumpLiteral(i32, i32),
  JumpRegister(usize, i32)
}

impl Instruction {
  fn execute(&self, state: State) -> State {
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
    return rv;
  }
}

impl FromStr for Instruction {
  type Err = ();

  fn from_str(s: &str) -> Result<Instruction, ()> {
    let copy_literal_re: Regex = Regex::new(r"^cpy (-?[0-9]+) ([a-z])$").unwrap();
    let copy_literal_captures = copy_literal_re.captures(s);
    match copy_literal_captures {
      Some(cap) => {
        return Ok(Instruction::CopyLiteral(
          cap.at(1).unwrap().parse().unwrap(),
          reg_index(cap.at(2)).unwrap()
        ));
      },
      None => {}
    }

    let copy_register_re: Regex = Regex::new(r"^cpy ([a-z]) ([a-z])$").unwrap();
    let copy_register_captures = copy_register_re.captures(s);
    match copy_register_captures {
      Some(cap) => {
        return Ok(Instruction::CopyRegister(
          reg_index(cap.at(1)).unwrap(),
          reg_index(cap.at(2)).unwrap()
        ));
      },
      None => {}
    }

    let increment_re: Regex = Regex::new(r"^inc ([a-z])$").unwrap();
    let increment_captures = increment_re.captures(s);
    match increment_captures {
      Some(cap) => {
        return Ok(Instruction::Increment(
          reg_index(cap.at(1)).unwrap()
        ));
      },
      None => {}
    }

    let decrement_re: Regex = Regex::new(r"^dec ([a-z])$").unwrap();
    let decrement_captures = decrement_re.captures(s);
    match decrement_captures {
      Some(cap) => {
        return Ok(Instruction::Decrement(
          reg_index(cap.at(1)).unwrap()
        ));
      },
      None => {}
    }

    let jump_literal_re: Regex = Regex::new(r"^jnz (-?[0-9]+) (-?[0-9]+)$").unwrap();
    let jump_literal_captures = jump_literal_re.captures(s);
    match jump_literal_captures {
      Some(cap) => {
        return Ok(Instruction::JumpLiteral(
          cap.at(1).unwrap().parse().unwrap(),
          cap.at(2).unwrap().parse().unwrap()
        ));
      },
      None => {}
    }

    let jump_register_re: Regex = Regex::new(r"^jnz ([a-z]) (-?[0-9]+)$").unwrap();
    let jump_register_captures = jump_register_re.captures(s);
    match jump_register_captures {
      Some(cap) => {
        return Ok(Instruction::JumpRegister(
          reg_index(cap.at(1)).unwrap(),
          cap.at(2).unwrap().parse().unwrap()
        ));
      },
      None => {}
    }

    return Err(());
  }
}

#[derive(Clone)]
#[derive(Debug)]
struct State {
  registers: [i32; 4],
  pc: i32
}

fn reg_index(s: Option<&str>) -> Option<usize> {
  match s.unwrap() {
    "a" => Some(0),
    "b" => Some(1),
    "c" => Some(2),
    "d" => Some(3),
    &_ => None
  }
}

fn execute(state: State, instructions: &Vec<Instruction>) -> State {
  let instruction = &instructions[state.pc as usize];
  return instruction.execute(state);
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl day::Day for Q {
  fn number(&self) -> String {
    return String::from("12");
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    let mut instructions : Vec<Instruction> = Vec::new();
    for line in INPUT.lines() {
      let instruction = line.parse().unwrap();
      instructions.push(instruction);
    }
    let mut state = State{registers: [0,0,0,0], pc: 0};

    while 0 <= state.pc && state.pc < instructions.len() as i32 {
      state = execute(state, &instructions);
    }

    let result = state.registers[0];
    println!("Result = {}", result);
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    let mut instructions : Vec<Instruction> = Vec::new();
    for line in INPUT.lines() {
      let instruction = line.parse().unwrap();
      instructions.push(instruction);
    }
    let mut state = State{registers: [0,0,1,0], pc: 0};

    while 0 <= state.pc && state.pc < instructions.len() as i32 {
      state = execute(state, &instructions);
    }

    let result = state.registers[0];
    println!("Result = {}", result);
  }
}
