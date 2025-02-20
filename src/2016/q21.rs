//-----------------------------------------------------
// Setup.

use aoc::Day;
use regex::Regex;
use std::str::FromStr;

// static PASSCODE : &'static str = "abcde";
// static INPUT : &'static str = "swap position 4 with position 0
// swap letter d with letter b
// reverse positions 0 through 4
// rotate left 1 step
// move position 1 to position 4
// move position 3 to position 0
// rotate based on position of letter b
// rotate based on position of letter d";
static INPUT: &str = include_str!("data/q21.data");
static PASSCODE: &str = "abcdefgh";
static SCRAMBLED: &str = "fbgdceah";

#[derive(Clone, Debug)]
enum Instruction {
    SwapPosition(usize, usize),
    SwapLetter(String, String),
    RotateLeft(usize),
    RotateRight(usize),
    RotateLetter(String),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl Instruction {
    fn execute(&self, state: &str) -> String {
        let mut rv = state.to_string();
        match (*self).clone() {
            Instruction::SwapPosition(x, y) => {
                let mut temp: Vec<char> = rv.chars().collect();
                temp.swap(x, y);
                rv = temp.into_iter().collect();
            }
            Instruction::SwapLetter(a, b) => {
                rv = rv.replace(&a, "?");
                rv = rv.replace(&b, &a);
                rv = rv.replace('?', &b);
            }
            Instruction::RotateLeft(n) => {
                let data: Vec<char> = rv.chars().collect();
                let mut temp = Vec::new();
                temp.extend_from_slice(&data[n..]);
                temp.extend_from_slice(&data[..n]);
                rv = temp.into_iter().collect();
            }
            Instruction::RotateRight(n) => {
                let data: Vec<char> = rv.chars().collect();
                let mut temp = Vec::new();
                let index = data.len() - n;
                temp.extend_from_slice(&data[index..]);
                temp.extend_from_slice(&data[..index]);
                rv = temp.into_iter().collect();
            }
            Instruction::RotateLetter(a) => {
                let data: Vec<char> = rv.chars().collect();
                let mut temp = Vec::new();
                let mut index = data.iter().position(|e| e.to_string() == a).unwrap();
                if index >= 4 {
                    index += 1;
                }
                index += 1;
                index %= data.len();
                index = data.len() - index;
                temp.extend_from_slice(&data[index..]);
                temp.extend_from_slice(&data[..index]);
                rv = temp.into_iter().collect();
            }
            Instruction::Reverse(x, y) => {
                let mut temp: Vec<char> = rv.chars().collect();
                temp[x..=y].reverse();
                rv = temp.into_iter().collect();
            }
            Instruction::Move(x, y) => {
                let mut temp: Vec<char> = rv.chars().collect();
                let element = temp.remove(x);
                temp.insert(y, element);
                rv = temp.into_iter().collect();
            }
        }
        rv
    }

    fn unexecute(&self, state: &str) -> String {
        let mut rv = state.to_string();
        match (*self).clone() {
            Instruction::SwapPosition(x, y) => {
                let mut temp: Vec<char> = rv.chars().collect();
                temp.swap(x, y);
                rv = temp.into_iter().collect();
            }
            Instruction::SwapLetter(a, b) => {
                rv = rv.replace(&a, "?");
                rv = rv.replace(&b, &a);
                rv = rv.replace('?', &b);
            }
            Instruction::RotateLeft(n) => {
                let data: Vec<char> = rv.chars().collect();
                let mut temp = Vec::new();
                let index = data.len() - n;
                temp.extend_from_slice(&data[index..]);
                temp.extend_from_slice(&data[..index]);
                rv = temp.into_iter().collect();
            }
            Instruction::RotateRight(n) => {
                let data: Vec<char> = rv.chars().collect();
                let mut temp = Vec::new();
                temp.extend_from_slice(&data[n..]);
                temp.extend_from_slice(&data[..n]);
                rv = temp.into_iter().collect();
            }
            Instruction::RotateLetter(a) => {
                let data: Vec<char> = rv.chars().collect();
                let mut temp = Vec::new();
                let mut new_index = data.iter().position(|e| e.to_string() == a).unwrap();
                if new_index == 0 {
                    new_index = 8;
                }
                let mut index = if new_index % 2 == 1 {
                    new_index.div_ceil(2)
                } else {
                    5 + new_index / 2
                };
                index %= data.len();
                temp.extend_from_slice(&data[index..]);
                temp.extend_from_slice(&data[..index]);
                rv = temp.into_iter().collect();
            }
            Instruction::Reverse(x, y) => {
                let mut temp: Vec<char> = rv.chars().collect();
                temp[x..=y].reverse();
                rv = temp.into_iter().collect();
            }
            Instruction::Move(x, y) => {
                let mut temp: Vec<char> = rv.chars().collect();
                let element = temp.remove(y);
                temp.insert(x, element);
                rv = temp.into_iter().collect();
            }
        }
        rv
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Instruction, ()> {
        let swap_position_re: &Regex = regex!("swap position ([0-9]+) with position ([0-9]+)");
        let swap_letter_re: &Regex = regex!("swap letter ([a-z]) with letter ([a-z])");
        let rotate_left_re: &Regex = regex!("rotate left ([0-9]+) steps?");
        let rotate_right_re: &Regex = regex!("rotate right ([0-9]+) steps?");
        let rotate_letter_re: &Regex = regex!("rotate based on position of letter ([a-z])");
        let reverse_re: &Regex = regex!("reverse positions ([0-9]+) through ([0-9]+)");
        let move_re: &Regex = regex!("move position ([0-9]+) to position ([0-9]+)");

        if let Some(cap) = swap_position_re.captures(s) {
            return Ok(Instruction::SwapPosition(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        if let Some(cap) = swap_letter_re.captures(s) {
            return Ok(Instruction::SwapLetter(
                cap[1].to_string(),
                cap[2].to_string(),
            ));
        }

        if let Some(cap) = rotate_left_re.captures(s) {
            return Ok(Instruction::RotateLeft(cap[1].parse().unwrap()));
        }

        if let Some(cap) = rotate_right_re.captures(s) {
            return Ok(Instruction::RotateRight(cap[1].parse().unwrap()));
        }

        if let Some(cap) = rotate_letter_re.captures(s) {
            return Ok(Instruction::RotateLetter(cap[1].to_string()));
        }

        if let Some(cap) = reverse_re.captures(s) {
            return Ok(Instruction::Reverse(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        if let Some(cap) = move_re.captures(s) {
            return Ok(Instruction::Move(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        Err(())
    }
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
    fn number(&self) -> String {
        String::from("21")
    }

    fn a(&self) {
        print!("{}A: ", self.number());
        let mut instructions: Vec<Instruction> = Vec::new();
        for line in INPUT.lines() {
            let instruction = line.parse().unwrap();
            instructions.push(instruction);
        }

        let mut rv = PASSCODE.to_string();
        for instruction in instructions {
            rv = instruction.execute(&rv);
        }
        println!("Result = {}", rv);
    }

    fn b(&self) {
        print!("{}B: ", self.number());
        let mut instructions: Vec<Instruction> = Vec::new();
        for line in INPUT.lines() {
            let instruction = line.parse().unwrap();
            instructions.push(instruction);
        }

        instructions.reverse();
        let mut rv = SCRAMBLED.to_string();
        for instruction in &instructions {
            rv = instruction.unexecute(&rv);
        }
        println!("Result = {}", rv);
    }
}
