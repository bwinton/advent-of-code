//-----------------------------------------------------
// Setup.

use aoc::Day;

static INPUT: &str = include_str!("data/q02.data");

fn process_data_a(data: &str) -> i32 {
    let mut rv = 0;
    for line in data.lines() {
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|i| i.parse::<i32>().unwrap())
            .collect();
        let max = values.iter().max().unwrap();
        let min = values.iter().min().unwrap();
        rv += max - min;
    }
    rv
}

fn process_data_b(data: &str) -> i32 {
    let mut rv = 0;
    for line in data.lines() {
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|i| i.parse::<i32>().unwrap())
            .collect();
        for x in &values {
            for y in &values {
                if x != y && x / y * y == *x {
                    rv += x / y;
                }
            }
        }
    }
    rv
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
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_a("5 1 9 5"), 8);
    assert_eq!(process_data_a("7 5 3"), 4);
    assert_eq!(process_data_a("2 4 6 8"), 6);
    assert_eq!(
        process_data_a(
            "5 1 9 5
7 5 3
2 4 6 8",
        ),
        18
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b("5 9 2 8"), 4);
    assert_eq!(process_data_b("9 4 7 3"), 3);
    assert_eq!(process_data_b("3 8 6 5"), 2);
    assert_eq!(
        process_data_b(
            "5 9 2 8
9 4 7 3
3 8 6 5",
        ),
        9
    );
}
