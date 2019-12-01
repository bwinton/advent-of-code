use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Result};
use std::rc::Rc;
use std::result;

use combine::char::{digit, letter};
use combine::parser::char::string;
use combine::{many1, one_of, Parser};

pub trait Instruction: Display + Debug {
    fn execute(&self, cpu: &mut CPU);
}

#[derive(Debug)]
pub struct InstructionError(());

pub type InstructionResult = result::Result<Box<dyn Instruction>, InstructionError>;
pub type InstructionsResult = result::Result<Vec<Rc<Box<dyn Instruction>>>, InstructionError>;

#[derive(Debug, Display)]
#[display(fmt = "hlf {}", register)]
pub struct Half {
    register: char,
}
impl Half {
    pub fn build(s: &str) -> InstructionResult {
        let result = string("hlf ").with(letter()).parse(s).map(|x| x.0);
        match result {
            Ok(reg) => Ok(Box::new(Half { register: reg })),
            _ => Err(InstructionError(())),
        }
    }
}
impl Instruction for Half {
    fn execute(&self, cpu: &mut CPU) {
        *cpu.registers.get_mut(&self.register).unwrap() /= 2;
    }
}

#[derive(Debug, Display)]
#[display(fmt = "tpl {}", register)]
pub struct Triple {
    register: char,
}
impl Triple {
    pub fn build(s: &str) -> InstructionResult {
        let result = string("tpl ").with(letter()).parse(s).map(|x| x.0);
        match result {
            Ok(reg) => Ok(Box::new(Triple { register: reg })),
            _ => Err(InstructionError(())),
        }
    }
}
impl Instruction for Triple {
    fn execute(&self, cpu: &mut CPU) {
        *cpu.registers.get_mut(&self.register).unwrap() *= 3;
    }
}

#[derive(Debug, Display)]
#[display(fmt = "inc {}", register)]
pub struct Increment {
    register: char,
}
impl Increment {
    pub fn build(s: &str) -> InstructionResult {
        let result = string("inc ").with(letter()).parse(s).map(|x| x.0);
        match result {
            Ok(reg) => Ok(Box::new(Increment { register: reg })),
            _ => Err(InstructionError(())),
        }
    }
}
impl Instruction for Increment {
    fn execute(&self, cpu: &mut CPU) {
        *cpu.registers.get_mut(&self.register).unwrap() += 1;
    }
}

#[derive(Debug, Display)]
#[display(fmt = "jmp {}", offset)]
pub struct Jump {
    offset: i64,
}
impl Jump {
    pub fn build(s: &str) -> InstructionResult {
        let result = string("jmp ")
            .with(one_of("+-".chars()).and(many1::<String, _>(digit())))
            .parse(s)
            .map(|x| x.0);
        match result {
            Ok((sign, value)) => {
                let mut offset: i64 = value.parse().unwrap();
                if sign == '-' {
                    offset = -offset;
                }
                Ok(Box::new(Jump { offset }))
            }
            _ => Err(InstructionError(())),
        }
    }
}
impl Instruction for Jump {
    fn execute(&self, cpu: &mut CPU) {
        cpu.pc += self.offset - 1
    }
}

#[derive(Debug, Display)]
#[display(fmt = "jie {} {}", register, offset)]
pub struct JumpEven {
    register: char,
    offset: i64,
}
impl JumpEven {
    pub fn build(s: &str) -> InstructionResult {
        let result = string("jie ")
            .with(
                letter()
                    .and(string(", ").with(one_of("+-".chars()).and(many1::<String, _>(digit())))),
            )
            .parse(s)
            .map(|x| x.0);
        match result {
            Ok((reg, (sign, value))) => {
                let mut offset: i64 = value.parse().unwrap();
                if sign == '-' {
                    offset = -offset;
                }
                Ok(Box::new(JumpEven {
                    register: reg,
                    offset,
                }))
            }
            _ => Err(InstructionError(())),
        }
    }
}
impl Instruction for JumpEven {
    fn execute(&self, cpu: &mut CPU) {
        cpu.pc += if cpu.registers[&self.register] % 2 == 0 {
            self.offset - 1
        } else {
            0
        }
    }
}

#[derive(Debug, Display)]
#[display(fmt = "jio {} {}", register, offset)]
pub struct JumpOne {
    register: char,
    offset: i64,
}
impl JumpOne {
    pub fn build(s: &str) -> InstructionResult {
        let result = string("jio ")
            .with(
                letter()
                    .and(string(", ").with(one_of("+-".chars()).and(many1::<String, _>(digit())))),
            )
            .parse(s)
            .map(|x| x.0);
        match result {
            Ok((reg, (sign, value))) => {
                let mut offset: i64 = value.parse().unwrap();
                if sign == '-' {
                    offset = -offset;
                }
                Ok(Box::new(JumpOne {
                    register: reg,
                    offset,
                }))
            }
            _ => Err(InstructionError(())),
        }
    }
}
impl Instruction for JumpOne {
    fn execute(&self, cpu: &mut CPU) {
        cpu.pc += if cpu.registers[&self.register] == 1 {
            self.offset - 1
        } else {
            0
        }
    }
}

#[derive(Clone)]
pub struct CPU {
    registers: HashMap<char, i64>,
    pc: i64,
    instructions: Vec<Rc<Box<dyn Instruction>>>,
}

impl CPU {
    pub fn new(registers: HashMap<char, i64>, instructions: Vec<Rc<Box<dyn Instruction>>>) -> CPU {
        CPU {
            registers,
            pc: 0,
            instructions,
        }
    }

    pub fn execute(&self) -> Option<Self> {
        if self.pc < 0 || self.pc >= self.instructions.len() as i64 {
            return None;
        }
        let mut rv = self.clone();
        let instruction = &self.instructions[rv.pc as usize];
        instruction.execute(&mut rv);
        rv.pc += 1;
        Some(rv)
    }

    pub fn get_register(&self, register: char) -> i64 {
        self.registers[&register]
    }
}

impl Display for CPU {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "{:?}", self.registers)?;
        for (pc, inst) in self.instructions.iter().enumerate() {
            writeln!(
                f,
                "{} {}",
                if (pc as i64) == self.pc { "->" } else { "  " },
                inst
            )?
        }
        Ok(())
    }
}

pub fn parse_instructions(
    s: &str,
    builders: &[fn(s: &str) -> InstructionResult],
) -> InstructionsResult {
    let mut instructions: Vec<Rc<Box<dyn Instruction>>> = vec![];
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
