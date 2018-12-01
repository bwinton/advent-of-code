//-----------------------------------------------------
// Setup.

use aoc::Day;

use std::collections::HashSet;

static INPUT: &'static str = include_str!("data/q01.data");

fn process_data_a(data: &str) -> i64 {
    data.lines().fold(0, |acc, x| acc + x.parse::<i64>().unwrap())
}

fn process_data_b(data: &str) -> i64 {
    let mut curr = 0;
    let mut values = data.lines().map(|x| x.parse::<i64>().unwrap()).cycle();
    let mut seen = HashSet::new();
    
    while !seen.contains(&curr) {
        seen.insert(curr);
        curr += values.next().unwrap();
    }
    curr
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
    assert_eq!(process_data_a("+1
-2
+3
+1"), 3);
    assert_eq!(process_data_a("+1
+1
+1"), 3);
    assert_eq!(process_data_a("+1
+1
-2"), 0);
    assert_eq!(process_data_a("-1
-2
-3"), -6);
}

#[test]
fn b() {
    assert_eq!(process_data_b("+1
-2
+3
+1"), 2);
    assert_eq!(process_data_b("+1
-1"), 0);
    assert_eq!(process_data_b("+3
+3
+4
-2
-4"), 10);
    assert_eq!(process_data_b("-6
+3
+8
+5
-6"), 5);
    assert_eq!(process_data_b("+7
+7
-2
-7
-4"), 14);
}
