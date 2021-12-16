//-----------------------------------------------------
// Setup.

use itertools::Itertools;

static INPUT: &str = include_str!("data/q16.data");

#[derive(Debug, Clone)]
struct Instruction {
    version: u64,
    type_id: u64,
    sub_instructions: Vec<Instruction>,
    value: u64,
}

impl Instruction {
    fn get_versions(&self) -> u64 {
        let mut rv = self.version;
        for instruction in &self.sub_instructions {
            rv += instruction.get_versions();
        }
        rv
    }

    fn evaluate(&self, prefix: usize) -> u64 {
        let mut rv = 0;
        match self.type_id {
            0 => {
                // sum
                // println!("{}Adding:", "  ".repeat(prefix));
                for instruction in &self.sub_instructions {
                    rv += instruction.evaluate(prefix + 1);
                }
                // println!("{}{}", "  ".repeat(prefix), rv);
            }
            1 => {
                // product
                // println!("{}Product:", "  ".repeat(prefix));
                rv = 1;
                for instruction in &self.sub_instructions {
                    rv *= instruction.evaluate(prefix + 1);
                }
                // println!("{}{}", "  ".repeat(prefix), rv);
            }
            2 => {
                // minimum
                // println!("{}Minimum:", "  ".repeat(prefix));
                let mut temp = vec![];
                for instruction in &self.sub_instructions {
                    temp.push(instruction.evaluate(prefix + 1));
                }
                rv = *temp.iter().min().unwrap();
                // println!("{}{}", "  ".repeat(prefix), rv);
            }
            3 => {
                // maximum
                // println!("{}Maximum:", "  ".repeat(prefix));
                let mut temp = vec![];
                for instruction in &self.sub_instructions {
                    temp.push(instruction.evaluate(prefix + 1));
                }
                rv = *temp.iter().max().unwrap();
                // println!("{}{}", "  ".repeat(prefix), rv);
            }
            4 => {
                // literal
                rv += self.value;
                // println!("{}Literal: {}", "  ".repeat(prefix), rv);
            }
            5 => {
                // greater than
                // println!("{}Greater Than:", "  ".repeat(prefix));
                if self.sub_instructions.len() != 2 {
                    println!(
                        "Error! Greater than has more than 2 sub-instructions! {:?}",
                        self
                    );
                }
                let a = self.sub_instructions[0].evaluate(prefix + 1);
                let b = self.sub_instructions[1].evaluate(prefix + 1);
                rv = if a > b { 1 } else { 0 };
                // println!("{}{}", "  ".repeat(prefix), rv);
            }
            6 => {
                // less than
                // println!("{}Less Than:", "  ".repeat(prefix));
                if self.sub_instructions.len() != 2 {
                    println!(
                        "Error! Greater than has more than 2 sub-instructions! {:?}",
                        self
                    );
                }
                let a = self.sub_instructions[0].evaluate(prefix + 1);
                let b = self.sub_instructions[1].evaluate(prefix + 1);
                rv = if a < b { 1 } else { 0 };
                // println!("{}{}", "  ".repeat(prefix), rv);
            }
            7 => {
                // equal to
                // println!("{}Equal To:", "  ".repeat(prefix));
                if self.sub_instructions.len() != 2 {
                    println!(
                        "Error! Greater than has more than 2 sub-instructions! {:?}",
                        self
                    );
                }
                let a = self.sub_instructions[0].evaluate(prefix + 1);
                let b = self.sub_instructions[1].evaluate(prefix + 1);
                rv = if a == b { 1 } else { 0 };
                // println!("{}{}", "  ".repeat(prefix), rv);
            }
            _ => {
                println!(
                    "\n\n===========\nUnknown Instruction! {:?}\n===========\n",
                    self
                );
            }
        }
        rv
    }
}

fn get_number(bits: &[u8], index: usize, size: usize) -> (u64, usize) {
    // println!("Parsing {}", &bits[index..index + size].iter().join(""));
    let value = u64::from_str_radix(&bits[index..index + size].iter().join(""), 2).unwrap();
    (value, index + size)
}

fn get_literal(bits: &[u8], start: usize) -> (u64, usize) {
    let mut index = start;
    let mut rv = 0;
    let mut done = false;
    while !done {
        let remaining = bits[index];
        index += 1;
        let (value, next) = get_number(bits, index, 4);
        index = next;
        rv <<= 4;
        rv += value;
        if index >= bits.len() || remaining == 0 {
            done = true;
        }
    }
    (rv, index)
}

fn get_operator(bits: &[u8], start: usize) -> (Vec<Instruction>, usize) {
    let mut rv = vec![];
    let mut index = start;
    let length = bits[index];
    index += 1;

    match length {
        0 => {
            // println!("    len {} of {}", index, bits.len());
            let (sub_length, next) = get_number(bits, index, 15);
            index = next;
            let end = index + sub_length as usize;
            while index < end {
                let (values, next) = parse_packet(bits, index);
                index = next;
                rv.extend(values.into_iter());
            }
        }
        1 => {
            // println!("    sub {} of {}", index, bits.len());
            let (sub_length, next) = get_number(bits, index, 11);
            index = next;
            for _ in 0..sub_length {
                let (values, next) = parse_packet(bits, index);
                index = next;
                rv.extend(values.into_iter());
            }
        }
        _ => {
            println!("Unknown length! {}", length);
            return (vec![], 0);
        }
    }
    (rv, index)
}

fn parse_packet(bits: &[u8], index: usize) -> (Vec<Instruction>, usize) {
    let mut rv = vec![];
    let (version, index) = get_number(bits, index, 3);
    let (type_id, index) = get_number(bits, index, 3);
    let mut index = index;
    // println!("{:?}, {:?}", version, type_id);
    match type_id {
        4 => {
            let (value, next) = get_literal(bits, index);
            index = next;
            // println!("  Literal {}", value);
            rv.push(Instruction {
                version,
                type_id,
                sub_instructions: vec![],
                value,
            });
        }
        _ => {
            let (sub_instructions, next) = get_operator(bits, index);
            index = next;
            // println!("  Operator {}", sub_instructions.len());
            rv.push(Instruction {
                version,
                type_id,
                sub_instructions,
                value: 0,
            });
        }
    }
    (rv, index)
}

fn process_data_a(data: &str) -> u64 {
    let mut bits: Vec<u8> = vec![];
    let data = data.trim();
    for value in data.chars() {
        // Do something
        let test = u8::from_str_radix(&value.to_string(), 16);
        if test.is_err() {
            println!("Error parsing {}: {:?}", value, test);
        }
        let value = test.unwrap();
        let add = format!("{:04b}", value);
        // println!("{:X} => {}", value, add);
        bits.extend(add.chars().map(|c| c.to_string().parse::<u8>().unwrap()));
    }
    // println!("{:?}", bits);
    let (instructions, _) = parse_packet(&bits, 0);
    // println!("{:?}", instructions);
    let mut rv = 0;
    for instruction in instructions {
        rv += instruction.get_versions();
    }
    rv
}

fn process_data_b(data: &str) -> u64 {
    let mut bits: Vec<u8> = vec![];
    let data = data.trim();
    for value in data.chars() {
        // Do something
        let test = u8::from_str_radix(&value.to_string(), 16);
        if test.is_err() {
            println!("Error parsing {}: {:?}", value, test);
        }
        let value = test.unwrap();
        let add = format!("{:04b}", value);
        // println!("{:X} => {}", value, add);
        bits.extend(add.chars().map(|c| c.to_string().parse::<u8>().unwrap()));
    }
    let (instructions, _) = parse_packet(&bits, 0);
    let mut rv = 0;
    for instruction in instructions {
        rv += instruction.evaluate(0);
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("16");

#[test]
fn a() {
    assert_eq!(process_data_a(indoc!("8A004A801A8002F478")), 16);
    assert_eq!(process_data_a(indoc!("620080001611562C8802118E34")), 12);
    assert_eq!(process_data_a(indoc!("C0015000016115A2E0802F182340")), 23);
    assert_eq!(process_data_a(indoc!("A0016C880162017C3686B18A3D4780")), 31);
}

#[test]
fn b() {
    assert_eq!(process_data_b(indoc!("C200B40A82")), 3);
    assert_eq!(process_data_b(indoc!("04005AC33890")), 54);
    assert_eq!(process_data_b(indoc!("880086C3E88112")), 7);
    assert_eq!(process_data_b(indoc!("CE00C43D881120")), 9);
    assert_eq!(process_data_b(indoc!("D8005AC2A8F0")), 1);
    assert_eq!(process_data_b(indoc!("F600BC2D8F")), 0);
    assert_eq!(process_data_b(indoc!("9C005AC2F8F0")), 0);
    assert_eq!(process_data_b(indoc!("9C0141080250320F1802104A08")), 1);
}
