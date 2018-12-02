//-----------------------------------------------------
// Setup.

use aoc::Day;

static INPUT: &'static str = include_str!("data/q01.data");

fn process_data(line: &str, offset: usize) -> u32 {
    let mut rv = 0;
    let chars = line.chars();
    let nexts = line.chars().cycle().skip(offset);
    for (curr, next) in chars.zip(nexts) {
        if curr == next {
            rv += curr.to_digit(10).unwrap();
        }
    }
    rv
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
    fn number(&self) -> String {
        String::from("1")
    }

    fn a(&self) {
        print!("{}A: ", self.number());
        let result = process_data(INPUT, 1);
        println!("Result = {}", result);
    }

    fn b(&self) {
        print!("{}B: ", self.number());
        let result = process_data(INPUT, INPUT.len() / 2);
        println!("Result = {}", result);
    }
}

#[test]
fn test_a() {
    assert_eq!(process_data("1122", 1), 3);
    assert_eq!(process_data("1111", 1), 4);
    assert_eq!(process_data("1234", 1), 0);
    assert_eq!(process_data("91212129", 1), 9);
}

#[test]
fn test_b() {
    assert_eq!(process_data("1212", 2), 6);
    assert_eq!(process_data("1221", 2), 0);
    assert_eq!(process_data("123425", 3), 4);
    assert_eq!(process_data("123123", 3), 12);
    assert_eq!(process_data("12131415", 4), 4);
}
