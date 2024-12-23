//-----------------------------------------------------
// Setup.

use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline},
    multi::many1,
    sequence::tuple,
};

static INPUT: &str = include_str!("data/q24.data");

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Connection {
    And(String, String),
    Or(String, String),
    Xor(String, String),
}

fn gate(i: &str) -> IResult<&str, (String, bool)> {
    // x00: 1
    let (input, (gate, _, value, _)) =
        tuple((alphanumeric1, tag(": "), alt((tag("0"), tag("1"))), newline))(i)?;
    Ok((input, (gate.to_owned(), value == "1")))
}

fn gates(i: &str) -> IResult<&str, HashMap<String, bool>> {
    let (input, gates) = many1(gate)(i)?;
    Ok((input, gates.into_iter().collect()))
}

fn connection(i: &str) -> IResult<&str, (String, Connection)> {
    // x00 AND y00 -> z00

    let (input, (a, op, b, _, c, _)) = tuple((
        alphanumeric1,
        alt((tag(" AND "), tag(" OR "), tag(" XOR "))),
        alphanumeric1,
        tag(" -> "),
        alphanumeric1,
        newline,
    ))(i)?;
    let a = a.to_owned();
    let b = b.to_owned();
    let c = c.to_owned();
    let connection = match op {
        " AND " => Connection::And(a, b),
        " OR " => Connection::Or(a, b),
        " XOR " => Connection::Xor(a, b),
        _ => panic!("Unknown operator: {}", op.trim()),
    };
    Ok((input, (c, connection)))
}

fn connections(i: &str) -> IResult<&str, HashMap<String, Connection>> {
    let (input, connections) = many1(connection)(i)?;
    Ok((input, connections.into_iter().collect()))
}

fn parser(i: &str) -> IResult<&str, (HashMap<String, bool>, HashMap<String, Connection>)> {
    let (input, (initial, _, connections)) = tuple((gates, newline, connections))(i)?;
    Ok((input, (initial, connections)))
}

fn get_value(
    value: &str,
    initial: &HashMap<String, bool>,
    connections: &HashMap<String, Connection>,
) -> bool {
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

fn get_values<'a, T>(start: &str, connections: &'a HashMap<String, T>) -> Vec<&'a String> {
    connections
        .keys()
        .filter(|&k| k.starts_with(start))
        .sorted()
        .rev()
        .collect()
}

fn set(values: &[&String], arg: i64, next: &mut HashMap<String, bool>) {
    let mut arg = arg;
    for value in values {
        next.insert(value.to_owned().clone(), arg & 1 == 1);
        arg >>= 1;
    }
}

fn collect_gates(
    value: &str,
    initial: &HashMap<String, bool>,
    connections: &HashMap<String, Connection>,
) -> Vec<Connection> {
    let mut rv = vec![];
    if !initial.contains_key(value) {
        let connection = connections.get(value).unwrap();
        match connection {
            Connection::And(a, b) |
            Connection::Or(a, b) | Connection::Xor(a, b)
            => {
                rv.push(connection.clone());
                rv.extend(collect_gates(a, initial, connections));
                rv.extend(collect_gates(b, initial, connections));
            }
        }
    }
    rv
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

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    let (initial, connections) = parser(data).unwrap().1;
    // println!("initial: {:?}", initial);
    // println!("connections: {:?}", connections);
    let xs = get_values("x", &initial);
    let ys = get_values("y", &initial);
    let zs = get_values("z", &connections);
    println!("xs: {:?}", &xs);
    println!("ys: {:?}", &ys);
    println!("zs: {:?}", &zs);
    // Set x and y to various values, and see what z is.

    let mut next = initial.clone();
    set(&xs, (1 << xs.len()) - 1, &mut next);
    rv = 0;
    for &x in &xs {
        rv <<= 1;
        if get_value(x, &next, &connections) {
            rv += 1;
        }
    }
    println!("x: {:1$b}", rv, zs.len());
    set(&ys, (1 << ys.len()) - 1, &mut next);
    rv = 0;
    for &y in &ys {
        rv <<= 1;
        if get_value(y, &next, &connections) {
            rv += 1;
        }
    }
    println!("y: {:1$b}", rv, zs.len());
    rv = 0;
    for &z in &zs {
        let gates = collect_gates(z, &next, &connections);
        println!("{:?}: {:?}", z, gates);
        rv <<= 1;
        if get_value(z, &next, &connections) {
            rv += 1;
        }
    }
    println!("z: {:1$b}", rv, zs.len());
    rv
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
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "
        x00: 0
        x01: 1
        x02: 0
        x03: 1
        x04: 0
        x05: 1
        y00: 0
        y01: 0
        y02: 1
        y03: 1
        y04: 0
        y05: 1

        x00 AND y00 -> z05
        x01 AND y01 -> z02
        x02 AND y02 -> z01
        x03 AND y03 -> z03
        x04 AND y04 -> z04
        x05 AND y05 -> z00
"
        )),
        0
    );
}
