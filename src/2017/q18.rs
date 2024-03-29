//-----------------------------------------------------
// Setup.

use aoc::Day;

use regex::Regex;
use std::{collections::HashMap, str::FromStr};

static INPUT: &str = include_str!("data/q18.data");

#[derive(Clone, Debug)]
enum Instruction {
    SendReg(char),
    SendLit(i64),
    SetReg(char, char),
    SetLit(char, i64),
    AddReg(char, char),
    AddLit(char, i64),
    MulReg(char, char),
    MulLit(char, i64),
    ModReg(char, char),
    ModLit(char, i64),
    Receive(char),
    JumpRegReg(char, char),
    JumpRegLit(char, i64),
    JumpLitReg(i64, char),
    JumpLitLit(i64, i64),
}

impl Instruction {
    fn execute(&self, state: &State) -> (State, Option<i64>) {
        let mut rv = state.clone();
        let mut value = None;
        match (*self).clone() {
            Instruction::SendReg(reg) => {
                rv.pc += 1;
                rv.outgoing.push(state.registers[&reg]);
            }
            Instruction::SendLit(lit) => {
                rv.pc += 1;
                rv.outgoing.push(lit);
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
                rv.registers
                    .insert(dst, state.registers[&dst] + state.registers[&src]);
            }
            Instruction::AddLit(reg, lit) => {
                rv.pc += 1;
                rv.registers.insert(reg, state.registers[&reg] + lit);
            }
            Instruction::MulReg(dst, src) => {
                rv.pc += 1;
                rv.registers
                    .insert(dst, state.registers[&dst] * state.registers[&src]);
            }
            Instruction::MulLit(reg, lit) => {
                rv.pc += 1;
                rv.registers.insert(reg, state.registers[&reg] * lit);
            }
            Instruction::ModReg(dst, src) => {
                rv.pc += 1;
                rv.registers
                    .insert(dst, state.registers[&dst] % state.registers[&src]);
            }
            Instruction::ModLit(reg, lit) => {
                rv.pc += 1;
                rv.registers.insert(reg, state.registers[&reg] % lit);
            }
            Instruction::Receive(reg) => {
                if state.kind == 'A' {
                    rv.pc += 1;
                    if state.registers[&reg] != 0 {
                        value = rv.incoming.pop();
                    }
                } else {
                    value = rv.incoming.pop();
                    match value {
                        None => rv.waiting = true,
                        Some(x) => {
                            rv.pc += 1;
                            rv.registers.insert(reg, x);
                        }
                    }
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
            Instruction::SendReg(reg)
            | Instruction::SetLit(reg, _)
            | Instruction::AddLit(reg, _)
            | Instruction::MulLit(reg, _)
            | Instruction::ModLit(reg, _)
            | Instruction::Receive(reg)
            | Instruction::JumpRegLit(reg, _)
            | Instruction::JumpLitReg(_, reg) => vec![reg],
            Instruction::SendLit(_) | Instruction::JumpLitLit(_, _) => vec![],
            Instruction::SetReg(a, b)
            | Instruction::AddReg(a, b)
            | Instruction::MulReg(a, b)
            | Instruction::ModReg(a, b)
            | Instruction::JumpRegReg(a, b) => vec![a, b],
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Instruction, ()> {
        let send_reg_re: &Regex = regex!(r"^snd ([a-z])$");
        let send_lit_re: &Regex = regex!(r"^snd (-?\d+)$");
        let set_reg_re: &Regex = regex!(r"^set ([a-z]) ([a-z])$");
        let set_lit_re: &Regex = regex!(r"^set ([a-z]) (-?\d+)$");
        let add_reg_re: &Regex = regex!(r"^add ([a-z]) ([a-z])$");
        let add_lit_re: &Regex = regex!(r"^add ([a-z]) (-?\d+)$");
        let mul_reg_re: &Regex = regex!(r"^mul ([a-z]) ([a-z])$");
        let mul_lit_re: &Regex = regex!(r"^mul ([a-z]) (-?\d+)$");
        let mod_reg_re: &Regex = regex!(r"^mod ([a-z]) ([a-z])$");
        let mod_lit_re: &Regex = regex!(r"^mod ([a-z]) (-?\d+)$");
        let receive_re: &Regex = regex!(r"^rcv ([a-z])$");
        let jump_litlit_re: &Regex = regex!(r"^jgz (-?[0-9]+) (-?[0-9]+)$");
        let jump_litreg_re: &Regex = regex!(r"^jgz (-?[0-9]+) ([a-z])$");
        let jump_reglit_re: &Regex = regex!(r"^jgz ([a-z]) (-?[0-9]+)$");
        let jump_regreg_re: &Regex = regex!(r"^jgz ([a-z]) ([a-z])$");

        if let Some(cap) = send_reg_re.captures(s) {
            return Ok(Instruction::SendReg(cap[1].parse().unwrap()));
        }

        if let Some(cap) = send_lit_re.captures(s) {
            return Ok(Instruction::SendLit(cap[1].parse().unwrap()));
        }

        if let Some(cap) = set_reg_re.captures(s) {
            return Ok(Instruction::SetReg(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        if let Some(cap) = set_lit_re.captures(s) {
            return Ok(Instruction::SetLit(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        if let Some(cap) = add_reg_re.captures(s) {
            return Ok(Instruction::AddReg(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        if let Some(cap) = add_lit_re.captures(s) {
            return Ok(Instruction::AddLit(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        if let Some(cap) = mul_reg_re.captures(s) {
            return Ok(Instruction::MulReg(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        if let Some(cap) = mul_lit_re.captures(s) {
            return Ok(Instruction::MulLit(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        if let Some(cap) = mod_reg_re.captures(s) {
            return Ok(Instruction::ModReg(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        if let Some(cap) = mod_lit_re.captures(s) {
            return Ok(Instruction::ModLit(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        if let Some(cap) = receive_re.captures(s) {
            return Ok(Instruction::Receive(cap[1].parse().unwrap()));
        }

        if let Some(cap) = jump_regreg_re.captures(s) {
            return Ok(Instruction::JumpRegReg(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        if let Some(cap) = jump_reglit_re.captures(s) {
            return Ok(Instruction::JumpRegLit(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        if let Some(cap) = jump_litreg_re.captures(s) {
            return Ok(Instruction::JumpLitReg(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        if let Some(cap) = jump_litlit_re.captures(s) {
            return Ok(Instruction::JumpLitLit(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        println!("Unknown instruction! '{}'", s);
        Err(())
    }
}

#[derive(Clone, Debug)]
struct State {
    kind: char,
    registers: HashMap<char, i64>,
    pc: i64,
    incoming: Vec<i64>,
    outgoing: Vec<i64>,
    instructions: Vec<Instruction>,
    waiting: bool,
}

impl State {
    pub fn new(kind: char, instructions: Vec<Instruction>, registers: HashMap<char, i64>) -> State {
        State {
            kind,
            registers,
            pc: 0,
            incoming: Vec::new(),
            outgoing: Vec::new(),
            instructions,
            waiting: false,
        }
    }

    fn execute(&self) -> (State, Option<i64>) {
        let instruction = &self.instructions[self.pc as usize];
        instruction.execute(self)
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
    let mut state = State::new('A', instructions, registers);
    let mut value = None;
    while value.is_none() && (state.pc as usize) < state.instructions.len() {
        let temp = state.execute();
        state = temp.0;
        value = temp.1;
        while let Some(data) = state.outgoing.pop() {
            state.incoming.insert(0, data);
        }
    }
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
    let mut state_a = State::new('B', instructions.clone(), regs_a);
    let mut regs_b = registers;
    regs_b.insert('p', 1);
    let mut state_b = State::new('B', instructions, regs_b);

    let mut value = 0;
    while !(state_a.waiting && state_b.waiting) {
        while !state_a.waiting && (state_a.pc as usize) < state_a.instructions.len() {
            let temp = state_a.execute();
            state_a = temp.0;
            while let Some(data) = state_a.outgoing.pop() {
                state_b.incoming.insert(0, data);
                state_b.waiting = false;
            }
        }
        while !state_b.waiting && (state_b.pc as usize) < state_b.instructions.len() {
            let temp = state_b.execute();
            state_b = temp.0;
            while let Some(data) = state_b.outgoing.pop() {
                value += 1;
                state_a.incoming.insert(0, data);
                state_a.waiting = false;
            }
        }
    }
    value
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
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
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(
            "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2",
        ),
        4
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(
            "snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d",
        ),
        3
    );
}
