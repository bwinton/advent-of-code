//-----------------------------------------------------
// Setup.

use aoc::Day;

use regex::Regex;
use std::iter::FromIterator;
use std::str::FromStr;

static INPUT: &str = include_str!("data/q16.data");

#[derive(Debug)]
enum Instruction {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Instruction, ()> {
        lazy_static! {
            static ref SPIN_RE: Regex = Regex::new(r"^s(-?\d+)$").unwrap();
            static ref EXCHANGE_RE: Regex = Regex::new(r"^x(-?[0-9]+)/(-?[0-9]+)$").unwrap();
            static ref PARTNER_RE: Regex = Regex::new(r"^p([a-z])/([a-z])$").unwrap();
        }

        if let Some(cap) = SPIN_RE.captures(s) {
            return Ok(Instruction::Spin(cap[1].parse().unwrap()));
        }

        if let Some(cap) = EXCHANGE_RE.captures(s) {
            return Ok(Instruction::Exchange(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        if let Some(cap) = PARTNER_RE.captures(s) {
            return Ok(Instruction::Partner(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        println!("Unknown instruction! '{}'", s);
        Err(())
    }
}

impl Instruction {
    fn execute(&self, programs: &[char]) -> Vec<char> {
        let mut rv = Vec::new();
        match *self {
            Instruction::Spin(x) => {
                let n = programs.len() - x;
                rv.extend_from_slice(&programs[n..]);
                rv.extend_from_slice(&programs[..n]);
            }
            Instruction::Exchange(a, b) => {
                rv = programs.to_owned();
                rv.swap(a, b);
            }
            Instruction::Partner(a, b) => {
                rv = programs
                    .iter()
                    .map(|x| match *x {
                        x if x == a => b,
                        x if x == b => a,
                        _ => *x,
                    })
                    .collect();
            }
        }
        // println!("{:?} => {:?} => {:?}", programs, self, rv);
        rv
    }
}

fn process_data_a(max: u8, data: &str) -> String {
    let mut rv: Vec<char> = Vec::new();
    for i in 0..max {
        rv.push((b'a' + i) as char);
    }
    for line in data.split(',') {
        let curr: Instruction = line.parse().unwrap();
        rv = curr.execute(&rv);
    }
    String::from_iter(&rv)
}

fn process_data_b(max: u8, data: &str, iterations: usize) -> String {
    let mut rv: Vec<char> = Vec::new();
    for i in 0..max {
        rv.push((b'a' + i) as char);
    }
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in data.split(',') {
        instructions.push(line.parse().unwrap());
    }
    let mut seen = Vec::new();
    for _ in 0..iterations {
        seen.push(rv.clone());
        for curr in &instructions {
            rv = curr.execute(&rv);
        }
        if seen.contains(&rv) {
            rv = seen[iterations % seen.len()].clone();
            break;
        }
    }
    String::from_iter(&rv)
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
    fn number(&self) -> String {
        String::from("16")
    }

    fn a(&self) {
        print!("{}A: ", self.number());
        let result = process_data_a(16, INPUT);
        println!("Result = {}", result);
    }

    fn b(&self) {
        print!("{}B: ", self.number());
        let result = process_data_b(16, INPUT, 1_000_000_000);
        println!("Result = {}", result);
    }
}

#[test]
fn a() {
    assert_eq!(process_data_a(5, "s1,x3/4,pe/b"), "baedc");
}

#[test]
fn b() {
    assert_eq!(process_data_b(5, "s1,x3/4,pe/b", 2), "ceadb");
}
