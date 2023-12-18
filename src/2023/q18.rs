//-----------------------------------------------------
// Setup.

use aoc::util::Direction;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{newline, space1, u32},
    multi::separated_list1,
    sequence::tuple,
    AsChar, IResult,
};

static INPUT: &str = include_str!("data/q18.data");

type Instruction = (Direction, usize);

fn direction(i: &str) -> IResult<&str, Direction> {
    let (input, value) = alt((tag("L"), tag("R"), tag("U"), tag("D")))(i)?;
    let direction = match value.chars().next().unwrap() {
        'L' => Direction::West,
        'R' => Direction::East,
        'U' => Direction::North,
        'D' => Direction::South,
        x => {
            panic!("Unknown character: {}", x);
        }
    };
    Ok((input, direction))
}

fn colour(i: &str) -> IResult<&str, Instruction> {
    let (input, (length, direction)) = tuple((
        take_while_m_n(5, 5, AsChar::is_hex_digit),
        take_while_m_n(1, 1, AsChar::is_hex_digit),
    ))(i)?;
    let direction = match direction {
        "0" => Direction::East,
        "1" => Direction::South,
        "2" => Direction::West,
        "3" => Direction::North,
        _ => {
            panic!("Unknown hex direction! {}", direction)
        }
    };
    let length = usize::from_str_radix(length, 16).unwrap();
    Ok((input, (direction, length)))
}

fn instruction(i: &str) -> IResult<&str, (Instruction, Instruction)> {
    let (input, (direction, _, length, _, _, colour, _)) =
        tuple((direction, space1, u32, space1, tag("(#"), colour, tag(")")))(i)?;
    Ok((input, ((direction, length as usize), colour)))
}

fn parser(i: &str) -> IResult<&str, Vec<(Instruction, Instruction)>> {
    let (input, instructions) = separated_list1(newline, instruction)(i)?;
    Ok((input, instructions))
}

fn get_result(instructions: &[Instruction]) -> usize {
    let mut curr = (0, 0);
    let mut area = 0;

    for &(direction, length) in instructions {
        let next = direction.move_pos(curr, length as i64, None, None).unwrap();
        area += curr.0 * next.1;
        area -= curr.1 * next.0;
        area += length as i64;
        curr = next;
    }

    area.unsigned_abs() as usize / 2 + 1
}

fn process_data_a(data: &str) -> usize {
    let instructions = parser(data).unwrap().1;

    let instructions: Vec<_> = instructions.into_iter().map(|(item, _)| item).collect();
    get_result(&instructions)
}

fn process_data_b(data: &str) -> usize {
    let instructions = parser(data).unwrap().1;

    let instructions: Vec<_> = instructions.into_iter().map(|(_, item)| item).collect();
    get_result(&instructions)
}

//-----------------------------------------------------
// Questions.

q_impl!("18");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "
    R 6 (#70c710)
    D 5 (#0dc571)
    L 2 (#5713f0)
    D 2 (#d2c081)
    R 2 (#59c680)
    D 2 (#411b91)
    L 5 (#8ceee2)
    U 2 (#caa173)
    L 1 (#1b58a2)
    U 2 (#caa171)
    R 2 (#7807d2)
    U 3 (#a77fa3)
    L 2 (#015232)
    U 2 (#7a21e3)
    "
        )),
        62
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "
    R 6 (#70c710)
    D 5 (#0dc571)
    L 2 (#5713f0)
    D 2 (#d2c081)
    R 2 (#59c680)
    D 2 (#411b91)
    L 5 (#8ceee2)
    U 2 (#caa173)
    L 1 (#1b58a2)
    U 2 (#caa171)
    R 2 (#7807d2)
    U 3 (#a77fa3)
    L 2 (#015232)
    U 2 (#7a21e3)
    "
        )),
        952408144115
    );
}
