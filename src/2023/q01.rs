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
    let forward_re = Regex::new("(one|two|three|four|five|six|seven|eight|nine|[0-9])").unwrap();
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
    ]);

    for line in data.lines() {
        let first = forward_re
            .find_iter(line)
            .map(|x| {
                map.get(x.as_str())
                    .copied()
                    .unwrap_or_else(|| x.as_str().parse().unwrap())
            })
            .next()
            .unwrap();
        let mut last = 0;
        let len = line.len();
        for i in 1..=len {
            if let Some(result) = forward_re.find_iter(&line[len - i..]).next() {
                let result = map
                    .get(result.as_str())
                    .copied()
                    .unwrap_or_else(|| result.as_str().parse().unwrap());
                last = result;
                break;
            }
        }

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
