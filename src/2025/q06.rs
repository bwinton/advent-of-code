//-----------------------------------------------------
// Setup.

use std::iter::zip;

use nom::{
    IResult, Parser,
    character::complete::{newline, one_of, space0, space1, u64 as value_parser},
    multi::separated_list1,
    sequence::{preceded, terminated},
};

static INPUT: &str = include_str!("data/q06.data");

type Value = u64;

#[derive(Debug)]
enum Operator {
    Plus,
    Times,
}

fn numbers(i: &str) -> IResult<&str, Vec<Vec<Value>>> {
    let (input, (numbers, _)) = (
        separated_list1(
            newline,
            preceded(
                space0,
                terminated(separated_list1(space1, value_parser), space0),
            ),
        ),
        newline,
    )
        .parse(i)?;
    Ok((input, numbers))
}

fn operator(i: &str) -> IResult<&str, Operator> {
    let (input, op_char) = one_of("+*").parse(i)?;
    let operator = match op_char {
        '+' => Operator::Plus,
        '*' => Operator::Times,
        _ => {
            panic!("Unknown operator! {op_char}");
        }
    };
    Ok((input, operator))
}

fn operators(i: &str) -> IResult<&str, Vec<Operator>> {
    let (input, (operators, _)) = (
        preceded(
            space0,
            terminated(separated_list1(space1, operator), space0),
        ),
        newline,
    )
        .parse(i)?;
    Ok((input, operators))
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn parse(i: &str) -> IResult<&str, (Vec<Vec<Value>>, Vec<Operator>)> {
    let (input, (numbers, operators)) = (numbers, operators).parse(i)?;
    let numbers = transpose(numbers);

    Ok((input, (numbers, operators)))
}

fn process_data_a(data: &str) -> Value {
    let mut rv = 0;
    let (_, (numbers, operators)) = parse(data).unwrap();
    for (values, op) in zip(numbers, operators) {
        match op {
            Operator::Plus => {
                rv += values.iter().sum::<Value>();
            }
            Operator::Times => {
                rv += values.iter().product::<Value>();
            }
        }
    }
    rv
}

fn process_data_b(data: &str) -> Value {
    let mut rv = 0;
    let mut chars = vec![];
    for line in data.lines() {
        let mut next = vec![];
        for cell in line.bytes() {
            next.push(cell)
        }
        chars.push(next);
    }
    let mut op = None;
    let mut values = vec![];
    for i in 0..chars[0].len() {
        let mut all_blanks = true;
        let mut curr: Value = 0;
        for row in &chars {
            match row[i] {
                c if c.is_ascii_digit() => {
                    // do something.
                    curr *= 10;
                    curr += (c - b'0') as Value;
                    all_blanks = false;
                }
                b'+' => {
                    op = Some(Operator::Plus);
                    all_blanks = false;
                }
                b'*' => {
                    op = Some(Operator::Times);
                    all_blanks = false;
                }
                b' ' => {} // pass
                _ => {
                    panic!("Unknown char! {}", row[i]);
                }
            }
        }
        if curr != 0 {
            values.push(curr);
        }
        if all_blanks || i == chars[0].len() - 1 {
            match op.expect("We really need an operator here!") {
                Operator::Plus => {
                    rv += values.iter().sum::<Value>();
                }
                Operator::Times => {
                    rv += values.iter().product::<Value>();
                }
            }
            op = None;
            values = vec![];
        }
    }

    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("6");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"
        )),
        4277556
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"
        )),
        3263827
    );
}
