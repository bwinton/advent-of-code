//-----------------------------------------------------
// Setup.

use itertools::Itertools;

use nom::{
    Err, IResult,
    bits::complete::{tag, take},
    branch::alt,
    error::{ErrorKind, make_error},
    sequence::tuple,
};

static INPUT: &str = include_str!("data/q16.data");

#[derive(Debug, Clone)]
struct InstructionV2 {
    version: u64,
    instruction_type: InstructionType,
}

impl InstructionV2 {
    fn get_versions(&self) -> u64 {
        let mut rv = self.version;
        match &self.instruction_type {
            InstructionType::Sum(sub_instructions)
            | InstructionType::Product(sub_instructions)
            | InstructionType::Minimum(sub_instructions)
            | InstructionType::Maximum(sub_instructions)
            | InstructionType::GreaterThan(sub_instructions)
            | InstructionType::LessThan(sub_instructions)
            | InstructionType::EqualTo(sub_instructions) => {
                for instruction in sub_instructions {
                    rv += instruction.get_versions();
                }
            }
            _ => {}
        }
        rv
    }

    fn evaluate(&self) -> u64 {
        self.instruction_type.evaluate()
    }
}

#[derive(Debug, Clone)]
enum InstructionType {
    Sum(Vec<InstructionV2>),
    Product(Vec<InstructionV2>),
    Minimum(Vec<InstructionV2>),
    Maximum(Vec<InstructionV2>),
    Literal(u64),
    GreaterThan(Vec<InstructionV2>),
    LessThan(Vec<InstructionV2>),
    EqualTo(Vec<InstructionV2>),
}

impl InstructionType {
    fn evaluate(&self) -> u64 {
        let mut rv = 0;
        match self {
            InstructionType::Sum(subs) => {
                for instruction in subs {
                    rv += instruction.evaluate();
                }
            }
            InstructionType::Product(subs) => {
                rv = 1;
                for instruction in subs {
                    rv *= instruction.evaluate();
                }
            }
            InstructionType::Minimum(subs) => {
                let mut temp = vec![];
                for instruction in subs {
                    temp.push(instruction.evaluate());
                }
                rv = *temp.iter().min().unwrap();
            }
            InstructionType::Maximum(subs) => {
                let mut temp = vec![];
                for instruction in subs {
                    temp.push(instruction.evaluate());
                }
                rv = *temp.iter().max().unwrap();
            }
            InstructionType::Literal(value) => {
                rv = *value;
            }
            InstructionType::GreaterThan(subs) => {
                let a = subs[0].evaluate();
                let b = subs[1].evaluate();
                rv = if a > b { 1 } else { 0 };
            }
            InstructionType::LessThan(subs) => {
                let a = subs[0].evaluate();
                let b = subs[1].evaluate();
                rv = if a < b { 1 } else { 0 };
            }
            InstructionType::EqualTo(subs) => {
                let a = subs[0].evaluate();
                let b = subs[1].evaluate();
                rv = if a == b { 1 } else { 0 };
            }
        }
        rv
    }
}

fn version(i: (&[u8], usize)) -> IResult<(&[u8], usize), u64> {
    take(3usize)(i)
}

fn operation(i: (&[u8], usize)) -> IResult<(&[u8], usize), Vec<InstructionV2>> {
    let (mut input, len_flag): (_, u32) = take(1usize)(i)?;
    if len_flag == 0 {
        let (next, length): (_, u32) = take(15usize)(input)?;
        let mut curr = next.0.len() * 8 + 8 - next.1;
        let target = curr - length as usize;
        input = next;
        // Parse until we get length number of bits…
        let mut subs = vec![];
        while curr > target {
            let (next, sub) = instruction(input)?;
            curr = next.0.len() * 8 + 8 - next.1;
            subs.push(sub);
            input = next;
        }
        Ok((input, subs))
    } else {
        let (next, length) = take(11usize)(input)?;
        input = next;
        let mut subs = vec![];
        for _ in 0..length {
            let (next, sub) = instruction(input)?;
            subs.push(sub);
            input = next;
        }
        Ok((input, subs))
    }
}

fn sum(i: (&[u8], usize)) -> IResult<(&[u8], usize), InstructionType> {
    let (input, (_, result)) = tuple((tag(0, 3usize), operation))(i)?;
    Ok((input, InstructionType::Sum(result)))
}

fn product(i: (&[u8], usize)) -> IResult<(&[u8], usize), InstructionType> {
    let (input, (_, result)) = tuple((tag(1, 3usize), operation))(i)?;
    Ok((input, InstructionType::Product(result)))
}

fn minimum(i: (&[u8], usize)) -> IResult<(&[u8], usize), InstructionType> {
    let (input, (_, result)) = tuple((tag(2, 3usize), operation))(i)?;
    Ok((input, InstructionType::Minimum(result)))
}

fn maximum(i: (&[u8], usize)) -> IResult<(&[u8], usize), InstructionType> {
    let (input, (_, result)) = tuple((tag(3, 3usize), operation))(i)?;
    Ok((input, InstructionType::Maximum(result)))
}

fn literal(i: (&[u8], usize)) -> IResult<(&[u8], usize), InstructionType> {
    let (mut input, _) = tag(4, 3usize)(i)?;
    let mut rv = 0;
    loop {
        let (next, len_flag): (_, u32) = take(1usize)(input)?;
        let (next, value): (_, u64) = take(4usize)(next)?;
        input = next;
        rv <<= 4;
        rv += value;
        if len_flag == 0 {
            break;
        }
    }
    Ok((input, InstructionType::Literal(rv)))
}

fn greater_than(i: (&[u8], usize)) -> IResult<(&[u8], usize), InstructionType> {
    let (input, (_, result)) = tuple((tag(5, 3usize), operation))(i)?;
    if result.len() != 2 {
        return Err(Err::Error(make_error(input, ErrorKind::Fail)));
    }
    Ok((input, InstructionType::GreaterThan(result)))
}

fn less_than(i: (&[u8], usize)) -> IResult<(&[u8], usize), InstructionType> {
    let (input, (_, result)) = tuple((tag(6, 3usize), operation))(i)?;
    if result.len() != 2 {
        return Err(Err::Error(make_error(input, ErrorKind::Fail)));
    }
    Ok((input, InstructionType::LessThan(result)))
}

fn equal_to(i: (&[u8], usize)) -> IResult<(&[u8], usize), InstructionType> {
    let (input, (_, result)) = tuple((tag(7, 3usize), operation))(i)?;
    if result.len() != 2 {
        return Err(Err::Error(make_error(input, ErrorKind::Fail)));
    }
    Ok((input, InstructionType::EqualTo(result)))
}

fn instruction(i: (&[u8], usize)) -> IResult<(&[u8], usize), InstructionV2> {
    let (input, (version, result)) = tuple((
        version,
        alt((
            sum,
            product,
            minimum,
            maximum,
            literal,
            greater_than,
            less_than,
            equal_to,
        )),
    ))(i)?;
    Ok((input, InstructionV2 {
        version,
        instruction_type: result,
    }))
}

fn parser(input: (&[u8], usize)) -> IResult<(&[u8], usize), InstructionV2> {
    instruction(input)
}

fn process_data_a(data: &str) -> u64 {
    let mut bits: Vec<u8> = vec![];
    let data = data.trim();
    for mut value in &data.chars().chunks(2) {
        let value = u8::from_str_radix(&value.join(""), 16).unwrap();
        bits.push(value);
    }
    let (_, instruction) = parser((&bits, 0)).unwrap();
    instruction.get_versions()
}

fn process_data_b(data: &str) -> u64 {
    let mut bits: Vec<u8> = vec![];
    let data = data.trim();
    for mut value in &data.chars().chunks(2) {
        let value = u8::from_str_radix(&value.join(""), 16).unwrap();
        bits.push(value);
    }
    let (_, result) = parser((&bits, 0)).unwrap();
    result.evaluate()
}

//-----------------------------------------------------
// Questions.

q_impl!("16");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_a(indoc!("8A004A801A8002F478")), 16);
    assert_eq!(process_data_a(indoc!("620080001611562C8802118E34")), 12);
    assert_eq!(process_data_a(indoc!("C0015000016115A2E0802F182340")), 23);
    assert_eq!(process_data_a(indoc!("A0016C880162017C3686B18A3D4780")), 31);
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b(indoc!("D2FE28")), 2021);
    assert_eq!(process_data_b(indoc!("C200B40A82")), 3);
    assert_eq!(process_data_b(indoc!("04005AC33890")), 54);
    assert_eq!(process_data_b(indoc!("880086C3E88112")), 7);
    assert_eq!(process_data_b(indoc!("CE00C43D881120")), 9);
    assert_eq!(process_data_b(indoc!("D8005AC2A8F0")), 1);
    assert_eq!(process_data_b(indoc!("F600BC2D8F")), 0);
    assert_eq!(process_data_b(indoc!("9C005AC2F8F0")), 0);
    assert_eq!(process_data_b(indoc!("9C0141080250320F1802104A08")), 1);
}
