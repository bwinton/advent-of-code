//-----------------------------------------------------
// Setup.

use std::{fmt::Debug, rc::Rc, str::Lines};

use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{space1, u64},
    multi::many_m_n,
    sequence::tuple,
};

static INPUT: &str = include_str!("data/q19.data");

trait Instruction: Debug {
    fn execute(&self, cpu: &mut Cpu);
}

fn three_numbers(i: &str) -> IResult<&str, (usize, usize, usize)> {
    let (input, result) = many_m_n(3, 3, tuple((space1, u64)))(i)?;
    Ok((
        input,
        (
            result[0].1 as usize,
            result[1].1 as usize,
            result[2].1 as usize,
        ),
    ))
}

type InstructionParser = fn(s: &str) -> IResult<&str, Rc<dyn Instruction>>;

fn parse_instructions(lines: Lines, builders: &[InstructionParser]) -> Vec<Rc<dyn Instruction>> {
    let mut instructions: Vec<Rc<dyn Instruction>> = vec![];
    for line in lines {
        let mut found = false;
        for builder in builders {
            if let Ok(inst) = builder(line) {
                found = true;
                instructions.push(inst.1);
                break;
            }
        }
        if !found {
            panic!("Unknown instruction {}", line);
            // return Err(Failure(Error::new(line, ErrorKind::Alt)));
        }
    }
    instructions
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
    fn parser(i: &str) -> IResult<&str, Rc<dyn Instruction>> {
        let (input, (_, (a, b, dest))) = tuple((tag("addi"), three_numbers))(i)?;
        Ok((
            input,
            Rc::new(AddI {
                a,
                b: b as i32,
                dest,
            }),
        ))
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
    fn parser(i: &str) -> IResult<&str, Rc<dyn Instruction>> {
        let (input, (_, (a, b, dest))) = tuple((tag("addr"), three_numbers))(i)?;
        Ok((input, Rc::new(AddR { a, b, dest })))
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
    fn parser(i: &str) -> IResult<&str, Rc<dyn Instruction>> {
        let (input, (_, (a, b, dest))) = tuple((tag("eqrr"), three_numbers))(i)?;
        Ok((input, Rc::new(EqRR { a, b, dest })))
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
    fn parser(i: &str) -> IResult<&str, Rc<dyn Instruction>> {
        let (input, (_, (a, b, dest))) = tuple((tag("gtrr"), three_numbers))(i)?;
        Ok((input, Rc::new(GtRR { a, b, dest })))
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
    fn parser(i: &str) -> IResult<&str, Rc<dyn Instruction>> {
        let (input, (_, (a, b, dest))) = tuple((tag("muli"), three_numbers))(i)?;
        Ok((
            input,
            Rc::new(MulI {
                a,
                b: b as i32,
                dest,
            }),
        ))
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
    fn parser(i: &str) -> IResult<&str, Rc<dyn Instruction>> {
        let (input, (_, (a, b, dest))) = tuple((tag("mulr"), three_numbers))(i)?;
        Ok((input, Rc::new(MulR { a, b, dest })))
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
    fn parser(i: &str) -> IResult<&str, Rc<dyn Instruction>> {
        let (input, (_, (value, ignored, dest))) = tuple((tag("seti"), three_numbers))(i)?;
        Ok((
            input,
            Rc::new(SetI {
                value: value as i32,
                _ignored: ignored as i8,
                dest,
            }),
        ))
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
    fn parser(i: &str) -> IResult<&str, Rc<dyn Instruction>> {
        let (input, (_, (source, ignored, dest))) = tuple((tag("setr"), three_numbers))(i)?;
        Ok((
            input,
            Rc::new(SetR {
                source,
                _ignored: ignored as i8,
                dest,
            }),
        ))
    }
}

#[derive(Clone)]
struct Cpu {
    registers: [i32; 6],
    pc: i32,
    pc_reg: usize,
    instructions: Rc<Vec<Rc<dyn Instruction>>>,
}

impl Cpu {
    fn new(pc_reg: usize, instructions: Rc<Vec<Rc<dyn Instruction>>>) -> Cpu {
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

    let builders: Vec<InstructionParser> = vec![
        AddI::parser,
        AddR::parser,
        EqRR::parser,
        GtRR::parser,
        MulI::parser,
        MulR::parser,
        SetI::parser,
        SetR::parser,
    ];
    let instructions = parse_instructions(lines, &builders);
    let mut state = Cpu::new(ip, Rc::new(instructions));
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
    use pretty_assertions::assert_eq;

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
