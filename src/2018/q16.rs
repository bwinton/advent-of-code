//-----------------------------------------------------
// Setup.

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::str::FromStr;

static INPUT: &'static str = include_str!("data/q16.data");

#[derive(Debug, Eq, Hash, PartialEq)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

static OPCODES: &'static [Opcode; 16] = &[
    Opcode::Addr,
    Opcode::Addi,
    Opcode::Mulr,
    Opcode::Muli,
    Opcode::Banr,
    Opcode::Bani,
    Opcode::Borr,
    Opcode::Bori,
    Opcode::Setr,
    Opcode::Seti,
    Opcode::Gtir,
    Opcode::Gtri,
    Opcode::Gtrr,
    Opcode::Eqir,
    Opcode::Eqri,
    Opcode::Eqrr,
];

impl Opcode {
    fn execute(&self, a: u32, b: u32, c: u32, registers: [u32; 4]) -> [u32; 4] {
        let mut rv = registers;
        match self {
            Opcode::Addr => {
                // addr (add register) stores into register C the result of adding register A and register B.
                rv[c as usize] = rv[a as usize] + rv[b as usize];
            }
            Opcode::Addi => {
                // addi (add immediate) stores into register C the result of adding register A and value B.
                rv[c as usize] = rv[a as usize] + b;
            }
            Opcode::Mulr => {
                // mulr (multiply register) stores into register C the result of multiplying register A and register B.
                rv[c as usize] = rv[a as usize] * rv[b as usize];
            }
            Opcode::Muli => {
                // muli (multiply immediate) stores into register C the result of multiplying register A and value B.
                rv[c as usize] = rv[a as usize] * b;
            }
            Opcode::Banr => {
                // banr (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
                rv[c as usize] = rv[a as usize] & rv[b as usize];
            }
            Opcode::Bani => {
                //bani (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.
                rv[c as usize] = rv[a as usize] & b;
            }
            Opcode::Borr => {
                // borr (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
                rv[c as usize] = rv[a as usize] | rv[b as usize];
            }
            Opcode::Bori => {
                // bori (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.
                rv[c as usize] = rv[a as usize] | b;
            }
            Opcode::Setr => {
                // setr (set register) copies the contents of register A into register C. (Input B is ignored.)
                rv[c as usize] = rv[a as usize];
            }
            Opcode::Seti => {
                // seti (set immediate) stores value A into register C. (Input B is ignored.)
                rv[c as usize] = a;
            }
            Opcode::Gtir => {
                // gtir (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
                if a > rv[b as usize] {
                    rv[c as usize] = 1;
                } else {
                    rv[c as usize] = 0;
                }
            }
            Opcode::Gtri => {
                // gtri (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
                if rv[a as usize] > b {
                    rv[c as usize] = 1;
                } else {
                    rv[c as usize] = 0;
                }
            }
            Opcode::Gtrr => {
                // gtrr (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
                if rv[a as usize] > rv[b as usize] {
                    rv[c as usize] = 1;
                } else {
                    rv[c as usize] = 0;
                }
            }
            Opcode::Eqir => {
                // eqir (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
                if a == rv[b as usize] {
                    rv[c as usize] = 1;
                } else {
                    rv[c as usize] = 0;
                }
            }
            Opcode::Eqri => {
                // eqri (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
                if rv[a as usize] == b {
                    rv[c as usize] = 1;
                } else {
                    rv[c as usize] = 0;
                }
            }
            Opcode::Eqrr => {
                // eqrr (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
                if rv[a as usize] == rv[b as usize] {
                    rv[c as usize] = 1;
                } else {
                    rv[c as usize] = 0;
                }
            }
        }
        rv
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: u32,
    a: u32,
    b: u32,
    c: u32,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Instruction, ()> {
        let mut values = s.split_whitespace();
        let opcode = values.next().unwrap().parse().unwrap();
        let a = values.next().unwrap().parse().unwrap();
        let b = values.next().unwrap().parse().unwrap();
        let c = values.next().unwrap().parse().unwrap();

        Ok(Instruction { opcode, a, b, c })
    }
}

#[derive(Debug)]
struct Example {
    before: [u32; 4],
    instruction: Instruction,
    after: [u32; 4],
}

impl Example {
    fn works_as(&self, opcode: &Opcode) -> bool {
        opcode.execute(
            self.instruction.a,
            self.instruction.b,
            self.instruction.c,
            self.before,
        ) == self.after
    }
}

fn parse_data(data: &str) -> (Vec<Example>, Vec<Instruction>) {
    lazy_static! {
        static ref BEFORE_RE: Regex = Regex::new(r"Before: \[(\d), (\d), (\d), (\d)\]").unwrap();
        static ref AFTER_RE: Regex = Regex::new(r"After:  \[(\d), (\d), (\d), (\d)\]").unwrap();
    }

    let mut lines = data.lines();

    let mut examples = vec![];
    loop {
        if let Some(cap) = BEFORE_RE.captures(lines.next().unwrap()) {
            let before = [
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
                cap[3].parse().unwrap(),
                cap[4].parse().unwrap(),
            ];
            let instruction = lines.next().unwrap().parse().unwrap();
            if let Some(cap) = AFTER_RE.captures(lines.next().unwrap()) {
                let after = [
                    cap[1].parse().unwrap(),
                    cap[2].parse().unwrap(),
                    cap[3].parse().unwrap(),
                    cap[4].parse().unwrap(),
                ];
                examples.push(Example {
                    before,
                    instruction,
                    after,
                });
                lines.next();
            }
        } else {
            // We're done parsing the first part, so skip the next blank line.
            lines.next();
            break;
        }
    }

    let mut instructions = vec![];
    for line in lines {
        instructions.push(line.parse().unwrap());
    }

    (examples, instructions)
}

fn process_data_a(data: &str) -> usize {
    let (examples, _instructions) = parse_data(data);

    let mut rv = 0;
    for example in examples {
        let mut works = 0;
        for opcode in OPCODES {
            if example.works_as(opcode) {
                works += 1;
            }
        }
        if works >= 3 {
            rv += 1;
        }
    }
    rv
}

fn process_data_b(data: &str) -> u32 {
    let (_examples, instructions) = parse_data(data);
    // for example in examples {
    //     let potentials: &mut HashSet<_> = opcodes.entry(example.instruction.opcode).or_insert_with(|| HashSet::from_iter(OPCODES.iter()));
    //     println!("Potential for {}: {:?}", example.instruction.opcode, potentials);
    //     for opcode in OPCODES {
    //         if !example.works_as(opcode) {
    //             potentials.remove(opcode);
    //         }
    //     }
    // }
    // println!("Instructions! {:?}", opcodes);

    // @todo: Calculate this instead of figuring it out by hand.
    let mut opcodes = HashMap::new();
    opcodes.insert(0, Opcode::Muli);
    opcodes.insert(1, Opcode::Borr);
    opcodes.insert(2, Opcode::Gtri);
    opcodes.insert(3, Opcode::Eqri);
    opcodes.insert(4, Opcode::Gtrr);
    opcodes.insert(5, Opcode::Eqir);
    opcodes.insert(6, Opcode::Addi);
    opcodes.insert(7, Opcode::Setr);
    opcodes.insert(8, Opcode::Mulr);
    opcodes.insert(9, Opcode::Addr);
    opcodes.insert(10, Opcode::Bori);
    opcodes.insert(11, Opcode::Bani);
    opcodes.insert(12, Opcode::Seti);
    opcodes.insert(13, Opcode::Eqrr);
    opcodes.insert(14, Opcode::Banr);
    opcodes.insert(15, Opcode::Gtir);

    let mut registers = [0; 4];
    for instruction in instructions {
        registers = opcodes[&instruction.opcode].execute(
            instruction.a,
            instruction.b,
            instruction.c,
            registers,
        )
    }
    registers[0]
}

//-----------------------------------------------------
// Questions.

q_impl!("16");

#[test]
fn a() {
    assert_eq!(process_data_a(""), 0);
}

#[test]
fn b() {
    assert_eq!(process_data_b(""), 0);
}
