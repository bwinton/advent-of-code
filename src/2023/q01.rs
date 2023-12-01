//-----------------------------------------------------
// Setup.

use std::collections::HashMap;

use regex::Regex;

static INPUT: &str = include_str!("data/q01.data");

fn process_data_a(data: &str) -> u32 {
    let mut rv = 0;
    for line in data.lines() {
        let mut digits = line.chars().filter(|x| x.is_ascii_digit());
        let first = digits.clone().next().unwrap().to_digit(10).unwrap();
        let last = digits.next_back().unwrap().to_digit(10).unwrap();
        rv += first * 10 + last;
    }
    rv
    // 54667
}

fn process_data_b(data: &str) -> u32 {
    let mut rv = 0;
    let digits = r"one|two|three|four|five|six|seven|eight|nine";
    let forward_re = Regex::new(&format!("({}|[0-9])", digits)).unwrap();
    let digits = digits.chars().rev().collect::<String>();
    let backward_re = Regex::new(&format!("({}|[0-9])", digits)).unwrap();
    let map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("eno", 1),
        ("owt", 2),
        ("eerht", 3),
        ("ruof", 4),
        ("evif", 5),
        ("xis", 6),
        ("neves", 7),
        ("thgie", 8),
        ("enin", 9),
    ]);

    for line in data.lines() {
        let first = forward_re
            .find_iter(line)
            .filter_map(|x| map.get(x.as_str()))
            .next()
            .unwrap();
        let last = backward_re
            .find_iter(&line.chars().rev().collect::<String>())
            .filter_map(|x| map.get(x.as_str()))
            .next()
            .unwrap();

        rv += first * 10 + last;
    }
    rv
    // 54203
}

//-----------------------------------------------------
// Questions.

q_impl!("1");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet
    "
        )),
        142
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen"
        )),
        281
    );
}
