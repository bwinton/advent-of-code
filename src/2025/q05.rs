//-----------------------------------------------------
// Setup.

use std::ops::RangeInclusive as Range;

use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{line_ending, newline, u64 as value_parser},
    multi::separated_list1,
};

type Values = u64;

static INPUT: &str = include_str!("data/q05.data");

fn range(i: &str) -> IResult<&str, Range<Values>> {
    let (input, (start, _, end)) = (value_parser, tag("-"), value_parser).parse(i)?;
    Ok((input, start..=end))
}

fn ranges(i: &str) -> IResult<&str, Vec<Range<Values>>> {
    let (input, ranges) = separated_list1(newline, range).parse(i)?;
    Ok((input, ranges))
}

fn ingredients(i: &str) -> IResult<&str, Vec<Values>> {
    let (input, ingredients) = separated_list1(newline, value_parser).parse(i)?;
    Ok((input, ingredients))
}

fn parse(i: &str) -> IResult<&str, (Vec<Range<Values>>, Vec<Values>)> {
    let (input, (ranges, _, _, ingredients, _)) =
        (ranges, newline, newline, ingredients, line_ending).parse(i)?;
    let mut consolidated: Vec<Range<Values>> = vec![];
    for range in &ranges {
        let mut value_to_add = range.clone();
        let mut values_to_remove = vec![];
        if consolidated.is_empty() {
            consolidated.push(range.clone());
            continue;
        }
        for (i, test) in consolidated.iter().enumerate() {
            if value_to_add.end() < test.start() || value_to_add.start() > test.end() {
                // They don't overlap at all!
                // So we'll just add the new range.
            } else if value_to_add.start() <= test.start() && value_to_add.end() >= test.end() {
                // Range includes test.
                // So remove test, and add the new range.
                values_to_remove.push(i);
            } else if value_to_add.start() >= test.start() && value_to_add.end() <= test.end() {
                // Test includes range.
                // So make range the same as test.
                values_to_remove.push(i);
                value_to_add = test.clone();
            } else if value_to_add.start() <= test.start() && value_to_add.end() < test.end() {
                values_to_remove.push(i);
                value_to_add = *value_to_add.start()..=*test.end();
            } else if value_to_add.start() > test.start() && value_to_add.end() >= test.end() {
                values_to_remove.push(i);
                value_to_add = *test.start()..=*value_to_add.end();
            } else {
                panic!("Unhandled case!!! {range:?}, {test:?}");
            }
        }

        for index in values_to_remove.into_iter().rev() {
            consolidated.remove(index);
        }
        consolidated.push(value_to_add);
    }
    Ok((input, (consolidated, ingredients)))
}

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    let (_, (ranges, ingredients)) = parse(data).unwrap();

    'outer: for ingredient in ingredients {
        for range in &ranges {
            if range.contains(&ingredient) {
                rv += 1;
                continue 'outer;
            }
        }
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    let (_, (ranges, _)) = parse(data).unwrap();

    for range in &ranges {
        rv += range.try_len().unwrap();
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("5");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "3-5
10-14
16-20
12-18

1
5
8
11
17
32
"
        )),
        3
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "3-5
10-14
16-20
12-18

1
5
8
11
17
32
"
        )),
        14
    );
}
