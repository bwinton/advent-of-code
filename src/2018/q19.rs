//-----------------------------------------------------
// Setup.

use combine::char::digit;
use combine::parser::char::string;
use combine::{many1, Parser};

use std::fmt::{Debug, Display, Formatter, Result};
use std::rc::Rc;
use std::result;

static INPUT: &'static str = include_str!("data/q19.data");

trait Instruction: Display + Debug {
    fn execute(&self, cpu: &mut CPU);
}

#[derive(Debug)]
pub struct InstructionError(());

type InstructionResult = result::Result<Box<Instruction>, InstructionError>;
type InstructionsResult = result::Result<Vec<Rc<Box<Instruction>>>, InstructionError>;

fn parse_instructions(
    s: &str,
    builders: &[fn(s: &str) -> InstructionResult],
) -> InstructionsResult {
    let mut instructions: Vec<Rc<Box<Instruction>>> = vec![];
    for line in s.lines() {
        let mut found = false;
        for builder in builders {
            if let Ok(inst) = builder(line) {
                found = true;
                instructions.push(Rc::new(inst));
                break;
            }
        }
        if !found {
            //Error!!!
            println!("Unknown instruction {}", line);
            return Err(InstructionError(()));
        }
    }
    Ok(instructions)
}

#[derive(Debug, Display)]
#[display(fmt = "addi {} {} {}", a, b, dest)]
struct AddI {
    a: usize,
    b: i32,
    dest: usize,
}
impl AddI {
    fn build(s: &str) -> InstructionResult {
        let result = string("addi ")
            .with(
                many1::<String, _>(digit())
                    .and(string(" ").with(many1::<String, _>(digit())))
                    .and(string(" ").with(many1::<String, _>(digit()))),
            )
            .parse(s)
            .map(|x| x.0);
        match result {
            Ok(((a, b), dest)) => {
                let a = a.parse().unwrap();
                let b = b.parse().unwrap();
                let dest = dest.parse().unwrap();
                Ok(Box::new(AddI { a, b, dest }))
            }
            _ => Err(InstructionError(())),
        }
    }
}
impl Instruction for AddI {
    fn execute(&self, cpu: &mut CPU) {
        cpu.registers[self.dest] = cpu.registers[self.a] + self.b;
    }
}

#[derive(Debug, Display)]
#[display(fmt = "addr {} {} {}", a, b, dest)]
struct AddR {
    a: usize,
    b: usize,
    dest: usize,
}
impl AddR {
    fn build(s: &str) -> InstructionResult {
        let result = string("addr ")
            .with(
                many1::<String, _>(digit())
                    .and(string(" ").with(many1::<String, _>(digit())))
                    .and(string(" ").with(many1::<String, _>(digit()))),
            )
            .parse(s)
            .map(|x| x.0);
        match result {
            Ok(((a, b), dest)) => {
                let a = a.parse().unwrap();
                let b = b.parse().unwrap();
                let dest = dest.parse().unwrap();
                Ok(Box::new(AddR { a, b, dest }))
            }
            _ => Err(InstructionError(())),
        }
    }
}
impl Instruction for AddR {
    fn execute(&self, cpu: &mut CPU) {
        cpu.registers[self.dest] = cpu.registers[self.a] + cpu.registers[self.b];
    }
}

#[derive(Debug, Display)]
#[display(fmt = "eqrr {} {} {}", a, b, dest)]
struct EqRR {
    a: usize,
    b: usize,
    dest: usize,
}
impl EqRR {
    fn build(s: &str) -> InstructionResult {
        let result = string("eqrr ")
            .with(
                many1::<String, _>(digit())
                    .and(string(" ").with(many1::<String, _>(digit())))
                    .and(string(" ").with(many1::<String, _>(digit()))),
            )
            .parse(s)
            .map(|x| x.0);
        match result {
            Ok(((a, b), dest)) => {
                let a = a.parse().unwrap();
                let b = b.parse().unwrap();
                let dest = dest.parse().unwrap();
                Ok(Box::new(EqRR { a, b, dest }))
            }
            _ => Err(InstructionError(())),
        }
    }
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

#[derive(Debug, Display)]
#[display(fmt = "gtrr {} {} {}", a, b, dest)]
struct GtRR {
    a: usize,
    b: usize,
    dest: usize,
}
impl GtRR {
    fn build(s: &str) -> InstructionResult {
        let result = string("gtrr ")
            .with(
                many1::<String, _>(digit())
                    .and(string(" ").with(many1::<String, _>(digit())))
                    .and(string(" ").with(many1::<String, _>(digit()))),
            )
            .parse(s)
            .map(|x| x.0);
        match result {
            Ok(((a, b), dest)) => {
                let a = a.parse().unwrap();
                let b = b.parse().unwrap();
                let dest = dest.parse().unwrap();
                Ok(Box::new(GtRR { a, b, dest }))
            }
            _ => Err(InstructionError(())),
        }
    }
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

#[derive(Debug, Display)]
#[display(fmt = "muli {} {} {}", a, b, dest)]
struct MulI {
    a: usize,
    b: i32,
    dest: usize,
}
impl MulI {
    fn build(s: &str) -> InstructionResult {
        let result = string("muli ")
            .with(
                many1::<String, _>(digit())
                    .and(string(" ").with(many1::<String, _>(digit())))
                    .and(string(" ").with(many1::<String, _>(digit()))),
            )
            .parse(s)
            .map(|x| x.0);
        match result {
            Ok(((a, b), dest)) => {
                let a = a.parse().unwrap();
                let b = b.parse().unwrap();
                let dest = dest.parse().unwrap();
                Ok(Box::new(MulI { a, b, dest }))
            }
            _ => Err(InstructionError(())),
        }
    }
}
impl Instruction for MulI {
    fn execute(&self, cpu: &mut CPU) {
        cpu.registers[self.dest] = cpu.registers[self.a] * self.b;
    }
}

#[derive(Debug, Display)]
#[display(fmt = "mulr {} {} {}", a, b, dest)]
struct MulR {
    a: usize,
    b: usize,
    dest: usize,
}
impl MulR {
    fn build(s: &str) -> InstructionResult {
        let result = string("mulr ")
            .with(
                many1::<String, _>(digit())
                    .and(string(" ").with(many1::<String, _>(digit())))
                    .and(string(" ").with(many1::<String, _>(digit()))),
            )
            .parse(s)
            .map(|x| x.0);
        match result {
            Ok(((a, b), dest)) => {
                let a = a.parse().unwrap();
                let b = b.parse().unwrap();
                let dest = dest.parse().unwrap();
                Ok(Box::new(MulR { a, b, dest }))
            }
            _ => Err(InstructionError(())),
        }
    }
}
impl Instruction for MulR {
    fn execute(&self, cpu: &mut CPU) {
        cpu.registers[self.dest] = cpu.registers[self.a] * cpu.registers[self.b];
    }
}

#[derive(Debug, Display)]
#[display(fmt = "seti {} {} {}", value, ignored, dest)]
struct SetI {
    value: i32,
    ignored: i8,
    dest: usize,
}
impl SetI {
    fn build(s: &str) -> InstructionResult {
        let result = string("seti ")
            .with(
                many1::<String, _>(digit())
                    .and(string(" ").with(many1::<String, _>(digit())))
                    .and(string(" ").with(many1::<String, _>(digit()))),
            )
            .parse(s)
            .map(|x| x.0);
        match result {
            Ok(((value, ignored), dest)) => {
                let value = value.parse().unwrap();
                let ignored = ignored.parse().unwrap();
                let dest = dest.parse().unwrap();
                Ok(Box::new(SetI {
                    value,
                    ignored,
                    dest,
                }))
            }
            _ => Err(InstructionError(())),
        }
    }
}
impl Instruction for SetI {
    fn execute(&self, cpu: &mut CPU) {
        cpu.registers[self.dest] = self.value;
    }
}

#[derive(Debug, Display)]
#[display(fmt = "setr {} {} {}", source, ignored, dest)]
struct SetR {
    source: usize,
    ignored: i8,
    dest: usize,
}
impl SetR {
    fn build(s: &str) -> InstructionResult {
        let result = string("setr ")
            .with(
                many1::<String, _>(digit())
                    .and(string(" ").with(many1::<String, _>(digit())))
                    .and(string(" ").with(many1::<String, _>(digit()))),
            )
            .parse(s)
            .map(|x| x.0);
        match result {
            Ok(((source, ignored), dest)) => {
                let source = source.parse().unwrap();
                let ignored = ignored.parse().unwrap();
                let dest = dest.parse().unwrap();
                Ok(Box::new(SetR {
                    source,
                    ignored,
                    dest,
                }))
            }
            _ => Err(InstructionError(())),
        }
    }
}
impl Instruction for SetR {
    fn execute(&self, cpu: &mut CPU) {
        cpu.registers[self.dest] = cpu.registers[self.source];
    }
}

#[derive(Clone)]
struct CPU {
    registers: [i32; 6],
    pc: i32,
    pc_reg: usize,
    instructions: Vec<Rc<Box<Instruction>>>,
}

impl CPU {
    fn new(pc_reg: usize, instructions: Vec<Rc<Box<Instruction>>>) -> CPU {
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

fn process_data_a(data: &str) -> i32 {
    let builders: Vec<fn(s: &str) -> InstructionResult> = vec![
        AddI::build,
        AddR::build,
        EqRR::build,
        GtRR::build,
        MulI::build,
        MulR::build,
        SetI::build,
        SetR::build,
    ];

    let mut lines = data.lines();
    let mut ip = lines.next().unwrap().to_string();
    let ip = ip.pop().unwrap().to_digit(10).unwrap() as usize;
    let instructions =
        parse_instructions(&lines.collect::<Vec<_>>().join("\n"), &builders).unwrap();
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
