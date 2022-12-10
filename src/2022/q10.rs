//-----------------------------------------------------
// Setup.

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

static INPUT: &str = include_str!("data/q10.data");

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i64),
}

impl Instruction {
    fn execute(&self, x: &mut i64) {
        match self {
            Instruction::Noop => {}
            Instruction::AddX(value) => {
                *x += value;
            }
        }
    }

    fn length(&self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        }
    }
}

fn noop(i: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("noop")(i)?;
    Ok((input, Instruction::Noop))
}

fn addx(i: &str) -> IResult<&str, Instruction> {
    let (input, value) = preceded(tag("addx "), complete::i64)(i)?;
    Ok((input, Instruction::AddX(value)))
}

fn instruction(i: &str) -> IResult<&str, Instruction> {
    let (input, rv) = alt((noop, addx))(i)?;
    Ok((input, rv))
}

fn parser(i: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, list) = separated_list1(line_ending, instruction)(i)?;
    Ok((input, list))
}

fn process_data_a(data: &str) -> i64 {
    let mut rv = 0;
    let mut x = 1;
    let mut cycle = 0;
    let instructions = parser(data).unwrap().1;
    for instruction in instructions {
        let time = instruction.length();
        for _ in 0..time {
            cycle += 1;
            if cycle % 40 == 20 {
                rv += cycle * x;
            }
        }
        instruction.execute(&mut x);
    }
    rv
}

fn process_data_b(data: &str) -> String {
    let mut rv = "".to_string();
    let mut x = 1;
    let mut cycle = 0;
    let instructions = parser(data).unwrap().1;

    for instruction in instructions {
        let time = instruction.length();
        for _ in 0..time {
            let test = cycle % 40;
            if test == x - 1 || test == x || test == x + 1 {
                rv.push('#');
            } else {
                rv.push('.');
            }
            if test % 40 == 39 {
                rv.push('\n');
            }
            cycle += 1;
        }
        instruction.execute(&mut x);
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("10");

#[test]
fn a() {
    assert_eq!(
        process_data_a(indoc!(
            "addx 15
    addx -11
    addx 6
    addx -3
    addx 5
    addx -1
    addx -8
    addx 13
    addx 4
    noop
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx -35
    addx 1
    addx 24
    addx -19
    addx 1
    addx 16
    addx -11
    noop
    noop
    addx 21
    addx -15
    noop
    noop
    addx -3
    addx 9
    addx 1
    addx -3
    addx 8
    addx 1
    addx 5
    noop
    noop
    noop
    noop
    noop
    addx -36
    noop
    addx 1
    addx 7
    noop
    noop
    noop
    addx 2
    addx 6
    noop
    noop
    noop
    noop
    noop
    addx 1
    noop
    noop
    addx 7
    addx 1
    noop
    addx -13
    addx 13
    addx 7
    noop
    addx 1
    addx -33
    noop
    noop
    noop
    addx 2
    noop
    noop
    noop
    addx 8
    noop
    addx -1
    addx 2
    addx 1
    noop
    addx 17
    addx -9
    addx 1
    addx 1
    addx -3
    addx 11
    noop
    noop
    addx 1
    noop
    addx 1
    noop
    noop
    addx -13
    addx -19
    addx 1
    addx 3
    addx 26
    addx -30
    addx 12
    addx -1
    addx 3
    addx 1
    noop
    noop
    noop
    addx -9
    addx 18
    addx 1
    addx 2
    noop
    noop
    addx 9
    noop
    noop
    noop
    addx -1
    addx 2
    addx -37
    addx 1
    addx 3
    noop
    addx 15
    addx -21
    addx 22
    addx -6
    addx 1
    noop
    addx 2
    addx 1
    noop
    addx -10
    noop
    noop
    addx 20
    addx 1
    addx 2
    addx 2
    addx -6
    addx -11
    noop
    noop
    noop
    "
        )),
        13140
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(indoc!(
            "addx 15
    addx -11
    addx 6
    addx -3
    addx 5
    addx -1
    addx -8
    addx 13
    addx 4
    noop
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx -35
    addx 1
    addx 24
    addx -19
    addx 1
    addx 16
    addx -11
    noop
    noop
    addx 21
    addx -15
    noop
    noop
    addx -3
    addx 9
    addx 1
    addx -3
    addx 8
    addx 1
    addx 5
    noop
    noop
    noop
    noop
    noop
    addx -36
    noop
    addx 1
    addx 7
    noop
    noop
    noop
    addx 2
    addx 6
    noop
    noop
    noop
    noop
    noop
    addx 1
    noop
    noop
    addx 7
    addx 1
    noop
    addx -13
    addx 13
    addx 7
    noop
    addx 1
    addx -33
    noop
    noop
    noop
    addx 2
    noop
    noop
    noop
    addx 8
    noop
    addx -1
    addx 2
    addx 1
    noop
    addx 17
    addx -9
    addx 1
    addx 1
    addx -3
    addx 11
    noop
    noop
    addx 1
    noop
    addx 1
    noop
    noop
    addx -13
    addx -19
    addx 1
    addx 3
    addx 26
    addx -30
    addx 12
    addx -1
    addx 3
    addx 1
    noop
    noop
    noop
    addx -9
    addx 18
    addx 1
    addx 2
    noop
    noop
    addx 9
    noop
    noop
    noop
    addx -1
    addx 2
    addx -37
    addx 1
    addx 3
    noop
    addx 15
    addx -21
    addx 22
    addx -6
    addx 1
    noop
    addx 2
    addx 1
    noop
    addx -10
    noop
    noop
    addx 20
    addx 1
    addx 2
    addx 2
    addx -6
    addx -11
    noop
    noop
    noop
    "
        )),
        indoc!(
            "##..##..##..##..##..##..##..##..##..##..
    ###...###...###...###...###...###...###.
    ####....####....####....####....####....
    #####.....#####.....#####.....#####.....
    ######......######......######......####
    #######.......#######.......#######.....
    "
        )
    );
}
