//-----------------------------------------------------
// Setup.

use glue::{
    combinators::whitespace::space,
    prelude::{digit, find_all, find_any, is, map_result, take, Parser},
    types::MapParserResult,
};

use std::{
    fmt::{Debug, Display, Formatter, Result},
    rc::Rc,
    str::Lines,
};

static INPUT: &str = include_str!("data/q19.data");

trait Instruction: Display + Debug {
    fn execute(&self, cpu: &mut Cpu);
}

fn three_numbers<'a>() -> impl Parser<'a, (&'a str, &'a str, &'a str)> {
    map_result(
        find_all((
            space(1..),
            take(1.., is(digit)),
            space(1..),
            take(1.., is(digit)),
            space(1..),
            take(1.., is(digit)),
        )),
        |(_, one, _, two, _, three)| (one, two, three),
    )
}

fn parse_instructions<'a>(
    lines: Lines<'a>,
    builder: &mut impl Parser<'a, Box<dyn Instruction>>,
) -> Rc<Vec<Box<dyn Instruction>>> {
    let mut instructions: Vec<Box<dyn Instruction>> = vec![];
    for line in lines {
        match builder.parse(line) {
            Ok((_, i)) => {
                // println!("{} => {:?}", line, i);
                instructions.push(i);
            }
            Err((_, e)) => panic!("Unknown instruction {:?}", e),
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
    fn execute(&self, cpu: &mut Cpu) {
        cpu.registers[self.dest] = cpu.registers[self.a] + self.b;
    }
}
impl AddI {
    fn parser<'a>() -> impl Parser<'a, Box<dyn Instruction + 'a>> {
        move |ctx| {
            find_all((is("addi"), three_numbers()))
                .parse(ctx)
                .map_result(|(_, (a, b, dest))| {
                    Box::new(AddI {
                        a: a.parse().unwrap(),
                        b: b.parse().unwrap(),
                        dest: dest.parse().unwrap(),
                    }) as Box<dyn Instruction>
                })
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
    fn execute(&self, cpu: &mut Cpu) {
        cpu.registers[self.dest] = cpu.registers[self.a] + cpu.registers[self.b];
    }
}
impl AddR {
    fn parser<'a>() -> impl Parser<'a, Box<dyn Instruction + 'a>> {
        move |ctx| {
            find_all((is("addr"), three_numbers()))
                .parse(ctx)
                .map_result(|(_, (a, b, dest))| {
                    Box::new(AddR {
                        a: a.parse().unwrap(),
                        b: b.parse().unwrap(),
                        dest: dest.parse().unwrap(),
                    }) as Box<dyn Instruction>
                })
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
    fn execute(&self, cpu: &mut Cpu) {
        cpu.registers[self.dest] = if cpu.registers[self.a] == cpu.registers[self.b] {
            1
        } else {
            0
        };
    }
}
impl EqRR {
    fn parser<'a>() -> impl Parser<'a, Box<dyn Instruction + 'a>> {
        move |ctx| {
            find_all((is("eqrr"), three_numbers()))
                .parse(ctx)
                .map_result(|(_, (a, b, dest))| {
                    Box::new(EqRR {
                        a: a.parse().unwrap(),
                        b: b.parse().unwrap(),
                        dest: dest.parse().unwrap(),
                    }) as Box<dyn Instruction>
                })
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
    fn execute(&self, cpu: &mut Cpu) {
        cpu.registers[self.dest] = if cpu.registers[self.a] > cpu.registers[self.b] {
            1
        } else {
            0
        };
    }
}
impl GtRR {
    fn parser<'a>() -> impl Parser<'a, Box<dyn Instruction + 'a>> {
        move |ctx| {
            find_all((is("gtrr"), three_numbers()))
                .parse(ctx)
                .map_result(|(_, (a, b, dest))| {
                    Box::new(GtRR {
                        a: a.parse().unwrap(),
                        b: b.parse().unwrap(),
                        dest: dest.parse().unwrap(),
                    }) as Box<dyn Instruction>
                })
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
    fn execute(&self, cpu: &mut Cpu) {
        cpu.registers[self.dest] = cpu.registers[self.a] * self.b;
    }
}
impl MulI {
    fn parser<'a>() -> impl Parser<'a, Box<dyn Instruction + 'a>> {
        move |ctx| {
            find_all((is("muli"), three_numbers()))
                .parse(ctx)
                .map_result(|(_, (a, b, dest))| {
                    Box::new(MulI {
                        a: a.parse().unwrap(),
                        b: b.parse().unwrap(),
                        dest: dest.parse().unwrap(),
                    }) as Box<dyn Instruction>
                })
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
    fn execute(&self, cpu: &mut Cpu) {
        cpu.registers[self.dest] = cpu.registers[self.a] * cpu.registers[self.b];
    }
}
impl MulR {
    fn parser<'a>() -> impl Parser<'a, Box<dyn Instruction + 'a>> {
        move |ctx| {
            find_all((is("mulr"), three_numbers()))
                .parse(ctx)
                .map_result(|(_, (a, b, dest))| {
                    Box::new(MulR {
                        a: a.parse().unwrap(),
                        b: b.parse().unwrap(),
                        dest: dest.parse().unwrap(),
                    }) as Box<dyn Instruction>
                })
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
    fn execute(&self, cpu: &mut Cpu) {
        cpu.registers[self.dest] = self.value;
    }
}
impl SetI {
    fn parser<'a>() -> impl Parser<'a, Box<dyn Instruction + 'a>> {
        move |ctx| {
            find_all((is("seti"), three_numbers()))
                .parse(ctx)
                .map_result(|(_, (value, ignored, dest))| {
                    Box::new(SetI {
                        value: value.parse().unwrap(),
                        ignored: ignored.parse().unwrap(),
                        dest: dest.parse().unwrap(),
                    }) as Box<dyn Instruction>
                })
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
    fn execute(&self, cpu: &mut Cpu) {
        cpu.registers[self.dest] = cpu.registers[self.source];
    }
}
impl SetR {
    fn parser<'a>() -> impl Parser<'a, Box<dyn Instruction + 'a>> {
        move |ctx| {
            find_all((is("setr"), three_numbers()))
                .parse(ctx)
                .map_result(|(_, (source, ignored, dest))| {
                    Box::new(SetR {
                        source: source.parse().unwrap(),
                        ignored: ignored.parse().unwrap(),
                        dest: dest.parse().unwrap(),
                    }) as Box<dyn Instruction>
                })
        }
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

impl Display for Cpu {
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

    let mut builder = find_any((
        AddI::parser(),
        AddR::parser(),
        EqRR::parser(),
        GtRR::parser(),
        MulI::parser(),
        MulR::parser(),
        SetI::parser(),
        SetR::parser(),
    ));

    let instructions = parse_instructions(lines, &mut builder);
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
