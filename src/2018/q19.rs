//-----------------------------------------------------
// Setup.

use glue::prelude::{all, any, digit, is, literal, merge, one_or_more, Parser};

use std::fmt::{Debug, Display, Formatter, Result};
use std::rc::Rc;
use std::str::Lines;

static INPUT: &str = include_str!("data/q19.data");

trait Instruction: Display + Debug {
    fn execute(&self, cpu: &mut CPU);
}

fn three_numbers<'a>(
    instruction: &'static str,
) -> impl Parser<
    &'a str,
    (
        &'a str,
        &'a str,
        std::vec::Vec<&'a str>,
        &'a str,
        std::vec::Vec<&'a str>,
        &'a str,
    ),
> {
    all((
        literal(instruction),
        merge(one_or_more(is(digit))),
        one_or_more(literal(" ")),
        merge(one_or_more(is(digit))),
        one_or_more(literal(" ")),
        merge(one_or_more(is(digit))),
    ))
}

fn parse_instructions<'a>(
    lines: Lines<'a>,
    builder: &dyn Parser<&'a str, Box<dyn Instruction>>,
) -> Rc<Vec<Box<dyn Instruction>>> {
    let mut instructions: Vec<Box<dyn Instruction>> = vec![];
    for line in lines {
        match builder.parse(line) {
            Ok(i) => {
                // println!("{} => {:?}", line, i);
                instructions.push(i.0);
            }
            Err(e) => panic!("Unknown instruction {:?}", e),
        }
    }
    Rc::new(instructions)
}

#[derive(Debug, Display)]
#[display(fmt = "addi {} {} {}", a, b, dest)]
struct AddI {
    a: usize,
    b: i32,
    dest: usize,
}
impl Instruction for AddI {
    fn execute(&self, cpu: &mut CPU) {
        cpu.registers[self.dest] = cpu.registers[self.a] + self.b;
    }
}
impl AddI {
    fn parser<'a>() -> impl Parser<&'a str, Box<dyn Instruction + 'a>> {
        move |input: &'a str| {
            let parser = three_numbers("addi ");
            let (token, input) = parser.parse(input)?;
            let a = token.1.parse().unwrap();
            let b = token.3.parse().unwrap();
            let dest = token.5.parse().unwrap();
            let rv = Box::new(AddI { a, b, dest });
            Ok((rv as Box<dyn Instruction>, input))
        }
    }
}

#[derive(Debug, Display)]
#[display(fmt = "addr {} {} {}", a, b, dest)]
struct AddR {
    a: usize,
    b: usize,
    dest: usize,
}
impl Instruction for AddR {
    fn execute(&self, cpu: &mut CPU) {
        cpu.registers[self.dest] = cpu.registers[self.a] + cpu.registers[self.b];
    }
}
impl AddR {
    fn parser<'a>() -> impl Parser<&'a str, Box<dyn Instruction + 'a>> {
        move |input: &'a str| {
            let parser = three_numbers("addr ");
            let (token, input) = parser.parse(input)?;
            let a = token.1.parse().unwrap();
            let b = token.3.parse().unwrap();
            let dest = token.5.parse().unwrap();
            let rv = Box::new(AddR { a, b, dest });
            Ok((rv as Box<dyn Instruction>, input))
        }
    }
}

#[derive(Debug, Display)]
#[display(fmt = "eqrr {} {} {}", a, b, dest)]
struct EqRR {
    a: usize,
    b: usize,
    dest: usize,
}
impl Instruction for EqRR {
    fn execute(&self, cpu: &mut CPU) {
        cpu.registers[self.dest] = if cpu.registers[self.a] == cpu.registers[self.b] {
            1
        } else {
            0
        };
    }
}
impl EqRR {
    fn parser<'a>() -> impl Parser<&'a str, Box<dyn Instruction + 'a>> {
        move |input: &'a str| {
            let parser = three_numbers("eqrr ");
            let (token, input) = parser.parse(input)?;
            let a = token.1.parse().unwrap();
            let b = token.3.parse().unwrap();
            let dest = token.5.parse().unwrap();
            let rv = Box::new(EqRR { a, b, dest });
            Ok((rv as Box<dyn Instruction>, input))
        }
    }
}

#[derive(Debug, Display)]
#[display(fmt = "gtrr {} {} {}", a, b, dest)]
struct GtRR {
    a: usize,
    b: usize,
    dest: usize,
}
impl Instruction for GtRR {
    fn execute(&self, cpu: &mut CPU) {
        cpu.registers[self.dest] = if cpu.registers[self.a] > cpu.registers[self.b] {
            1
        } else {
            0
        };
    }
}
impl GtRR {
    fn parser<'a>() -> impl Parser<&'a str, Box<dyn Instruction + 'a>> {
        move |input: &'a str| {
            let parser = three_numbers("gtrr ");
            let (token, input) = parser.parse(input)?;
            let a = token.1.parse().unwrap();
            let b = token.3.parse().unwrap();
            let dest = token.5.parse().unwrap();
            let rv = Box::new(GtRR { a, b, dest });
            Ok((rv as Box<dyn Instruction>, input))
        }
    }
}

#[derive(Debug, Display)]
#[display(fmt = "muli {} {} {}", a, b, dest)]
struct MulI {
    a: usize,
    b: i32,
    dest: usize,
}
impl Instruction for MulI {
    fn execute(&self, cpu: &mut CPU) {
        cpu.registers[self.dest] = cpu.registers[self.a] * self.b;
    }
}
impl MulI {
    fn parser<'a>() -> impl Parser<&'a str, Box<dyn Instruction + 'a>> {
        move |input: &'a str| {
            let parser = three_numbers("muli ");
            let (token, input) = parser.parse(input)?;
            let a = token.1.parse().unwrap();
            let b = token.3.parse().unwrap();
            let dest = token.5.parse().unwrap();
            let rv = Box::new(MulI { a, b, dest });
            Ok((rv as Box<dyn Instruction>, input))
        }
    }
}

#[derive(Debug, Display)]
#[display(fmt = "mulr {} {} {}", a, b, dest)]
struct MulR {
    a: usize,
    b: usize,
    dest: usize,
}
impl Instruction for MulR {
    fn execute(&self, cpu: &mut CPU) {
        cpu.registers[self.dest] = cpu.registers[self.a] * cpu.registers[self.b];
    }
}
impl MulR {
    fn parser<'a>() -> impl Parser<&'a str, Box<dyn Instruction + 'a>> {
        move |input: &'a str| {
            let parser = three_numbers("mulr ");
            let (token, input) = parser.parse(input)?;
            let a = token.1.parse().unwrap();
            let b = token.3.parse().unwrap();
            let dest = token.5.parse().unwrap();
            let rv = Box::new(MulR { a, b, dest });
            Ok((rv as Box<dyn Instruction>, input))
        }
    }
}

#[derive(Debug, Display)]
#[display(fmt = "seti {} {} {}", value, ignored, dest)]
struct SetI {
    value: i32,
    ignored: i8,
    dest: usize,
}
impl Instruction for SetI {
    fn execute(&self, cpu: &mut CPU) {
        cpu.registers[self.dest] = self.value;
    }
}
impl SetI {
    fn parser<'a>() -> impl Parser<&'a str, Box<dyn Instruction + 'a>> {
        move |input: &'a str| {
            let parser = three_numbers("seti ");
            let (token, input) = parser.parse(input)?;
            let value = token.1.parse().unwrap();
            let ignored = token.3.parse().unwrap();
            let dest = token.5.parse().unwrap();
            let rv = Box::new(SetI {
                value,
                ignored,
                dest,
            });
            Ok((rv as Box<dyn Instruction>, input))
        }
    }
}

#[derive(Debug, Display)]
#[display(fmt = "setr {} {} {}", source, ignored, dest)]
struct SetR {
    source: usize,
    ignored: i8,
    dest: usize,
}
impl Instruction for SetR {
    fn execute(&self, cpu: &mut CPU) {
        cpu.registers[self.dest] = cpu.registers[self.source];
    }
}
impl SetR {
    fn parser<'a>() -> impl Parser<&'a str, Box<dyn Instruction + 'a>> {
        move |input: &'a str| {
            let parser = three_numbers("setr ");
            let (token, input) = parser.parse(input)?;
            let source = token.1.parse().unwrap();
            let ignored = token.3.parse().unwrap();
            let dest = token.5.parse().unwrap();
            let rv = Box::new(SetR {
                source,
                ignored,
                dest,
            });
            Ok((rv as Box<dyn Instruction>, input))
        }
    }
}

#[derive(Clone)]
struct CPU {
    registers: [i32; 6],
    pc: i32,
    pc_reg: usize,
    instructions: Rc<Vec<Box<dyn Instruction>>>,
}

impl CPU {
    fn new(pc_reg: usize, instructions: Rc<Vec<Box<dyn Instruction>>>) -> CPU {
        CPU {
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

impl Display for CPU {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "{:?}", self.registers)?;
        for (pc, inst) in self.instructions.iter().enumerate() {
            writeln!(
                f,
                "{} {}",
                if (pc as i32) == self.pc { "->" } else { "  " },
                inst
            )?
        }
        Ok(())
    }
}

fn process_data_a(data: &'static str) -> i32 {
    let mut lines = data.lines();
    let mut ip = lines.next().unwrap().to_string();
    let ip = ip.pop().unwrap().to_digit(10).unwrap() as usize;

    let builder = any((
        AddI::parser(),
        AddR::parser(),
        EqRR::parser(),
        GtRR::parser(),
        MulI::parser(),
        MulR::parser(),
        SetI::parser(),
        SetR::parser(),
    ));

    let instructions = parse_instructions(lines, &builder);
    let mut state = CPU::new(ip, instructions);
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
