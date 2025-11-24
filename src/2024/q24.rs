//-----------------------------------------------------
// Setup.

use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline},
    multi::many1,
};

static INPUT: &str = include_str!("data/q24.data");

type Initial<'a> = HashMap<&'a str, bool>;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Connection<'a> {
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    Xor(&'a str, &'a str),
}

fn gate(i: &str) -> IResult<&str, (&str, bool)> {
    // x00: 1
    let (input, (gate, _, value, _)) =
        (alphanumeric1, tag(": "), alt((tag("0"), tag("1"))), newline).parse(i)?;
    Ok((input, (gate, value == "1")))
}

fn gates(i: &str) -> IResult<&str, Initial> {
    let (input, gates) = many1(gate).parse(i)?;
    Ok((input, gates.into_iter().collect()))
}

fn connection(i: &str) -> IResult<&str, (&str, Connection)> {
    // x00 AND y00 -> z00

    let (input, (a, op, b, _, c, _)) = (
        alphanumeric1,
        alt((tag(" AND "), tag(" OR "), tag(" XOR "))),
        alphanumeric1,
        tag(" -> "),
        alphanumeric1,
        newline,
    )
        .parse(i)?;
    let (a, b) = if a < b { (a, b) } else { (b, a) };
    let connection = match op {
        " AND " => Connection::And(a, b),
        " OR " => Connection::Or(a, b),
        " XOR " => Connection::Xor(a, b),
        _ => panic!("Unknown operator: {}", op.trim()),
    };
    Ok((input, (c, connection)))
}

fn connections(i: &str) -> IResult<&str, HashMap<&str, Connection>> {
    let (input, connections) = many1(connection).parse(i)?;
    Ok((input, connections.into_iter().collect()))
}

fn parser(i: &str) -> IResult<&str, (Initial, HashMap<&str, Connection>)> {
    let (input, (initial, _, connections)) = (gates, newline, connections).parse(i)?;
    Ok((input, (initial, connections)))
}

fn get_value(value: &str, initial: &Initial, connections: &HashMap<&str, Connection>) -> bool {
    if initial.contains_key(value) {
        return initial[value];
    }
    let connection = connections.get(value).unwrap();
    match connection {
        Connection::And(a, b) => {
            let a = get_value(a, initial, connections);
            let b = get_value(b, initial, connections);
            a & b
        }
        Connection::Or(a, b) => {
            let a = get_value(a, initial, connections);
            let b = get_value(b, initial, connections);
            a | b
        }
        Connection::Xor(a, b) => {
            let a = get_value(a, initial, connections);
            let b = get_value(b, initial, connections);
            a ^ b
        }
    }
}

fn get_values<'a, T>(start: &str, connections: &HashMap<&'a str, T>) -> Vec<&'a str> {
    connections
        .keys()
        .filter(|&k| k.starts_with(start))
        .sorted()
        .rev()
        .cloned()
        .collect()
}

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    let (initial, connections) = parser(data).unwrap().1;
    let zs = get_values("z", &connections);
    for z in zs {
        rv <<= 1;
        if get_value(z, &initial, &connections) {
            rv += 1;
        }
    }
    rv
}

fn process_data_b(data: &str) -> String {
    let mut rv = vec![];
    let (_initial, connections) = parser(data).unwrap().1;
    let zs = get_values("z", &connections);
    let last_bit = zs[0];
    for (gate, connection) in connections.clone() {
        match (gate, connection) {
            ("z00", Connection::And("x00", "y00")) => {}, 
            (gate, Connection::Or(_, _)) if gate == last_bit => {},
            (gate, Connection::And(_, _) | Connection::Or(_, _)) if gate.starts_with("z") => {
                rv.push(gate)
            },
            (gate, Connection::Xor(x, y)) if x.starts_with("x") && y.starts_with("y") &&
                x != "x00" && y != "y00" => {
                if !connections
                    .values()
                    .any(|connection| matches!(connection, &Connection::And(a, b) if a == gate || b == gate))
                {
                    rv.push(gate);
                }
            },
            (_, Connection::Xor(x, y)) if x.starts_with("x") && y.starts_with("y") => {},
            (gate, Connection::Xor(_, _)) if gate.starts_with('z') => {},
            (gate, Connection::Xor(_, _)) => {
                rv.push(gate)
            },
            (gate, Connection::And(x, y)) if x.starts_with("x") && y.starts_with("y") &&
                x != "x00" && y != "y00" => {
                if !connections
                    .values()
                    .any(|connection| matches!(connection, &Connection::Or(a, b) if a == gate || b == gate))
                {
                    rv.push(gate);
                }
            },
            (_, _) => {}
        }
    }
    rv.sort();
    rv.join(",")
}

//-----------------------------------------------------
// Questions.

q_impl!("24");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "
        x00: 1
        x01: 1
        x02: 1
        y00: 0
        y01: 1
        y02: 0

        x00 AND y00 -> z00
        x01 XOR y01 -> z01
        x02 OR y02 -> z02
        "
        )),
        4
    );

    assert_eq!(
        process_data_a(indoc!(
            "
    x00: 1
    x01: 0
    x02: 1
    x03: 1
    x04: 0
    y00: 1
    y01: 1
    y02: 1
    y03: 1
    y04: 1

    ntg XOR fgs -> mjb
    y02 OR x01 -> tnw
    kwq OR kpj -> z05
    x00 OR x03 -> fst
    tgd XOR rvg -> z01
    vdt OR tnw -> bfw
    bfw AND frj -> z10
    ffh OR nrd -> bqk
    y00 AND y03 -> djm
    y03 OR y00 -> psh
    bqk OR frj -> z08
    tnw OR fst -> frj
    gnj AND tgd -> z11
    bfw XOR mjb -> z00
    x03 OR x00 -> vdt
    gnj AND wpb -> z02
    x04 AND y00 -> kjc
    djm OR pbm -> qhw
    nrd AND vdt -> hwm
    kjc AND fst -> rvg
    y04 OR y02 -> fgs
    y01 AND x02 -> pbm
    ntg OR kjc -> kwq
    psh XOR fgs -> tgd
    qhw XOR tgd -> z09
    pbm OR djm -> kpj
    x03 XOR y03 -> ffh
    x00 XOR y04 -> ntg
    bfw OR bqk -> z06
    nrd XOR fgs -> wpb
    frj XOR qhw -> z04
    bqk OR frj -> z07
    y03 OR x01 -> nrd
    hwm AND bqk -> z03
    tgd XOR rvg -> z12
    tnw OR pbm -> gnj
    "
        )),
        2024
    );
}

#[test]
fn b() {}
