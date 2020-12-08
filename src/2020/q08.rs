//-----------------------------------------------------
// Setup.

use glue::prelude::{digit, eoi, find_all, find_any, find_separated, is, optional, take, Parser};
use glue::types::MapParserResult;
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

fn number_parser<'a>() -> impl Parser<'a, i64> {
    move |ctx| {
        find_all((optional(is('+')), optional(is('-')), take(1.., is(digit))))
            .parse(ctx)
            .map_result(|(_, minus, digits)| {
                let mut rv: i64 = digits.parse().unwrap();
                if minus.is_some() {
                    rv = -rv;
                }
                rv
            })
    }
}

fn instruction_parser<'a>() -> impl Parser<'a, Instruction> {
    move |ctx| {
        find_all((
            find_any((
                is("acc"),
                is("jmp"),
                is("nop"),
            )),
            is(' '),
            number_parser()
        ))
        .parse(ctx)
        .map_result(|(inst, _, number)| match inst {
            "acc" => Instruction::Acc(number),
            "jmp" => Instruction::Jmp(number),
            "nop" => Instruction::Nop(number),
            x => panic!("Unknown instruction {:?}", x)
        })
    }
}

fn parser<'a>() -> impl Parser<'a, Vec<Instruction>> {
    move |ctx| {
        find_all((find_separated(1.., instruction_parser(), is('\n')), eoi()))
            .parse(ctx)
            .map_result(|(instructions, _)| instructions)
    }
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
    let instructions = parser().parse(data).unwrap().1;
    let mut state = State::new(&instructions);
    let mut seen = HashSet::new();
    while !seen.contains(&state.pc) {
        seen.insert(state.pc);
        execute(&mut state);
    }
    state.acc
    // println!("{:?}", instructions);
}

fn process_data_b(data: &str) -> i64 {
    let instructions = parser().parse(data).unwrap().1;
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
