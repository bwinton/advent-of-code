//-----------------------------------------------------
// Setup.

use aoc::Day;

use std::collections::HashMap;

static INPUT: &'static str = include_str!("data/q02.data");

fn get_counts(line: &str) -> HashMap<char, u32> {
    let mut seen = HashMap::new();
    for char in line.chars() {
        let entry = seen.entry(char).or_insert(0);
        *entry += 1;
    }
    seen
}

fn process_data_a(data: &str) -> i32 {
    let mut two_count = 0;
    let mut three_count = 0;
    for line in data.lines() {
        let counts = get_counts(line);
        if counts.values().any(|x| x == &2) {
            two_count += 1;
        }
        if counts.values().any(|x| x == &3) {
            three_count += 1;
        }
    }
    two_count * three_count
}

fn process_data_b(data: &str) -> String {
    for (skip, line) in data.lines().enumerate() {
        let target = line.len() - 1;
        for test in data.lines().skip(skip + 1) {
            let answer: String = line
                .chars()
                .zip(test.chars())
                .filter_map(|x| if x.0 == x.1 { Some(x.0) } else { None })
                .collect();
            if answer.len() == target {
                return answer;
            }
        }
    }
    "".to_string()
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
    fn number(&self) -> String {
        String::from("2")
    }

    fn a(&self) {
        print!("{}A: ", self.number());
        let result = process_data_a(INPUT);
        println!("Result = {}", result);
    }

    fn b(&self) {
        print!("{}B: ", self.number());
        let result = process_data_b(INPUT);
        println!("Result = {}", result);
    }
}

#[test]
fn a() {
    assert_eq!(
        process_data_a(
            "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab"
        ),
        12
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(
            "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz"
        ),
        "fgij".to_string()
    );
}
