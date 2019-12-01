//-----------------------------------------------------
// Setup.

use aoc::Day;
use std::collections::HashMap;

static INPUT: &str = include_str!("data/q06.data");
// static INPUT : &'static str = "eedadn
// drvtee
// eandsr
// raavrd
// atevrs
// tsrnev
// sdttsa
// rasrtv
// nssdts
// ntnada
// svetve
// tesnvt
// vntsnd
// vrdear
// dvrsen
// enarar";

/*
make a password struct.
contains n items, one per letter.
add an update method that adds new letters to each item.

    let mut chars = Vec::<(i32, char)>::new();
    for char in self.name.chars() {
      if char != '-' {
        let pos = chars.iter().position(|&r| r.1 == char);
        match pos {
          None => chars.push((-1, char)),
          Some(i) => chars.get_mut(i).unwrap().0 -= 1
        }
      }
    }

at the end
    chars.sort();
    chars.truncate(1);
    let data = String::from_iter(chars.iter().map(|x| x.1));
(Actually get the first element from each item's chars.)

*/

fn get_most_common(frequencies: Vec<HashMap<char, i32>>) -> String {
    let mut result: Vec<char> = Vec::new();
    for frequency in frequencies {
        let mut count: Vec<_> = frequency.iter().collect();
        count.sort_by(|a, b| b.1.cmp(a.1));
        result.push(*count[0].0);
    }
    result.into_iter().collect()
}

fn get_least_common(frequencies: Vec<HashMap<char, i32>>) -> String {
    let mut result: Vec<char> = Vec::new();
    for frequency in frequencies {
        let mut count: Vec<_> = frequency.iter().collect();
        count.sort_by(|a, b| a.1.cmp(b.1));
        result.push(*count[0].0);
    }
    result.into_iter().collect()
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
    fn number(&self) -> String {
        String::from("6")
    }

    fn a(&self) {
        print!("{}A: ", self.number());
        let mut frequencies: Vec<HashMap<char, i32>> = Vec::new();
        for line in INPUT.lines() {
            if frequencies.is_empty() {
                for _ in 0..line.len() {
                    frequencies.push(HashMap::<char, i32>::new().clone())
                }
            }
            for (i, freq) in frequencies.iter_mut().enumerate().take(line.len()) {
                let curr = line.chars().nth(i).unwrap();
                let value = freq.entry(curr).or_insert(0);
                *value += 1;
            }
        }
        let result = get_most_common(frequencies);
        println!("Result = {:?}", result);
    }

    fn b(&self) {
        print!("{}B: ", self.number());
        let mut frequencies: Vec<HashMap<char, i32>> = Vec::new();
        for line in INPUT.lines() {
            if frequencies.is_empty() {
                for _ in 0..line.len() {
                    frequencies.push(HashMap::<char, i32>::new().clone())
                }
            }
            for (i, freq) in frequencies.iter_mut().enumerate().take(line.len()) {
                let curr = line.chars().nth(i).unwrap();
                let value = freq.entry(curr).or_insert(0);
                *value += 1;
            }
        }
        let result = get_least_common(frequencies);
        println!("Result = {:?}", result);
    }
}
