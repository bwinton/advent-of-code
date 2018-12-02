//-----------------------------------------------------
// Setup.

use nom::alpha;
use nom::types::CompleteStr;
use regex::Regex;
use std::collections::HashSet;

static INPUT: &'static str = include_str!("data/q19.data");

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

named!(rule_parser<CompleteStr, Rule>, do_parse!(
  source: alpha >>
  tag!(" => ") >>
  dest: alpha >>
  tag!("\n") >>
  (Rule { source: Regex::new(&source).unwrap(), dest: dest.to_string() })
));

named!(parser<CompleteStr, (Vec<Rule>, String)>, complete!(do_parse!(
  rules: many1!(rule_parser) >>
  tag!("\n") >>
  start: alpha >>
  (rules.to_vec(), start.to_string())
)));

fn process_data_a(data: &str) -> usize {
    let (rules, start) = parser(CompleteStr(data)).unwrap().1;
    let mut rv = HashSet::new();
    for rule in rules {
        let matches = rule.match_all(&start);
        rv.extend(matches);
    }
    rv.len()
}

fn process_data_b(data: &str) -> usize {
    let (_, goal) = parser(CompleteStr(data)).unwrap().1;
    let tokens: Vec<String> = Regex::new("[A-Z][a-z]?")
        .unwrap()
        .captures_iter(&goal)
        .map(|x| x[0].to_owned())
        .collect();
    let brackets: Vec<&String> = tokens
        .iter()
        .filter(|&x| *x == "Rn" || *x == "Ar")
        .collect();
    let commas: Vec<&String> = tokens.iter().filter(|&x| *x == "Y").collect();
    tokens.len() - brackets.len() - 2 * commas.len() - 1
}

//-----------------------------------------------------
// Questions.

q_impl!("19");

#[test]
fn test_a() {
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
fn test_b() {}
