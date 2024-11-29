//-----------------------------------------------------
// Setup.

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, one_of},
    combinator::opt,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use num_integer::lcm;

static INPUT: &str = include_str!("data/q20.data");

#[derive(Clone, Debug, Eq, PartialEq)]
enum GateType<'a> {
    Normal,
    // false = off, true = on!
    FlipFlop(bool),
    // false = low, true = high!
    Conjunction(HashMap<&'a str, bool>),
}
impl Hash for GateType<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            GateType::Normal => {}
            GateType::FlipFlop(x) => x.hash(state),
            GateType::Conjunction(map) => {
                for entry in map.iter() {
                    entry.hash(state);
                }
            }
        }
        let _ = state.finish();
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Gate<'a> {
    name: &'a str,
    gate_type: GateType<'a>,
    inputs: Vec<&'a str>,
    outputs: Vec<&'a str>,
}
impl<'a> Gate<'a> {
    fn process(&mut self, source: &str, pulse: bool) -> Vec<(&'a str, bool, &'a str)> {
        let mut rv = vec![];
        let pulse = match &mut self.gate_type {
            GateType::Normal => Some(pulse),
            GateType::FlipFlop(x) => {
                // If it's a low pulse, flip and send the new value.
                if !pulse {
                    *x = !*x;
                    Some(*x)
                } else {
                    None
                }
            }
            GateType::Conjunction(inputs) => {
                *inputs.get_mut(source).unwrap() = pulse;
                Some(!inputs.values().all(|x| *x))
            }
        };
        if let Some(pulse) = pulse {
            for &output in &self.outputs {
                // if output == "rx" { println!("Found one!!! {}", pulse)}
                rv.push((self.name, pulse, output));
            }
        }
        rv
    }
}

fn gate(i: &str) -> IResult<&str, Gate> {
    // broadcaster -> a, b, c
    let (input, (gate_type, name, _, outputs)) = tuple((
        opt(one_of("&%")),
        alpha1,
        tag(" -> "),
        separated_list1(tag(", "), alpha1),
    ))(i)?;
    let gate_type = match gate_type.unwrap_or(' ') {
        ' ' => GateType::Normal,
        '%' => GateType::FlipFlop(false),
        '&' => GateType::Conjunction(HashMap::new()),
        _ => {
            panic!("Unknown gate type! {:?}", gate_type);
        }
    };
    let inputs = vec![];
    Ok((
        input,
        Gate {
            name,
            gate_type,
            inputs,
            outputs,
        },
    ))
}

fn parser(i: &str) -> IResult<&str, HashMap<&str, Gate>> {
    let (input, gates) = separated_list1(newline, gate)(i)?;
    let mut gates = HashMap::from_iter(gates.into_iter().map(|gate| (gate.name, gate)));
    for (key, gate) in gates.clone() {
        for output in gate.outputs {
            if let Some(output) = gates.get_mut(output) {
                output.inputs.push(key);
                if let GateType::Conjunction(x) = &mut output.gate_type {
                    x.insert(key, false);
                }
            }
        }
    }
    Ok((input, gates))
}

fn process_data_a(data: &str) -> usize {
    let mut gates = parser(data).unwrap().1;
    let mut pulses = VecDeque::new();
    let mut lows = 0usize;
    let mut highs = 0usize;
    for _ in 0usize..1000 {
        pulses.push_back(("button", false, "broadcaster"));
        let _prev = (lows, highs);
        while let Some((source, pulse, dest)) = pulses.pop_front() {
            if pulse {
                highs += 1;
            } else {
                lows += 1;
            }
            if let Some(gate) = gates.get_mut(dest) {
                for next in gate.process(source, pulse) {
                    pulses.push_back(next);
                }
            }
        }
    }
    lows * highs
}

fn process_data_b(data: &str) -> usize {
    let mut gates = parser(data).unwrap().1;
    let mut pulses = VecDeque::new();
    let mut presses: usize = 0;
    let mut values = HashMap::new();
    // We need to find rx.
    let mut inputs: Vec<&str> = gates
        .values()
        .filter(|&gate| gate.outputs.contains(&"rx"))
        .map(|gate| gate.name)
        .collect();
    while inputs.len() == 1 {
        inputs = gates.get(inputs[0]).unwrap().inputs.clone();
    }
    'outer: loop {
        pulses.push_back(("button", false, "broadcaster"));
        presses += 1;
        while let Some((source, pulse, dest)) = pulses.pop_front() {
            if pulse && inputs.contains(&source) {
                values.entry(source).or_insert(presses);
                if values.len() == 4 {
                    break 'outer;
                }
            }

            // println!("Processing {:?}", (source, pulse, dest));
            if let Some(gate) = gates.get_mut(dest) {
                for next in gate.process(source, pulse) {
                    // println!("  adding {:?}", next);
                    if !next.1 && next.2 == "rx" {
                        break 'outer;
                    }
                    pulses.push_back(next);
                }
            }
        }
    }
    let temp = values.into_values().reduce(lcm);
    temp.unwrap()
}

//-----------------------------------------------------
// Questions.

q_impl!("20");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "
    broadcaster -> a, b, c
    %a -> b
    %b -> c
    %c -> inv
    &inv -> a
    "
        )),
        32000000
    );

    assert_eq!(
        process_data_a(indoc!(
            "broadcaster -> a
    %a -> inv, con
    &inv -> b
    %b -> con
    &con -> output
    "
        )),
        11687500
    );
}

#[test]
fn b() {
    // use pretty_assertions::assert_eq;

    // assert_eq!(process_data_b(indoc!("")), 0);
}
