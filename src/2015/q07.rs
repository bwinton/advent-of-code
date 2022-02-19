//-----------------------------------------------------
// Setup.

use regex::Regex;
use std::{collections::HashMap, str::FromStr};

static INPUT: &str = include_str!("data/q07.data");

#[derive(Clone, Debug, Eq, PartialEq)]
struct Wire {
    name: String,
    number: Option<u16>,
}

impl Wire {
    pub fn new(name: &str, number: Option<u16>) -> Wire {
        Wire {
            name: name.to_string(),
            number,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Gate {
    ValueWire { a: String, out: String },
    ValueVal { a: u16, out: String },
    AndWire { a: String, b: String, out: String },
    AndVal { a: u16, b: String, out: String },
    Or { a: String, b: String, out: String },
    LShift { a: String, b: u16, out: String },
    RShift { a: String, b: u16, out: String },
    Not { a: String, out: String },
}

impl FromStr for Gate {
    type Err = ();

    fn from_str(s: &str) -> Result<Gate, ()> {
        let value_wire_re: &Regex = regex!("^([a-z]+) -> ([a-z]+)$");
        let value_val_re: &Regex = regex!("^([0-9]+) -> ([a-z]+)$");
        let and_wire_re: &Regex = regex!("^([a-z]+) AND ([a-z]+) -> ([a-z]+)$");
        let and_val_re: &Regex = regex!("^([0-9]+) AND ([a-z]+) -> ([a-z]+)$");
        let or_re: &Regex = regex!("^([a-z]+) OR ([a-z]+) -> ([a-z]+)$");
        let lshift_re: &Regex = regex!("^([a-z]+) LSHIFT ([0-9]+) -> ([a-z]+)$");
        let rshift_re: &Regex = regex!("^([a-z]+) RSHIFT ([0-9]+) -> ([a-z]+)$");
        let not_re: &Regex = regex!("^NOT ([a-z]+) -> ([a-z]+)$");

        if let Some(cap) = value_wire_re.captures(s) {
            return Ok(Gate::ValueWire {
                a: cap[1].parse().unwrap(),
                out: cap[2].parse().unwrap(),
            });
        }

        if let Some(cap) = value_val_re.captures(s) {
            return Ok(Gate::ValueVal {
                a: cap[1].parse().unwrap(),
                out: cap[2].parse().unwrap(),
            });
        }

        if let Some(cap) = and_wire_re.captures(s) {
            return Ok(Gate::AndWire {
                a: cap[1].parse().unwrap(),
                b: cap[2].parse().unwrap(),
                out: cap[3].parse().unwrap(),
            });
        }

        if let Some(cap) = and_val_re.captures(s) {
            return Ok(Gate::AndVal {
                a: cap[1].parse().unwrap(),
                b: cap[2].parse().unwrap(),
                out: cap[3].parse().unwrap(),
            });
        }

        if let Some(cap) = or_re.captures(s) {
            return Ok(Gate::Or {
                a: cap[1].parse().unwrap(),
                b: cap[2].parse().unwrap(),
                out: cap[3].parse().unwrap(),
            });
        }

        if let Some(cap) = lshift_re.captures(s) {
            return Ok(Gate::LShift {
                a: cap[1].parse().unwrap(),
                b: cap[2].parse().unwrap(),
                out: cap[3].parse().unwrap(),
            });
        }

        if let Some(cap) = rshift_re.captures(s) {
            return Ok(Gate::RShift {
                a: cap[1].parse().unwrap(),
                b: cap[2].parse().unwrap(),
                out: cap[3].parse().unwrap(),
            });
        }

        if let Some(cap) = not_re.captures(s) {
            return Ok(Gate::Not {
                a: cap[1].parse().unwrap(),
                out: cap[2].parse().unwrap(),
            });
        }

        println!("Unknown line: {}", s);
        Err(())
    }
}

impl Gate {
    fn execute(&self, wires: &mut HashMap<String, Wire>) -> bool {
        match *self {
            Gate::ValueWire { ref a, ref out } => {
                ensure_wire(wires, out);
                ensure_wire(wires, a);
                match wires[a].number {
                    None => false,
                    Some(x) => {
                        wires.get_mut(out).unwrap().number = Some(x);
                        true
                    }
                }
            }

            Gate::ValueVal { ref a, ref out } => {
                ensure_wire(wires, out);
                wires.get_mut(out).unwrap().number = Some(*a);
                true
            }

            Gate::AndWire {
                ref a,
                ref b,
                ref out,
            } => {
                ensure_wire(wires, out);
                ensure_wire(wires, a);
                ensure_wire(wires, b);
                match wires[a].number {
                    None => false,
                    Some(x) => match wires[b].number {
                        None => false,
                        Some(y) => {
                            wires.get_mut(out).unwrap().number = Some(x & y);
                            true
                        }
                    },
                }
            }

            Gate::AndVal {
                ref a,
                ref b,
                ref out,
            } => {
                ensure_wire(wires, out);
                ensure_wire(wires, b);
                match wires[b].number {
                    None => false,
                    Some(y) => {
                        wires.get_mut(out).unwrap().number = Some(a & y);
                        true
                    }
                }
            }

            Gate::Or {
                ref a,
                ref b,
                ref out,
            } => {
                ensure_wire(wires, out);
                ensure_wire(wires, a);
                ensure_wire(wires, b);
                match wires[a].number {
                    None => false,
                    Some(x) => match wires[b].number {
                        None => false,
                        Some(y) => {
                            wires.get_mut(out).unwrap().number = Some(x | y);
                            true
                        }
                    },
                }
            }

            Gate::LShift {
                ref a,
                ref b,
                ref out,
            } => {
                ensure_wire(wires, out);
                ensure_wire(wires, a);
                match wires[a].number {
                    None => false,
                    Some(x) => {
                        wires.get_mut(out).unwrap().number = Some(x << b);
                        true
                    }
                }
            }

            Gate::RShift {
                ref a,
                ref b,
                ref out,
            } => {
                ensure_wire(wires, out);
                ensure_wire(wires, a);
                match wires[a].number {
                    None => false,
                    Some(x) => {
                        wires.get_mut(out).unwrap().number = Some(x >> b);
                        true
                    }
                }
            }

            Gate::Not { ref a, ref out } => {
                ensure_wire(wires, out);
                ensure_wire(wires, a);
                match wires[a].number {
                    None => false,
                    Some(x) => {
                        wires.get_mut(out).unwrap().number = Some(!x);
                        true
                    }
                }
            }
        }
    }
}

fn ensure_wire(wires: &mut HashMap<String, Wire>, name: &str) {
    if !wires.contains_key(name) {
        wires.insert(name.to_string(), Wire::new(name, None));
    }
}

fn run_pending(pending: &mut Vec<Gate>, wires: &mut HashMap<String, Wire>) {
    loop {
        let len = pending.len();
        pending.retain(|gate| !gate.execute(wires));
        if pending.len() == len {
            break;
        }
    }
}

fn process_data(data: &str) -> HashMap<String, Wire> {
    let mut wires: HashMap<String, Wire> = hashmap![];
    let mut pending: Vec<Gate> = Vec::new();

    for line in data.lines() {
        let gate: Gate = line.parse().unwrap();
        pending.insert(0, gate);
        // println!("  {:?}", pending);
        run_pending(&mut pending, &mut wires);
        // println!("  {:?}\n  {:?}\n", pending, wires);
    }
    wires
}

fn process_data_a(data: &str) -> u16 {
    let input = "44430 -> b\n".to_owned() + data;
    let result = process_data(&input);
    result["a"].number.unwrap()
}

fn process_data_b(data: &str) -> u16 {
    let input = "3176 -> b\n".to_owned() + data;
    let result = process_data(&input);
    result["a"].number.unwrap()
}

//-----------------------------------------------------
// Questions.

q_impl!("7");

#[test]
fn a() {
    assert_eq!(
        process_data(
            "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i",
        ),
        hashmap![
          "d".to_string() => Wire::new("d", Some(72)),
          "e".to_string() => Wire::new("e", Some(507)),
          "f".to_string() => Wire::new("f", Some(492)),
          "g".to_string() => Wire::new("g", Some(114)),
          "h".to_string() => Wire::new("h", Some(65412)),
          "i".to_string() => Wire::new("i", Some(65079)),
          "x".to_string() => Wire::new("x", Some(123)),
          "y".to_string() => Wire::new("y", Some(456)),
        ]
    );
}

#[test]
fn b() {}
