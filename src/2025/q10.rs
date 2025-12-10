//-----------------------------------------------------
// Setup.

use std::collections::{BinaryHeap, HashMap, HashSet};

use itertools::Itertools;

use nom::{
    IResult, Parser,
    bytes::complete::is_a,
    character::complete::{char, line_ending, space1, u32 as value_parser},
    multi::{many1, separated_list1},
    sequence::{delimited, terminated},
};

static INPUT: &str = include_str!("data/q10.data");

type Value = u32;
type Line = (Value, Vec<(Value, Vec<Value>)>, Vec<Value>);

fn indicator(i: &str) -> IResult<&str, Value> {
    let (input, values) = delimited(char('['), is_a(".#"), (char(']'), space1)).parse(i)?;
    let values = values.replace('.', "0").replace('#', "1");
    let values: String = values.chars().rev().collect();
    let indicators = Value::from_str_radix(&values, 2).expect("{values} isn't a binary number!");
    Ok((input, indicators))
}

fn button(i: &str) -> IResult<&str, (Value, Vec<Value>)> {
    let (input, values) = delimited(
        char('('),
        separated_list1(char(','), value_parser),
        char(')'),
    )
    .parse(i)?;
    let mut value = 0;
    for bit in &values {
        value |= 1 << bit;
    }
    Ok((input, (value, values)))
}

fn buttons(i: &str) -> IResult<&str, Vec<(Value, Vec<Value>)>> {
    let (input, buttons) = many1(terminated(button, space1)).parse(i)?;
    Ok((input, buttons))
}

fn joltages(i: &str) -> IResult<&str, Vec<Value>> {
    let (input, values) = delimited(
        char('{'),
        separated_list1(char(','), value_parser),
        char('}'),
    )
    .parse(i)?;
    Ok((input, values))
}

fn line(i: &str) -> IResult<&str, Line> {
    let (input, (indicator, buttons, joltages, _)) =
        (indicator, buttons, joltages, line_ending).parse(i)?;
    Ok((input, (indicator, buttons, joltages)))
}

fn parse(i: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = many1(line).parse(i)?;
    Ok((input, lines))
}

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    let (_, lines) = parse(data).unwrap();
    for (indicator, buttons, _joltages) in lines {
        // Actually do the processing…
        let mut found = false;
        let mut stack = BinaryHeap::new();
        let mut seen = HashSet::new();
        stack.push((0, 0));
        'outer: while !stack.is_empty() {
            let (moves, curr) = stack.pop().unwrap();
            if seen.contains(&curr) {
                continue;
            }
            seen.insert(curr);
            for (button, _) in &buttons {
                let new = curr ^ button;
                if new == indicator {
                    rv -= moves - 1;
                    found = true;
                    break 'outer;
                }
                if !seen.contains(&new) {
                    stack.push((moves - 1, new));
                }
            }
        }
        if !found {
            println!("Whoops!");
        }
    }
    rv as usize
}

fn presses(
    target: &Vec<Value>,
    patterns: &HashMap<Vec<Value>, Vec<Vec<Value>>>,
    ops: &HashMap<Vec<Value>, Vec<Value>>,
    cache: &mut HashMap<Vec<Value>, Value>,
) -> Value {
    if cache.contains_key(target) {
        return cache[target];
    }
    // if all(x == 0 for x in target): return 0
    if target.iter().all(|&x| x == 0) {
        cache.insert(target.clone(), 0);
        return 0;
    }
    let mut total = Value::MAX;
    let lights: Vec<Value> = target.iter().map(|&x| x % 2).collect();
    if let Some(pattern) = patterns.get(&lights) {
        for pressed in pattern {
            let diff = ops[pressed].clone();
            let new_target: Vec<Value> = diff
                .into_iter()
                .zip(target.iter())
                .filter_map(|(a, &b)| if a <= b { Some((b - a) / 2) } else { None })
                .collect();
            // Handle negative numbers, in a kinda terrible way.
            let presses = if new_target.len() == target.len() {
                pressed
                    .iter()
                    .sum::<Value>()
                    .saturating_add(presses(&new_target, patterns, ops, cache).saturating_mul(2))
            } else {
                Value::MAX
            };
            total = total.min(presses)
        }
    }
    cache.insert(target.clone(), total);
    total
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    let (_, lines) = parse(data).unwrap();
    for (_indicator, buttons, joltages) in lines {
        let mut ops: HashMap<Vec<Value>, Vec<Value>> = HashMap::new();
        let mut patterns: HashMap<Vec<Value>, Vec<Vec<Value>>> = HashMap::new();
        let temp = (0..buttons.len()).map(|_| 0..=1).multi_cartesian_product();
        for pressed in temp {
            let mut jolt = vec![0 as Value; joltages.len()];
            for (i, p) in pressed.iter().enumerate() {
                for &j in &buttons[i].1 {
                    jolt[j as usize] += p;
                }
            }
            let lights: Vec<_> = jolt.iter().map(|x| x % 2).collect();
            patterns.entry(lights).or_default().push(pressed.clone());
            ops.insert(pressed, jolt);
        }

        // These are both okay!
        // println!("ops: {ops:?}");
        // println!("patterns: {patterns:?}");
        // println!("\nTarget {joltages:?}");
        rv += presses(&joltages, &patterns, &ops, &mut HashMap::new());
    }
    rv as usize
    // 20298
}

//-----------------------------------------------------
// Questions.

q_impl!("10");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
            "
        )),
        2
    );
    assert_eq!(
        process_data_a(indoc!(
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
            "
        )),
        3
    );
    assert_eq!(
        process_data_a(indoc!(
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
            "
        )),
        2
    );
    assert_eq!(
        process_data_a(indoc!(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
            [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
            [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
            "
        )),
        7
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
            "
        )),
        10
    );
    assert_eq!(
        process_data_b(indoc!(
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
            "
        )),
        12
    );
    assert_eq!(
        process_data_b(indoc!(
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
            "
        )),
        11
    );
    assert_eq!(
        process_data_b(indoc!(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
            [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
            [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
            "
        )),
        33
    );
}
