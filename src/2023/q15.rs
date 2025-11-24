//-----------------------------------------------------
// Setup.

use std::collections::HashMap;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    multi::separated_list1,
};

static INPUT: &str = include_str!("data/q15.data");

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    for value in data.trim().split(',') {
        let mut curr = 0;
        for c in value.chars() {
            curr += c as usize;
            curr *= 17;
            curr %= 256;
        }
        rv += curr;
    }
    rv
}

#[derive(Debug)]
enum Operation {
    Minus,
    Equal(usize),
}

fn minus(i: &str) -> IResult<&str, Operation> {
    let (input, _name) = tag("-").parse(i)?;
    Ok((input, Operation::Minus))
}

fn equal(i: &str) -> IResult<&str, Operation> {
    let (input, (_name, digits)) = (tag("="), digit1).parse(i)?;
    Ok((input, Operation::Equal(digits.parse().unwrap())))
}

fn instruction(i: &str) -> IResult<&str, (&str, usize, Operation)> {
    let (input, (name, operation)) = (alpha1, alt((minus, equal))).parse(i)?;
    let mut curr = 0;
    for c in name.chars() {
        if c == '=' || c == '-' {
            break;
        }
        curr += c as usize;
        curr *= 17;
        curr %= 256;
    }

    Ok((input, (name, curr, operation)))
}

fn parser(i: &str) -> IResult<&str, Vec<(&str, usize, Operation)>> {
    let (input, instructions) = separated_list1(tag(","), instruction).parse(i)?;
    Ok((input, instructions))
}

fn process_data_b(data: &str) -> usize {
    let mut boxes: HashMap<usize, Vec<(&str, usize)>> = HashMap::new();
    let operations = parser(data).unwrap().1;
    for (label, curr, op) in operations {
        match op {
            Operation::Minus => {
                if let Some(lenses) = boxes.get_mut(&curr) {
                    lenses.retain(|lens| lens.0 != label);
                }
            }
            Operation::Equal(focus) => {
                let lenses = boxes.entry(curr).or_default();
                let mut found = false;
                for lens in lenses.iter_mut() {
                    if lens.0 == label {
                        found = true;
                        lens.1 = focus;
                    }
                }
                if !found {
                    lenses.push((label, focus));
                }
            }
        }
    }

    let mut rv = 0;
    for (curr, lenses) in boxes {
        for (i, (_, focus)) in lenses.iter().enumerate() {
            rv += (curr + 1) * (i + 1) * focus;
        }
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("15");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
        )),
        1320
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
        )),
        145
    );
}
