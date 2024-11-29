//-----------------------------------------------------
// Setup.

use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;

static INPUT: &str = include_str!("data/q14.data");

static INSTRUCTION_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^mem\[([0-9]+)\] = ([0-9]+)$").unwrap());

#[derive(Debug)]
struct Instruction {
    addresses: Vec<usize>,
    value: u128,
}
impl Instruction {
    fn parse(line: &str, mask: &str) -> Self {
        match INSTRUCTION_RE.captures(line) {
            Some(captures) => {
                let addresses = vec![captures[1].parse().unwrap()];
                let mut value = captures[2].parse().unwrap();
                for (i, character) in mask.chars().rev().enumerate() {
                    match character {
                        '0' => value &= !(1 << i),
                        '1' => value |= 1 << i,
                        _ => {}
                    }
                }
                Instruction { addresses, value }
            }
            _ => {
                panic!("Invalid line: {}", line);
            }
        }
    }

    fn get_mask(variants: &[usize], i: usize) -> usize {
        let mut rv = 0;
        for (index, value) in variants.iter().enumerate() {
            let test = 1 << index;
            if (i & test) != 0 {
                rv |= 1 << value;
            }
        }
        rv
    }

    fn parse_b(line: &str, mask: &str) -> Self {
        match INSTRUCTION_RE.captures(line) {
            Some(captures) => {
                let mut base_address: usize = captures[1].parse().unwrap();
                let mut variants = vec![];
                let value = captures[2].parse().unwrap();
                for (i, character) in mask.chars().rev().enumerate() {
                    match character {
                        '1' => base_address |= 1 << i,
                        'X' => {
                            base_address &= !(1 << i);
                            variants.push(i);
                        }
                        _ => {}
                    }
                }
                let mut addresses = vec![];
                let len: usize = 1 << variants.len();
                for i in 0..len {
                    let mask = Instruction::get_mask(&variants, i);
                    let address = base_address | mask;
                    addresses.push(address)
                }
                Instruction { addresses, value }
            }
            _ => {
                panic!("Invalid line: {}", line);
            }
        }
    }
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    fn new() -> Self {
        Program {
            instructions: vec![],
        }
    }
}

fn process_data_a(data: &str) -> u128 {
    let mut programs: Vec<Program> = vec![];
    let mut curr = Program::new();
    let mut mask = "".to_string();
    for line in data.lines() {
        if line.starts_with("mask = ") {
            if !&mask.is_empty() {
                programs.push(curr);
                curr = Program::new();
            }
            mask = line.strip_prefix("mask = ").unwrap().to_string();
            continue;
        }
        // Line is mem.
        curr.instructions.push(Instruction::parse(line, &mask));
    }
    programs.push(curr);

    let mut rv = 0;
    let mut memory: HashMap<usize, u128> = HashMap::new();
    for program in programs {
        for instruction in program.instructions {
            for address in instruction.addresses {
                memory.insert(address, instruction.value);
            }
        }
    }
    for value in memory {
        rv += value.1;
    }
    rv
}

fn process_data_b(data: &str) -> u128 {
    let mut programs: Vec<Program> = vec![];
    let mut curr = Program::new();
    let mut mask = "".to_string();
    for line in data.lines() {
        if line.starts_with("mask = ") {
            if !&mask.is_empty() {
                programs.push(curr);
                curr = Program::new();
            }
            mask = line.strip_prefix("mask = ").unwrap().to_string();
            continue;
        }
        // Line is mem.
        curr.instructions.push(Instruction::parse_b(line, &mask));
    }
    programs.push(curr);

    let mut rv = 0;
    let mut memory: HashMap<usize, u128> = HashMap::new();
    for program in programs {
        for instruction in program.instructions {
            for address in instruction.addresses {
                memory.insert(address, instruction.value);
            }
        }
    }
    for value in memory {
        rv += value.1;
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("14");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
"
        ),
        165
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(
            "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
"
        ),
        208
    );
}
