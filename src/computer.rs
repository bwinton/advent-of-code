use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter, Result},
    rc::Rc,
};

use nom::{
    bytes::complete::tag,
    character::complete::i64,
    error::{Error, ErrorKind},
    sequence::tuple,
    Err::Failure,
    IResult,
};

use crate::nom_util::single_letter;

pub trait Instruction: Display + Debug {
    fn execute(&self, cpu: &mut CPU);
}

pub type InstructionResult<'a> = IResult<&'a str, Rc<dyn Instruction>>;
pub type InstructionsResult<'a> = IResult<&'a str, Vec<Rc<dyn Instruction>>>;

#[derive(Debug, Display)]
#[display("hlf {}", register)]
pub struct Half {
    register: char,
}
impl Half {
    pub fn build(i: &str) -> InstructionResult {
        let (input, (_, register)) = tuple((tag("hlf "), single_letter))(i)?;
        Ok((input, Rc::new(Self { register })))
    }
}
impl Instruction for Half {
    fn execute(&self, cpu: &mut CPU) {
        *cpu.registers.get_mut(&self.register).unwrap() /= 2;
    }
}

#[derive(Debug, Display)]
#[display("tpl {}", register)]
pub struct Triple {
    register: char,
}
impl Triple {
    pub fn build(i: &str) -> InstructionResult {
        let (input, (_, register)) = tuple((tag("tpl "), single_letter))(i)?;
        Ok((input, Rc::new(Self { register })))
    }
}
impl Instruction for Triple {
    fn execute(&self, cpu: &mut CPU) {
        *cpu.registers.get_mut(&self.register).unwrap() *= 3;
    }
}

#[derive(Debug, Display)]
#[display("inc {}", register)]
pub struct Increment {
    register: char,
}
impl Increment {
    pub fn build(i: &str) -> InstructionResult {
        let (input, (_, register)) = tuple((tag("inc "), single_letter))(i)?;
        Ok((input, Rc::new(Self { register })))
    }
}
impl Instruction for Increment {
    fn execute(&self, cpu: &mut CPU) {
        *cpu.registers.get_mut(&self.register).unwrap() += 1;
    }
}

#[derive(Debug, Display)]
#[display("jmp {}", offset)]
pub struct Jump {
    offset: i64,
}
impl Jump {
    pub fn build(i: &str) -> InstructionResult {
        let (input, (_, offset)) = tuple((tag("jmp "), i64))(i)?;
        Ok((input, Rc::new(Self { offset })))
    }
}
impl Instruction for Jump {
    fn execute(&self, cpu: &mut CPU) {
        cpu.pc += self.offset - 1
    }
}

#[derive(Debug, Display)]
#[display("jie {} {}", register, offset)]
pub struct JumpEven {
    register: char,
    offset: i64,
}
impl JumpEven {
    pub fn build(i: &str) -> InstructionResult {
        let (input, (_, register, _, offset)) =
            tuple((tag("jie "), single_letter, tag(", "), i64))(i)?;
        Ok((input, Rc::new(Self { register, offset })))
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
#[display("jio {} {}", register, offset)]
pub struct JumpOne {
    register: char,
    offset: i64,
}
impl JumpOne {
    pub fn build(i: &str) -> InstructionResult {
        let (input, (_, register, _, offset)) =
            tuple((tag("jio "), single_letter, tag(", "), i64))(i)?;
        Ok((input, Rc::new(Self { register, offset })))
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
    instructions: Vec<Rc<dyn Instruction>>,
}

impl CPU {
    pub fn new(registers: HashMap<char, i64>, instructions: Vec<Rc<dyn Instruction>>) -> CPU {
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

pub fn parse_instructions<'a>(
    s: &'a str,
    builders: &[fn(s: &str) -> InstructionResult],
) -> InstructionsResult<'a> {
    let mut instructions: Vec<Rc<dyn Instruction>> = vec![];
    for line in s.lines() {
        let mut found = false;
        for builder in builders {
            if let Ok(inst) = builder(line) {
                found = true;
                instructions.push(inst.1);
                break;
            }
        }
        if !found {
            //Error!!!
            println!("Unknown instruction {}", line);
            return Err(Failure(Error::new(line, ErrorKind::Alt)));
        }
    }
    Ok(("", instructions))
}
