//-----------------------------------------------------
// Setup.

use std::collections::HashSet;

use glue::{
    prelude::{alphabetic, find, find_all, is, take, Parser},
    types::MapParserResult,
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

fn rule_parser<'a>() -> impl Parser<'a, Rule> {
    move |ctx| {
        find_all((
            take(1.., is(alphabetic)),
            is(" => "),
            take(1.., is(alphabetic)),
            is("\n"),
        ))
        .parse(ctx)
        .map_result(|(source, _, dest, _)| Rule {
            source: Regex::new(source).unwrap(),
            dest: dest.to_string(),
        })
    }
}

fn parser<'a>() -> impl Parser<'a, (Vec<Rule>, String)> {
    move |ctx| {
        find_all((
            find(1.., rule_parser()),
            is("\n"),
            take(1.., is(alphabetic)),
        ))
        .parse(ctx)
        .map_result(|(rules, _, start)| (rules, start.to_string()))
    }
}

fn process_data_a(data: &str) -> usize {
    let (rules, start) = parser().parse(data).unwrap().1;
    let mut rv = HashSet::new();
    for rule in rules {
        let matches = rule.match_all(&start);
        rv.extend(matches);
    }
    rv.len()
}

fn process_data_b(data: &str) -> usize {
    let (_, goal) = parser().parse(data).unwrap().1;
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
