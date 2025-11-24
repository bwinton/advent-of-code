//-----------------------------------------------------
// Setup.

use std::collections::VecDeque;

use mod_exp::mod_exp;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{i32, line_ending},
    multi::separated_list0,
};

static INPUT: &str = include_str!("data/q22.data");

#[derive(Clone, Debug)]
enum Instruction {
    Cut(i128),
    Deal(usize),
    NewStack,
}

// cut 902
// deal with increment 11
// deal into new stack

fn cut(i: &str) -> IResult<&str, Instruction> {
    let (input, (_, offset)) = (tag("cut "), i32).parse(i)?;
    Ok((input, Instruction::Cut(offset as i128)))
}

fn deal_with(i: &str) -> IResult<&str, Instruction> {
    let (input, (_, offset)) = (tag("deal with increment "), i32).parse(i)?;
    Ok((input, Instruction::Deal(offset as usize)))
}

fn new_stack(i: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("deal into new stack")(i)?;
    Ok((input, Instruction::NewStack))
}

fn instruction(i: &str) -> IResult<&str, Instruction> {
    let (input, result) = alt((cut, deal_with, new_stack)).parse(i)?;
    Ok((input, result))
}

fn parser(i: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions) = separated_list0(line_ending, instruction).parse(i)?;
    Ok((input, instructions))
}

fn deal_cards(data: &str, length: i128, iterations: usize) -> VecDeque<i128> {
    let instructions = parser(data).unwrap().1;
    let mut cards = VecDeque::new();
    for i in 0..length {
        cards.push_back(i);
    }
    for i in 0..iterations {
        if i > 0 && cards[2020] == 2020 {
            return VecDeque::from(vec![2020]);
        }

        for instruction in instructions.clone() {
            match instruction {
                Instruction::Cut(n) => {
                    let n = if n >= 0 {
                        n as usize
                    } else {
                        cards.len() - (-n as usize)
                    };
                    cards.rotate_left(n);
                }
                Instruction::Deal(n) => {
                    let mut table = cards.clone();
                    let mut i = 0;
                    while !cards.is_empty() {
                        table[i] = cards.pop_front().unwrap();
                        i += n;
                        i %= table.len();
                    }
                    cards = table;
                }
                Instruction::NewStack => {
                    cards = cards.into_iter().rev().collect();
                }
            }
        }
    }
    cards
}

fn process_data_a(data: &str) -> usize {
    deal_cards(data, 10_007, 0)
        .iter()
        .position(|x| x == &2019)
        .unwrap()

    // Not 9533…
    // Not 144…
}

// Convert to a linear equation ax + b
fn to_linear_equation<I>(input: I, length: i128) -> (i128, i128)
where
    I: IntoIterator<Item = Instruction>,
{
    let mut a = 1;
    let mut b = 0;
    for cmd in input {
        match cmd {
            Instruction::Deal(n) => {
                let n = mod_exp(n as i128, length - 2, length) as i128;
                a *= n;
                b *= n;
            }
            Instruction::Cut(n) => {
                b += n + length;
            }
            Instruction::NewStack => {
                a = -a;
                b = length - 1 - b;
            }
        }
        a %= length;
        b %= length;
    }
    (a, b)
}

fn deal_cards_b(data: &str, length: i128, iterations: i128, target: i128) -> i128 {
    let instructions = parser(data).unwrap().1.into_iter().rev();
    let (a, b) = to_linear_equation(instructions, length);

    // Applying the function n times simplifies to:
    // x * a^n + b * (a^n - 1) / (a-1)
    let term1 = target * mod_exp(a, iterations, length) % length;
    let tmp = (mod_exp(a, iterations, length) - 1) * mod_exp(a - 1, length - 2, length) % length;
    let term2 = b * tmp % length;
    (term1 + term2) % length
}

fn process_data_b(data: &str) -> i128 {
    deal_cards_b(data, 119_315_717_514_047, 101_741_582_076_661, 2020)

    // 58414781636390 is too high.
    // 82745271260243
}

//-----------------------------------------------------
// Questions.

q_impl!("22");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        deal_cards(
            "deal into new stack
",
            10,
            1
        ),
        [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
    );
    assert_eq!(
        deal_cards(
            "cut 3
", 10, 1
        ),
        [3, 4, 5, 6, 7, 8, 9, 0, 1, 2]
    );
    assert_eq!(
        deal_cards(
            "cut -4
", 10, 1
        ),
        [6, 7, 8, 9, 0, 1, 2, 3, 4, 5]
    );
    assert_eq!(
        deal_cards(
            "deal with increment 3
",
            10,
            1
        ),
        [0, 7, 4, 1, 8, 5, 2, 9, 6, 3]
    );

    assert_eq!(
        deal_cards(
            "deal with increment 7
deal into new stack
deal into new stack
",
            10,
            1
        ),
        [0, 3, 6, 9, 2, 5, 8, 1, 4, 7]
    );
    assert_eq!(
        deal_cards(
            "cut 6
deal with increment 7
deal into new stack
",
            10,
            1
        ),
        [3, 0, 7, 4, 1, 8, 5, 2, 9, 6]
    );
    assert_eq!(
        deal_cards(
            "deal with increment 7
deal with increment 9
cut -2
",
            10,
            1
        ),
        [6, 3, 0, 7, 4, 1, 8, 5, 2, 9]
    );
    assert_eq!(
        deal_cards(
            "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1
",
            10,
            1
        ),
        [9, 2, 5, 8, 1, 4, 7, 0, 3, 6]
    );
}

#[test]
fn b() {
    // assert_eq!(process_data_b(""), 0);
}
