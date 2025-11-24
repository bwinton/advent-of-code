//-----------------------------------------------------
// Setup.

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::terminated,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;

static INPUT: &str = include_str!("data/q19.data");

fn designs(i: &str) -> IResult<&str, Vec<String>> {
    let (input, designs) =
        separated_list1(tag("\n"), alpha1.map(|s: &str| s.to_owned())).parse(i)?;
    Ok((input, designs))
}

fn towels(i: &str) -> IResult<&str, Vec<String>> {
    let (input, towels) = terminated(
        separated_list1(tag(", "), alpha1.map(|s: &str| s.to_owned())),
        newline,
    )
    .parse(i)?;
    Ok((input, towels))
}

fn parser(i: &str) -> IResult<&str, (Vec<String>, Vec<String>)> {
    let (input, (mut towels, _, designs)) = (towels, newline, designs).parse(i)?;
    towels.sort_by_key(|s| usize::MAX - s.len());
    Ok((input, (towels, designs)))
}

fn valid(design: &str, towels: &[String]) -> bool {
    if design.is_empty() {
        return true;
    }
    for towel in towels {
        if design.starts_with(towel) && valid(&design[towel.len()..], towels) {
            return true;
        }
    }
    false
}

fn count_valid(design: &str, towels: &[String], seen: &mut HashMap<String, usize>) -> usize {
    if design.is_empty() {
        return 1;
    }
    if seen.contains_key(design) {
        return seen[design];
    }
    let mut rv = 0;
    for towel in towels {
        if design.starts_with(towel) {
            let valid = count_valid(&design[towel.len()..], towels, seen);
            rv += valid;
        }
    }
    seen.insert(design.to_owned(), rv);
    rv
}

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    let (towels, designs) = parser(data).unwrap().1;
    for design in designs {
        if valid(&design, &towels) {
            rv += 1;
        }
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let (towels, designs) = parser(data).unwrap().1;
    let seen = HashMap::new();
    designs
        .par_iter()
        .map(|design| count_valid(design, &towels, &mut seen.clone()))
        .sum()
}

//-----------------------------------------------------
// Questions.

q_impl!("19");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
        "
        )),
        6
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
        "
        )),
        16
    );
}
