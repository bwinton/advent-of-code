//-----------------------------------------------------
// Setup.

use aoc::Day;

use nom;
use std::collections::HashMap;
use std::str::FromStr;

static INPUT : &'static str = "jio a, +16
inc a
inc a
tpl a
tpl a
tpl a
inc a
inc a
tpl a
inc a
inc a
tpl a
tpl a
tpl a
inc a
jmp +23
tpl a
inc a
inc a
tpl a
inc a
inc a
tpl a
tpl a
inc a
inc a
tpl a
inc a
tpl a
inc a
tpl a
inc a
inc a
tpl a
inc a
tpl a
tpl a
inc a
jio a, +8
inc b
jie a, +4
tpl a
inc a
jmp +2
hlf a
jmp -7";

#[derive(Clone)]
#[derive(Debug)]
enum Instruction {
  Half(char),
  Triple(char),
  Increment(char),
  Jump(i64),
  JumpEven(char, i64),
  JumpOne(char, i64)
}

impl Instruction {
  fn execute(&self, state: &mut State) {
    match *self {
      Instruction::Half(reg) => {
        *state.registers.get_mut(&reg).unwrap() /= 2;
        state.pc += 1;
      },
      Instruction::Triple(reg) => {
        *state.registers.get_mut(&reg).unwrap() *= 3;
        state.pc += 1;
      },
      Instruction::Increment(reg) => {
        *state.registers.get_mut(&reg).unwrap() += 1;
        state.pc += 1;
      },
      Instruction::Jump(offset) => {
        state.pc += offset;
      },
      Instruction::JumpEven(reg, offset) => {
        state.pc += if state.registers[&reg] % 2 == 0 { offset } else { 1 }
      },
      Instruction::JumpOne(reg, offset) => {
        state.pc += if state.registers[&reg] == 1 { offset } else { 1 }
      },
    }
  }
}

#[derive(Clone)]
#[derive(Debug)]
struct State {
  registers: HashMap<char, i64>,
  pc: i64,
  instructions: Vec<Instruction>
}

impl State {
  fn execute(&self) -> Option<State> {
    if self.pc < 0 || self.pc >= self.instructions.len() as i64 {
      return None;
    }
    let mut rv = self.clone();
    let instruction = &self.instructions[rv.pc as usize];
    instruction.execute(&mut rv);
    // println!("{:?}\n  {:?}", instruction, rv);
    Some(rv)
  }
}

named!(offset_parser<&str, i64>, do_parse!(
  op: one_of!("+-") >>
  val: map_res!(nom::digit, i64::from_str) >>
  (if op == '+' {
    val
  } else {
    -val
  })
));

named!(half_parser<&str, Instruction>, do_parse!(
  tag!("hlf ") >>
  reg: one_of!("ab") >>
  (Instruction::Half(reg))
));

named!(triple_parser<&str, Instruction>, do_parse!(
  tag!("tpl ") >>
  reg: one_of!("ab") >>
  (Instruction::Triple(reg))
));
named!(increment_parser<&str, Instruction>, do_parse!(
  tag!("inc ") >>
  reg: one_of!("ab") >>
  (Instruction::Increment(reg))
));
named!(jump_parser<&str, Instruction>, do_parse!(
  tag!("jmp ") >>
  offset: offset_parser >>
  (Instruction::Jump(offset))
));
named!(jump_even_parser<&str, Instruction>, do_parse!(
  tag!("jie ") >>
  reg: one_of!("ab") >>
  tag!(", ") >>
  offset: offset_parser >>
  (Instruction::JumpEven(reg, offset))
));
named!(jump_one_parser<&str, Instruction>, do_parse!(
  tag!("jio ") >>
  reg: one_of!("ab") >>
  tag!(", ") >>
  offset: offset_parser >>
  (Instruction::JumpOne(reg, offset))
));

named!(instruction_parser<&str, Instruction>, do_parse!(
  instruction: alt!(half_parser | triple_parser | increment_parser |
    jump_parser | jump_even_parser | jump_one_parser ) >>
  (instruction)
));


named!(machine_parser<&str, Vec<Instruction>>, separated_list_complete!(tag!("\n"), instruction_parser));

fn process_data_a(data: &str, reg: char) -> i64 {
  let instructions = machine_parser(data).unwrap().1;
  let mut state = State {
    registers: hashmap!{ 'a' => 0, 'b' => 0 },
    pc: 0,
    instructions: instructions
  };
  while let Some(new) = state.execute() {
    state = new;
  }
  state.registers[&reg]
}

fn process_data_b(data: &str, reg: char) -> i64 {
  let instructions = machine_parser(data).unwrap().1;
  let mut state = State {
    registers: hashmap!{ 'a' => 1, 'b' => 0 },
    pc: 0,
    instructions: instructions
  };
  while let Some(new) = state.execute() {
    state = new;
  }
  state.registers[&reg]
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
    let result = process_data_a(INPUT, 'b');
    println!("Result = {}", result);
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    let result = process_data_b(INPUT, 'b');
    println!("Result = {}", result);
  }
}

#[test]
fn a() {
  assert_eq!(process_data_a("inc a
jio a, +2
tpl a
inc a", 'a'), 2);
}

#[test]
fn b() {
  // assert_eq!(process_data_b(""), 0);
}
