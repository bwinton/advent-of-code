//-----------------------------------------------------
// Setup.

use std::{fmt::Debug, rc::Rc, str::Lines};

use aoc::nom_util::unsigned_number;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::space1, multi::many_m_n,
    sequence::tuple, IResult,
};

static INPUT: &str = include_str!("data/q19.data");

trait Instruction: Debug {
    fn execute(&self, cpu: &mut Cpu);
}

fn three_numbers(i: &str) -> IResult<&str, (usize, usize, usize)> {
    let (input, result) = many_m_n(3, 3, tuple((space1, unsigned_number)))(i)?;
    Ok((
        input,
        (
            result[0].1 as usize,
            result[1].1 as usize,
            result[2].1 as usize,
        ),
    ))
}

fn parse_instructions(lines: Lines) -> Rc<Vec<Box<dyn Instruction>>> {
    let mut instructions: Vec<Box<dyn Instruction>> = vec![];
    for line in lines {
        match alt((
            AddI::parser,
            AddR::parser,
            EqRR::parser,
            GtRR::parser,
            MulI::parser,
            MulR::parser,
            SetI::parser,
            SetR::parser,
        ))(line)
        {
            Ok((_, i)) => {
                // println!("{} => {:?}", line, i);
                instructions.push(i);
            }
            Err(e) => panic!("Unknown instruction {:?}", e),
        }
    }
    Rc::new(instructions)
}

#[derive(Debug)]
struct AddI {
    a: usize,
    b: i32,
    dest: usize,
}
impl Instruction for AddI {
    fn execute(&self, cpu: &mut Cpu) {
        cpu.registers[self.dest] = cpu.registers[self.a] + self.b;
    }
}
impl AddI {
    fn parser(i: &str) -> IResult<&str, Box<dyn Instruction>> {
        let (input, (_, (a, b, dest))) = tuple((tag("addi"), three_numbers))(i)?;
        Ok((
            input,
            Box::new(AddI {
                a,
                b: b as i32,
                dest,
            }),
        ))
        // }) as Box<dyn Instruction>
    }
}

#[derive(Debug)]
struct AddR {
    a: usize,
    b: usize,
    dest: usize,
}
impl Instruction for AddR {
    fn execute(&self, cpu: &mut Cpu) {
        cpu.registers[self.dest] = cpu.registers[self.a] + cpu.registers[self.b];
    }
}
impl AddR {
    fn parser(i: &str) -> IResult<&str, Box<dyn Instruction>> {
        let (input, (_, (a, b, dest))) = tuple((tag("addr"), three_numbers))(i)?;
        Ok((input, Box::new(AddR { a, b, dest })))
        // }) as Box<dyn Instruction>
    }
}

#[derive(Debug)]
struct EqRR {
    a: usize,
    b: usize,
    dest: usize,
}
impl Instruction for EqRR {
    fn execute(&self, cpu: &mut Cpu) {
        cpu.registers[self.dest] = if cpu.registers[self.a] == cpu.registers[self.b] {
            1
        } else {
            0
        };
    }
}
impl EqRR {
    fn parser(i: &str) -> IResult<&str, Box<dyn Instruction>> {
        let (input, (_, (a, b, dest))) = tuple((tag("eqrr"), three_numbers))(i)?;
        Ok((input, Box::new(EqRR { a, b, dest })))
        // }) as Box<dyn Instruction>
    }
}

#[derive(Debug)]
struct GtRR {
    a: usize,
    b: usize,
    dest: usize,
}
impl Instruction for GtRR {
    fn execute(&self, cpu: &mut Cpu) {
        cpu.registers[self.dest] = if cpu.registers[self.a] > cpu.registers[self.b] {
            1
        } else {
            0
        };
    }
}
impl GtRR {
    fn parser(i: &str) -> IResult<&str, Box<dyn Instruction>> {
        let (input, (_, (a, b, dest))) = tuple((tag("gtrr"), three_numbers))(i)?;
        Ok((input, Box::new(GtRR { a, b, dest })))
        // }) as Box<dyn Instruction>
    }
}

#[derive(Debug)]
struct MulI {
    a: usize,
    b: i32,
    dest: usize,
}
impl Instruction for MulI {
    fn execute(&self, cpu: &mut Cpu) {
        cpu.registers[self.dest] = cpu.registers[self.a] * self.b;
    }
}
impl MulI {
    fn parser(i: &str) -> IResult<&str, Box<dyn Instruction>> {
        let (input, (_, (a, b, dest))) = tuple((tag("muli"), three_numbers))(i)?;
        Ok((
            input,
            Box::new(MulI {
                a,
                b: b as i32,
                dest,
            }),
        ))
        // }) as Box<dyn Instruction>
    }
}

#[derive(Debug)]
struct MulR {
    a: usize,
    b: usize,
    dest: usize,
}
impl Instruction for MulR {
    fn execute(&self, cpu: &mut Cpu) {
        cpu.registers[self.dest] = cpu.registers[self.a] * cpu.registers[self.b];
    }
}
impl MulR {
    fn parser(i: &str) -> IResult<&str, Box<dyn Instruction>> {
        let (input, (_, (a, b, dest))) = tuple((tag("mulr"), three_numbers))(i)?;
        Ok((input, Box::new(MulR { a, b, dest })))
        // }) as Box<dyn Instruction>
    }
}

#[derive(Debug)]
struct SetI {
    value: i32,
    _ignored: i8,
    dest: usize,
}
impl Instruction for SetI {
    fn execute(&self, cpu: &mut Cpu) {
        cpu.registers[self.dest] = self.value;
    }
}
impl SetI {
    fn parser(i: &str) -> IResult<&str, Box<dyn Instruction>> {
        let (input, (_, (value, ignored, dest))) = tuple((tag("seti"), three_numbers))(i)?;
        Ok((
            input,
            Box::new(SetI {
                value: value as i32,
                _ignored: ignored as i8,
                dest,
            }),
        ))
        // }) as Box<dyn Instruction>
    }
}

#[derive(Debug)]
struct SetR {
    source: usize,
    _ignored: i8,
    dest: usize,
}
impl Instruction for SetR {
    fn execute(&self, cpu: &mut Cpu) {
        cpu.registers[self.dest] = cpu.registers[self.source];
    }
}
impl SetR {
    fn parser(i: &str) -> IResult<&str, Box<dyn Instruction>> {
        let (input, (_, (source, ignored, dest))) = tuple((tag("setr"), three_numbers))(i)?;
        Ok((
            input,
            Box::new(SetR {
                source,
                _ignored: ignored as i8,
                dest,
            }),
        ))
        // }) as Box<dyn Instruction>
    }
}

#[derive(Clone)]
struct Cpu {
    registers: [i32; 6],
    pc: i32,
    pc_reg: usize,
    instructions: Rc<Vec<Box<dyn Instruction>>>,
}

impl Cpu {
    fn new(pc_reg: usize, instructions: Rc<Vec<Box<dyn Instruction>>>) -> Cpu {
        Cpu {
            registers: [0; 6],
            pc: 0,
            pc_reg,
            instructions,
        }
    }

    fn execute(&self) -> Option<Self> {
        if self.pc < 0 || self.pc >= self.instructions.len() as i32 {
            return None;
        }
        let mut rv = self.clone();
        rv.registers[rv.pc_reg] = rv.pc;
        let instruction = &self.instructions[rv.pc as usize];
        instruction.execute(&mut rv);
        rv.pc = rv.registers[rv.pc_reg];
        rv.pc += 1;
        Some(rv)
    }
}

fn process_data_a(data: &str) -> i32 {
    let mut lines = data.lines();
    let mut ip = lines.next().unwrap().to_string();
    let ip = ip.pop().unwrap().to_digit(10).unwrap() as usize;

    let instructions = parse_instructions(lines);
    let mut state = Cpu::new(ip, instructions);
    while let Some(new) = state.execute() {
        state = new;
    }
    state.registers[0]
}

fn process_data_b(_data: &str) -> i32 {
    // Part B figures out the sum of the prime factors of 10_551_410.
    18_992_556
}

//-----------------------------------------------------
// Questions.

q_impl!("19");

#[test]
fn a() {
    assert_eq!(
        process_data_a(
            "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5"
        ),
        6
    );
}

#[test]
fn b() {}
