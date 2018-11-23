//-----------------------------------------------------
// Setup.

use aoc::Day;
use regex::Regex;

static INPUT: &'static str = include_str!("data/q09.data");
// static INPUT : &'static str = "ADVENT
// A(1x5)BC
// (3x3)XYZ
// A(2x2)BCD(2x2)EFG
// (6x1)(1x3)A
// X(8x2)(3x3)ABCY
// (27x12)(20x12)(13x14)(7x10)(1x12)A
// (25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN";

fn decompress(line: &str) -> String {
    // print!("{}", line);
    let marker_re = Regex::new(r"\((\d+)x(\d+)\)").unwrap();
    let mut rv = String::new();
    let mut rest = String::from(line);
    loop {
        match marker_re.captures(&rest.clone()) {
            None => {
                rv.push_str(&rest[..]);
                break;
            }
            Some(marker) => {
                let first = marker.get(0).unwrap();
                rv.push_str(&rest[..first.start()]);
                rest = String::from(&rest[first.end()..]);
                let repeat: usize = marker[2].parse().unwrap();
                let count: usize = marker[1].parse().unwrap();
                let rep_str = String::from(&rest[..count]);
                for _ in 0..repeat {
                    rv.push_str(&rep_str);
                }
                rest = String::from(&rest[count..]);
            }
        }
    }
    // println!(" => {}", rv);
    rv
}

fn double_decompress(line: &str) -> String {
    // print!("{}", line);
    let marker_re = Regex::new(r"\((\d+)x(\d+)\)").unwrap();
    let mut rv = String::new();
    let mut rest = String::from(line);
    loop {
        match marker_re.captures(&rest.clone()) {
            None => {
                rv.push_str(&rest[..]);
                break;
            }
            Some(marker) => {
                let first = marker.get(0).unwrap();
                rv.push_str(&rest[..first.start()]);
                rest = String::from(&rest[first.end()..]);
                let count: usize = marker[1].parse().unwrap();
                let repeat: usize = marker[2].parse().unwrap();
                let mut rep_str = String::from(&rest[..count]);
                rep_str = decompress(&rep_str);
                for _ in 0..repeat {
                    rv.push_str(&rep_str);
                }
                rest = String::from(&rest[count..]);
            }
        }
    }
    // println!(" => {}", rv);
    rv
}
//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
    fn number(&self) -> String {
        String::from("9")
    }

    fn a(&self) {
        print!("{}A: ", self.number());
        let mut result = 0;
        for line in INPUT.lines() {
            let output = decompress(line);
            result += output.len();
        }
        println!("Result = {}", result);
    }

    fn b(&self) {
        print!("{}B: ", self.number());
        let mut result = 0;
        for line in INPUT.lines() {
            let output = double_decompress(line);
            result += output.len();
        }
        println!("Result = {}", result);
    }
}
