//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q08.data");

use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending},
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};
use num_integer::lcm;

type Paths<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn directions(i: &str) -> IResult<&str, Vec<char>> {
    let (input, directions) = many1(alt((tag("L"), tag("R"))))(i)?;
    Ok((
        input,
        directions
            .iter()
            .map(|x| x.chars().next().unwrap())
            .collect(),
    ))
}

fn path(i: &str) -> IResult<&str, (&str, (&str, &str))> {
    // AAA = (BBB, CCC)

    let (input, (key, _, left, _, right, _)) = tuple((
        alphanumeric1,
        tag(" = ("),
        alphanumeric1,
        tag(", "),
        alphanumeric1,
        tag(")"),
    ))(i)?;
    Ok((input, (key, (left, right))))
}

fn paths(i: &str) -> IResult<&str, HashMap<&str, (&str, &str)>> {
    let (input, paths) = separated_list1(line_ending, path)(i)?;
    Ok((input, HashMap::from_iter(paths)))
}

fn parser(i: &str) -> IResult<&str, (Vec<char>, Paths)> {
    let (input, (directions, _, paths)) = tuple((directions, many1(line_ending), paths))(i)?;
    Ok((input, (directions, paths)))
}

fn process_data_a(data: &str) -> usize {
    let mut rv: usize = 0;
    let (directions, paths) = parser(data).unwrap().1;
    let mut current_node = "AAA";

    for direction in directions.iter().cycle() {
        let potentials = paths.get(current_node).unwrap();
        match direction {
            'L' => current_node = potentials.0,
            'R' => current_node = potentials.1,
            _ => panic!("Aaaaahhhh! Unknown direction! {}", direction),
        }
        rv += 1;
        if current_node == "ZZZ" {
            break;
        }
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let (directions, paths) = parser(data).unwrap().1;
    let current_nodes: Vec<&str> = paths
        .clone()
        .into_iter()
        .filter_map(|(key, _)| if key.ends_with('A') { Some(key) } else { None })
        .collect();

    let mut cycles = vec![];
    for current_node in current_nodes {
        let mut current_node = current_node;
        let mut cycle = 0;
        for direction in directions.iter().cycle() {
            let potentials = paths.get(current_node).unwrap();
            match direction {
                'L' => current_node = potentials.0,
                'R' => current_node = potentials.1,
                _ => panic!("Aaaaahhhh! Unknown direction! {}", direction),
            }
            cycle += 1;
            if current_node.ends_with('Z') {
                break;
            }
        }
        cycles.push(cycle);
    }
    cycles.into_iter().reduce(lcm).unwrap()
}

//-----------------------------------------------------
// Questions.

q_impl!("8");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)
    "
        )),
        2
    );

    assert_eq!(
        process_data_a(indoc!(
            "LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)
    "
        )),
        6
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)
    "
        )),
        6
    );
}
