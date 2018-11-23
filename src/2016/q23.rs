//-----------------------------------------------------
// Setup.

use aoc::Day;
use regex::Regex;
use std::str::FromStr;

// static INPUT : &'static str = "cpy 2 a
// tgl a
// tgl a
// tgl a
// cpy 1 a
// dec a
// dec a";
static INPUT: &'static str = include_str!("data/q23.data");

#[derive(Clone, Debug)]
enum Instruction {
    CopyLiteral(i32, i32),
    CopyRegister(i32, i32),
    Increment(i32),
    Decrement(i32),
    JumpLitLit(i32, i32),
    JumpLitReg(i32, i32),
    JumpRegLit(i32, i32),
    JumpRegReg(i32, i32),
    Toggle(i32),
}

impl Instruction {
    fn execute(&self, state: &State) -> State {
        let mut rv = state.clone();
        match (*self).clone() {
            Instruction::CopyLiteral(lit, reg) => {
                rv.pc += 1;
                if reg_valid(reg, &rv) {
                    rv.registers[reg as usize] = lit;
                }
            }
            Instruction::CopyRegister(reg_a, reg_b) => {
                rv.pc += 1;
                if reg_valid(reg_a, &rv) && reg_valid(reg_b, &rv) {
                    rv.registers[reg_b as usize] = rv.registers[reg_a as usize];
                }
            }
            Instruction::Increment(reg) => {
                rv.pc += 1;
                if reg_valid(reg, &rv) {
                    rv.registers[reg as usize] += 1;
                }
            }
            Instruction::Decrement(reg) => {
                rv.pc += 1;
                if reg_valid(reg, &rv) {
                    rv.registers[reg as usize] -= 1;
                }
            }
            Instruction::JumpLitLit(test, offset) => {
                if test != 0 {
                    rv.pc += offset;
                } else {
                    rv.pc += 1;
                }
            }
            Instruction::JumpLitReg(test, reg) => {
                if reg_valid(reg, &rv) && test != 0 {
                    rv.pc += rv.registers[reg as usize];
                } else {
                    rv.pc += 1;
                }
            }
            Instruction::JumpRegLit(reg, offset) => {
                if reg_valid(reg, &rv) && rv.registers[reg as usize] != 0 {
                    rv.pc += offset;
                } else {
                    rv.pc += 1;
                }
            }
            Instruction::JumpRegReg(reg_test, reg_offset) => {
                if reg_valid(reg_test, &rv)
                    && reg_valid(reg_offset, &rv)
                    && rv.registers[reg_test as usize] != 0
                {
                    rv.pc += rv.registers[reg_offset as usize];
                } else {
                    rv.pc += 1;
                }
            }
            Instruction::Toggle(reg) => {
                if reg_valid(reg, &rv) {
                    let index = (rv.pc + rv.registers[reg as usize]) as usize;
                    if index < rv.instructions.len() {
                        let new_instruction = rv.instructions[index].clone();
                        match new_instruction {
                            Instruction::CopyLiteral(lit, reg) => {
                                rv.instructions[index] = Instruction::JumpLitReg(lit, reg);
                            }
                            Instruction::CopyRegister(reg_a, reg_b) => {
                                rv.instructions[index] = Instruction::JumpRegReg(reg_a, reg_b);
                            }
                            Instruction::Increment(reg) => {
                                rv.instructions[index] = Instruction::Decrement(reg);
                            }
                            Instruction::Decrement(reg) | Instruction::Toggle(reg) => {
                                rv.instructions[index] = Instruction::Increment(reg);
                            }
                            Instruction::JumpLitLit(_test, _offset)
                            | Instruction::JumpRegLit(_test, _offset) => {
                                println!("GAAAAHHHH!!!!");
                                // rv.instructions[index] = Instruction::CopyLiteral(test, offset);
                            }
                            Instruction::JumpLitReg(test, offset) => {
                                rv.instructions[index] = Instruction::CopyLiteral(test, offset);
                            }
                            Instruction::JumpRegReg(reg_test, reg_offset) => {
                                rv.instructions[index] =
                                    Instruction::CopyRegister(reg_test, reg_offset);
                            }
                        }
                    }
                }
                rv.pc += 1;
            }
        }
        rv
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Instruction, ()> {
        lazy_static! {
            static ref COPY_LITERAL_RE: Regex = Regex::new(r"^cpy (-?[0-9]+) ([a-z])$").unwrap();
            static ref COPY_REGISTER_RE: Regex = Regex::new(r"^cpy ([a-z]) ([a-z])$").unwrap();
            static ref INCREMENT_RE: Regex = Regex::new(r"^inc ([a-z])$").unwrap();
            static ref DECREMENT_RE: Regex = Regex::new(r"^dec ([a-z])$").unwrap();
            static ref JUMP_LITLIT_RE: Regex = Regex::new(r"^jnz (-?[0-9]+) (-?[0-9]+)$").unwrap();
            static ref JUMP_LITREG_RE: Regex = Regex::new(r"^jnz (-?[0-9]+) ([a-z])$").unwrap();
            static ref JUMP_REGLIT_RE: Regex = Regex::new(r"^jnz ([a-z]) (-?[0-9]+)$").unwrap();
            static ref JUMP_REGREG_RE: Regex = Regex::new(r"^jnz ([a-z]) ([a-z])$").unwrap();
            static ref TOGGLE_RE: Regex = Regex::new(r"^tgl ([a-z])$").unwrap();
        }

        if let Some(cap) = COPY_LITERAL_RE.captures(s) {
            return Ok(Instruction::CopyLiteral(
                cap[1].parse().unwrap(),
                reg_index(&cap[2]).unwrap(),
            ));
        }

        if let Some(cap) = COPY_REGISTER_RE.captures(s) {
            return Ok(Instruction::CopyRegister(
                reg_index(&cap[1]).unwrap(),
                reg_index(&cap[2]).unwrap(),
            ));
        }

        if let Some(cap) = INCREMENT_RE.captures(s) {
            return Ok(Instruction::Increment(reg_index(&cap[1]).unwrap()));
        }

        if let Some(cap) = DECREMENT_RE.captures(s) {
            return Ok(Instruction::Decrement(reg_index(&cap[1]).unwrap()));
        }

        if let Some(cap) = JUMP_LITLIT_RE.captures(s) {
            return Ok(Instruction::JumpLitLit(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        if let Some(cap) = JUMP_LITREG_RE.captures(s) {
            return Ok(Instruction::JumpLitReg(
                cap[1].parse().unwrap(),
                reg_index(&cap[2]).unwrap(),
            ));
        }

        if let Some(cap) = JUMP_REGLIT_RE.captures(s) {
            return Ok(Instruction::JumpRegLit(
                reg_index(&cap[1]).unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        if let Some(cap) = JUMP_REGREG_RE.captures(s) {
            return Ok(Instruction::JumpRegReg(
                reg_index(&cap[1]).unwrap(),
                reg_index(&cap[2]).unwrap(),
            ));
        }

        if let Some(cap) = TOGGLE_RE.captures(s) {
            return Ok(Instruction::Toggle(reg_index(&cap[1]).unwrap()));
        }

        println!("Unknown instruction! '{}'", s);
        Err(())
    }
}

#[derive(Clone, Debug)]
struct State {
    registers: [i32; 4],
    pc: i32,
    instructions: Vec<Instruction>,
}

fn reg_index(s: &str) -> Option<i32> {
    match s {
        "a" => Some(0),
        "b" => Some(1),
        "c" => Some(2),
        "d" => Some(3),
        &_ => None,
    }
}

fn reg_valid(reg: i32, state: &State) -> bool {
    reg >= 0 && reg < state.registers.len() as i32
}

fn execute(state: &State) -> State {
    state.instructions[state.pc as usize].execute(state)
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
    fn number(&self) -> String {
        String::from("23")
    }

    fn a(&self) {
        print!("{}A: ", self.number());
        let mut instructions: Vec<Instruction> = Vec::new();
        for line in INPUT.lines() {
            let instruction = line.parse().unwrap();
            instructions.push(instruction);
        }
        let mut state = State {
            registers: [7, 0, 0, 0],
            pc: 0,
            instructions,
        };

        while 0 <= state.pc && state.pc < state.instructions.len() as i32 {
            state = execute(&state);
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
            registers: [12, 0, 0, 0],
            pc: 0,
            instructions,
        };
        while 0 <= state.pc && state.pc < state.instructions.len() as i32 {
            state = execute(&state);
        }

        let result = state.registers[0];
        println!("Result = {}", result);
    }
}
