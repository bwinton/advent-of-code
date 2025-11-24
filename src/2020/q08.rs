//-----------------------------------------------------
// Setup.

use nom::{
    Err::Failure,
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{i64, line_ending},
    combinator::eof,
    error::{Error, ErrorKind},
    multi::separated_list0,
    sequence::terminated,
};
use std::collections::HashSet;

static INPUT: &str = include_str!("data/q08.data");

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State<'a> {
    instructions: &'a [Instruction],
    pc: i64,
    acc: i64,
}

impl<'a> State<'a> {
    fn new(instructions: &'a [Instruction]) -> Self {
        State {
            instructions,
            pc: 0,
            acc: 0,
        }
    }
}

fn code(i: &str) -> IResult<&str, &str> {
    let (input, result) = alt((tag("acc"), tag("jmp"), tag("nop"))).parse(i)?;
    Ok((input, result))
}

fn instruction(i: &str) -> IResult<&str, Instruction> {
    let (input, (inst, _, value)) = (code, tag(" "), i64).parse(i)?;

    let result = match inst {
        "acc" => Instruction::Acc(value),
        "jmp" => Instruction::Jmp(value),
        "nop" => Instruction::Nop(value),
        _ => return Err(Failure(Error::new(i, ErrorKind::Alt))),
    };
    Ok((input, result))
}

fn parser(i: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions) =
        terminated(separated_list0(line_ending, instruction), eof).parse(i)?;
    Ok((input, instructions))
}

fn execute(state: &mut State) {
    let curr = state.instructions[state.pc as usize];
    match curr {
        Instruction::Acc(number) => {
            state.acc += number;
        }
        Instruction::Jmp(number) => {
            state.pc += number - 1;
        }
        Instruction::Nop(_) => {}
    }
    state.pc += 1;
}

fn process_data_a(data: &str) -> i64 {
    let instructions = parser(data).unwrap().1;
    let mut state = State::new(&instructions);
    let mut seen = HashSet::new();
    while !seen.contains(&state.pc) {
        seen.insert(state.pc);
        execute(&mut state);
    }
    state.acc
}

fn process_data_b(data: &str) -> i64 {
    let instructions = parser(data).unwrap().1;
    for index in 0..instructions.len() {
        let mut curr = instructions.clone();
        match instructions[index] {
            Instruction::Acc(_) => {
                continue;
            }
            Instruction::Jmp(number) => curr[index] = Instruction::Nop(number),
            Instruction::Nop(number) => curr[index] = Instruction::Jmp(number),
        }
        let mut state = State::new(&curr);
        let mut seen = HashSet::new();
        while !seen.contains(&state.pc) {
            seen.insert(state.pc);
            execute(&mut state);
            if (state.pc < 0) || (state.pc >= (curr.len() as i64)) {
                return state.acc;
            }
        }
    }
    -1
}

//-----------------------------------------------------
// Questions.

q_impl!("8");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(
            "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
        ),
        5
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(
            "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
        ),
        8
    );
}
