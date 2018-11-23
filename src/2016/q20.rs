//-----------------------------------------------------
// Setup.

use aoc::Day;
use std::ops::Range;
use std::u32;

// static INPUT : &'static str = "5-8
// 0-2
// 4-7";
static INPUT: &'static str = include_str!("data/q20.data");

fn get_ranges() -> Vec<Range<u32>> {
    let mut ranges = Vec::new();
    for line in INPUT.lines() {
        let data: Vec<u32> = line.split('-').map(|i| i.parse::<u32>().unwrap()).collect();
        ranges.push(Range {
            start: data[0],
            end: data[1],
        });
    }
    ranges.sort_by_key(|k| k.start);
    ranges
}

fn get_first_allowed() -> u32 {
    let ranges = get_ranges();
    let mut rv = 0;
    for range in ranges.clone() {
        if rv < range.start {
            break;
        }
        if rv <= range.end {
            rv = range.end + 1;
        }
    }
    rv
}

fn get_num_allowed() -> u32 {
    let ranges = get_ranges();
    let mut curr = 0;
    let mut rv = 0;
    for range in ranges.clone() {
        if curr < range.start {
            rv += range.start - curr;
        }
        if range.end == u32::MAX {
            break;
        }
        if curr < range.end {
            curr = range.end + 1;
        }
    }
    rv
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
    fn number(&self) -> String {
        String::from("20")
    }

    fn a(&self) {
        print!("{}A: ", self.number());
        println!("Result = {}", get_first_allowed());
    }

    fn b(&self) {
        print!("{}B: ", self.number());
        println!("Result = {}", get_num_allowed());
    }
}
