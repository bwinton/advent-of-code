//-----------------------------------------------------
// Setup.

use aoc::Day;

static INPUT: &str = include_str!("data/q09.data");

fn process_data_a(data: &str) -> i32 {
    let mut rv = 0;
    let mut group_count = 0;
    let mut skip_next = false;
    let mut in_garbage = false;
    for curr in data.chars() {
        if skip_next {
            skip_next = false;
            continue;
        }
        if in_garbage {
            match curr {
                '!' => skip_next = true,
                '>' => in_garbage = false,
                _ => {}
            }
        } else {
            match curr {
                '!' => skip_next = true,
                '{' => {
                    group_count += 1;
                    rv += group_count;
                }
                '}' => group_count -= 1,
                '<' => in_garbage = true,
                _ => {}
            }
        }
    }
    rv
}

fn process_data_b(data: &str) -> i32 {
    let mut rv = 0;
    let mut skip_next = false;
    let mut in_garbage = false;
    for curr in data.chars() {
        if skip_next {
            skip_next = false;
            continue;
        }
        if in_garbage {
            match curr {
                '!' => skip_next = true,
                '>' => in_garbage = false,
                _ => rv += 1,
            }
        } else {
            match curr {
                '!' => skip_next = true,
                '<' => in_garbage = true,
                _ => {}
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
        String::from("9")
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

    assert_eq!(process_data_a("{}"), 1);
    assert_eq!(process_data_a("{{{}}}"), 6);
    assert_eq!(process_data_a("{{},{}}"), 5);
    assert_eq!(process_data_a("{{{},{},{{}}}}"), 16);
    assert_eq!(process_data_a("{<a>,<a>,<a>,<a>}"), 1);
    assert_eq!(process_data_a("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
    assert_eq!(process_data_a("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
    assert_eq!(process_data_a("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b("<>"), 0);
    assert_eq!(process_data_b("<random characters>"), 17);
    assert_eq!(process_data_b("<<<<>"), 3);
    assert_eq!(process_data_b("<{!>}>"), 2);
    assert_eq!(process_data_b("<!!>"), 0);
    assert_eq!(process_data_b("<!!!>>"), 0);
    assert_eq!(process_data_b("<{o\"i!a,<{i<a>"), 10);
}
