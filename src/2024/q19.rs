//-----------------------------------------------------
// Setup.

use nom::{bytes::complete::tag, character::complete::{alpha1, newline}, multi::separated_list1, sequence::{terminated, tuple}, IResult, Parser};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

static INPUT: &str = include_str!("data/q19.data");

fn designs(i: &str) -> IResult<&str, Vec<String>> {
    let (input, designs) = separated_list1(tag("\n"), alpha1.map(|s: &str| s.to_owned()))(i)?;
    Ok((input, designs))
}

fn towels(i: &str) -> IResult<&str, Vec<String>> {
    let (input, towels) = terminated(
        separated_list1(tag(", "), alpha1.map(|s: &str| s.to_owned())),
        newline
    )(i)?;
    Ok((input, towels))
}

fn parser(i: &str) -> IResult<&str, (Vec<String>, Vec<String>)> {
    let (input, (mut towels, _, designs)) = tuple((towels, newline, designs))(i)?;
    towels.sort_by_key(|s| usize::MAX - s.len());
    Ok((input, (towels, designs)))
}

fn valid(design: &str, towels: &[String]) -> bool {
    // println!("Valid: {:?}", design);
    if design.is_empty() {
        return true;
    }
    for towel in towels {
        if design.starts_with(towel) {
            if valid(&design[towel.len()..], towels) {
                return true;
            }
        }
    }
    false
}

fn count_valid(design: &str, towels: &[String]) -> usize {
    // println!("Valid: {:?}", design);
    if design.is_empty() {
        return 1;
    }
    let mut rv = 0;
    for towel in towels {
        if design.starts_with(towel) {
            rv += count_valid(&design[towel.len()..], towels);
        }
    }
    rv
}

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    let (towels, designs) = parser(data).unwrap().1;
    println!("\ntowels: {:?}\n designs: {:?}", towels, designs);
    for design in designs {
        if valid(&design, &towels) {
            rv += 1;
        }
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let (towels, designs) = parser(data).unwrap().1;
    println!("\ntowels: {:?}\n designs: {:?}", towels, designs);
    designs.par_iter().map(|design| {
        count_valid(&design, &towels)
    }).sum()
    // rv
}

//-----------------------------------------------------
// Questions.

q_impl!("19");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_a(indoc!("
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
        ")), 6);
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b(indoc!("
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
        ")), 16);
}
