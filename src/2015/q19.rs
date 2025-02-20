//-----------------------------------------------------
// Setup.

use std::collections::HashSet;

use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    multi::many1,
    sequence::tuple,
};
use regex::Regex;

static INPUT: &str = include_str!("data/q19.data");

#[derive(Clone, Debug)]
struct Rule {
    source: Regex,
    dest: String,
}

impl Rule {
    fn match_all(self, start: &str) -> Vec<String> {
        let mut rv = Vec::new();
        for found in self.source.find_iter(start) {
            let mut dest = start.to_owned();
            dest.replace_range(found.start()..found.end(), &self.dest);
            rv.push(dest);
        }
        rv
    }
}

fn rule(i: &str) -> IResult<&str, Rule> {
    let (input, (source, _, dest, _)) = tuple((alpha1, tag(" => "), alpha1, line_ending))(i)?;
    Ok((
        input,
        Rule {
            source: Regex::new(source).unwrap(),
            dest: dest.to_string(),
        },
    ))
}

fn parser(i: &str) -> IResult<&str, (Vec<Rule>, String)> {
    let (input, (rules, _, start)) = tuple((many1(rule), line_ending, alpha1))(i)?;
    Ok((input, (rules, start.to_string())))
}

fn process_data_a(data: &str) -> usize {
    let (rules, start) = parser(data).unwrap().1;
    let mut rv = HashSet::new();
    for rule in rules {
        let matches = rule.match_all(&start);
        rv.extend(matches);
    }
    rv.len()
}

fn process_data_b(data: &str) -> usize {
    let (_, goal) = parser(data).unwrap().1;
    let tokens: Vec<String> = Regex::new("[A-Z][a-z]?")
        .unwrap()
        .captures_iter(&goal)
        .map(|x| x[0].to_owned())
        .collect();
    let brackets = tokens.iter().filter(|&x| *x == "Rn" || *x == "Ar");
    let commas = tokens.iter().filter(|&x| *x == "Y");
    tokens.len() - brackets.count() - 2 * commas.count() - 1
}

//-----------------------------------------------------
// Questions.

q_impl!("19");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(
            "H => HO
H => OH
O => HH

HOH"
        ),
        4
    );
    assert_eq!(
        process_data_a(
            "H => HO
H => OH
O => HH

HOHOHO"
        ),
        7
    );
}

#[test]
fn b() {}
