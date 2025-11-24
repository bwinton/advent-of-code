//-----------------------------------------------------
// Setup.

use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{i64, newline, one_of},
    multi::{many1, separated_list1},
    sequence::preceded,
};

static INPUT: &str = include_str!("data/q17.data");

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Adv(i64),
    Bxl(i64),
    Bst(i64),
    Jnz(i64),
    Bxc(i64),
    Out(i64),
    Bdv(i64),
    Cdv(i64),
}

impl Instruction {
    fn new(op: i64, arg: i64) -> Self {
        match op {
            0 => Instruction::Adv(arg),
            1 => Instruction::Bxl(arg),
            2 => Instruction::Bst(arg),
            3 => Instruction::Jnz(arg),
            4 => Instruction::Bxc(arg),
            5 => Instruction::Out(arg),
            6 => Instruction::Bdv(arg),
            7 => Instruction::Cdv(arg),
            _ => {
                panic!("Unknown instruction! {}, {}", op, arg)
            }
        }
    }

    fn get_value(&self, arg: i64, registers: &[i64]) -> i64 {
        match arg {
            0..=3 => arg,
            4..=6 => registers[(arg - 4) as usize],
            _ => panic!("Invalid arg {}", arg),
        }
    }

    fn run(&self, ip: &mut usize, registers: &mut [i64], out: &mut Vec<i64>) {
        match self {
            Instruction::Adv(arg) => {
                registers[0] >>= self.get_value(*arg, registers);
            }
            Instruction::Bxl(arg) => {
                registers[1] ^= *arg;
            }
            Instruction::Bst(arg) => {
                registers[1] = self.get_value(*arg, registers) % 8;
            }
            Instruction::Jnz(arg) => {
                if registers[0] != 0 {
                    *ip = *arg as usize;
                    return;
                }
            }
            Instruction::Bxc(_arg) => {
                registers[1] ^= registers[2];
            }
            Instruction::Out(arg) => {
                out.push(self.get_value(*arg, registers) % 8);
            }
            Instruction::Bdv(arg) => {
                registers[1] = registers[0] >> self.get_value(*arg, registers);
            }
            Instruction::Cdv(arg) => {
                registers[2] = registers[0] >> self.get_value(*arg, registers);
            }
        }
        *ip += 2;
    }
}

fn register(i: &str) -> IResult<&str, i64> {
    // Register A: 62769524
    let (input, (_, _, _, register, _)) =
        (tag("Register "), one_of("ABC"), tag(": "), i64, newline).parse(i)?;
    Ok((input, register))
}

fn registers(i: &str) -> IResult<&str, Vec<i64>> {
    let (input, registers) = many1(register).parse(i)?;
    Ok((input, registers))
}

fn program(i: &str) -> IResult<&str, Vec<i64>> {
    let (input, program) = preceded(tag("Program: "), separated_list1(tag(","), i64)).parse(i)?;
    Ok((input, program))
}

fn parser(i: &str) -> IResult<&str, (Vec<i64>, Vec<i64>)> {
    let (input, (registers, _, program)) = (registers, newline, program).parse(i)?;
    Ok((input, (registers, program)))
}

fn process_data_a(data: &str) -> String {
    let mut rv: Vec<i64> = vec![];
    let (mut registers, program) = parser(data).unwrap().1;
    let mut ip = 0;
    while ip < program.len() {
        let instruction = Instruction::new(program[ip], program[ip + 1]);
        instruction.run(&mut ip, &mut registers, &mut rv);
    }
    rv.into_iter().map(|i| i.to_string()).join(",")
}

fn process_data_b(data: &str) -> i64 {
    let (registers, program) = parser(data).unwrap().1;
    let mut i = 0;
    // find each correct digit starting from the end.
    'outer: for digit in 0..program.len() {
        for x in 0.. {
            // Reset the computer.
            let mut output: Vec<i64> = vec![];
            let mut registers = registers.clone();
            registers[0] = x + i * 8;
            let mut ip = 0;
            // Run the computer.
            while ip < program.len() {
                let instruction = Instruction::new(program[ip], program[ip + 1]);
                instruction.run(&mut ip, &mut registers, &mut output);
            }
            // If we've found it, move our number up one (in octal), and add the amount we found.
            if output[..] == program[program.len() - digit - 1..] {
                i *= 8;
                i += x;
                continue 'outer;
            }
        }
    }
    i
}

//-----------------------------------------------------
// Questions.

q_impl!("17");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "
            Register A: 729
            Register B: 0
            Register C: 0

            Program: 0,1,5,4,3,0
            "
        )),
        "4,6,3,5,6,3,5,2,1,0"
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "
            Register A: 2024
            Register B: 0
            Register C: 0

            Program: 0,3,5,4,3,0
            "
        )),
        117440
    );
}
