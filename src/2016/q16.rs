//-----------------------------------------------------
// Setup.

use aoc::Day;
use std::iter::FromIterator;

// static INPUT : &'static str = "10000";
// static LENGTH_A : usize = 20;
static INPUT: &str = "10111100110001111";
static LENGTH_A: usize = 272;
static LENGTH_B: usize = 35_651_584;

fn get_checksum(input: &str) -> String {
    let mut rv = Vec::from_iter(input.chars());
    while rv.len() % 2 == 0 {
        let mut new = Vec::new();
        for pair in rv.chunks(2) {
            if pair[0] == pair[1] {
                new.push('1');
            } else {
                new.push('0');
            }
        }
        rv = new;
    }
    rv.into_iter().collect()
}

fn dragon(input: &str) -> String {
    let mut rv = String::from("0");
    let temp = input
        .chars()
        .rev()
        .map(|c| match c {
            '0' => '1',
            _ => '0',
        })
        .collect::<String>();
    rv.push_str(&temp);
    rv
}

fn get_result(input: &str, length: usize) -> String {
    let mut data = String::from(input);
    while data.len() < length {
        let extra = dragon(&data);
        data.push_str(&extra);
    }
    data = data[0..length].to_string();
    get_checksum(&data)
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
        println!("Result = {}", get_result(INPUT, LENGTH_A));
    }

    fn b(&self) {
        print!("{}B: ", self.number());
        println!("Result = {}", get_result(INPUT, LENGTH_B));
    }
}
