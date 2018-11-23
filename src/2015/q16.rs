//-----------------------------------------------------
// Setup.

use aoc::Day;

use nom;
use nom::alpha;
use std::collections::HashMap;
use std::str;
use std::str::FromStr;

static INPUT: &'static str = include_str!("data/q16.data");

#[derive(Debug)]
struct AuntSue {
    name: Option<u32>,
    features: HashMap<String, u32>,
}

impl AuntSue {
    fn matches_a(&self, other: &AuntSue) -> bool {
        let mut rv = true;
        for feature in other.features.keys() {
            rv &= other.features[feature] == self.features[feature];
        }
        rv
    }

    fn matches_b(&self, other: &AuntSue) -> bool {
        let mut rv = true;
        for feature in other.features.keys() {
            match feature.as_str() {
                "cats" | "trees" => {
                    rv &= other.features[feature] > self.features[feature];
                }
                "pomeranians" | "goldfish" => {
                    rv &= other.features[feature] < self.features[feature];
                }
                _ => {
                    rv &= other.features[feature] == self.features[feature];
                }
            }
        }
        rv
    }
}

named!(digits<&str, u32>, map_res!(
  nom::digit,
  u32::from_str
));

named!(aunt_name_parser<&str, u32>, do_parse!(
  tag!("Sue ") >>
  name: digits >>
  tag!(": ") >>
  (name)
));

named!(feature_parser<&str, (String, u32)>, do_parse!(
  pair: separated_pair!(alpha, tag!(": "), digits) >>
  (pair.0.to_owned(), pair.1)
));

named!(aunt_parser<&str, AuntSue>, complete!(do_parse!(
  name: aunt_name_parser >>
  features: separated_list_complete!(tag!(", "), feature_parser) >>
  (AuntSue {
    name: Some(name),
    features: features.iter().cloned().collect()
  }))
));

fn process_data_a(data: &str) -> u32 {
    let gifter = AuntSue {
        name: None,
        features: hashmap!{
          "children".to_string() => 3,
          "cats".to_string() => 7,
          "samoyeds".to_string() => 2,
          "pomeranians".to_string() => 3,
          "akitas".to_string() => 0,
          "vizslas".to_string() => 0,
          "goldfish".to_string() => 5,
          "trees".to_string() => 3,
          "cars".to_string() => 2,
          "perfumes".to_string() => 1
        },
    };
    let mut aunts_sue = Vec::new();
    for line in data.lines() {
        let mut aunt_sue = aunt_parser(line).unwrap().1;
        aunts_sue.push(aunt_sue);
    }
    for aunt_sue in aunts_sue {
        if gifter.matches_a(&aunt_sue) {
            return aunt_sue.name.unwrap();
        }
    }
    0
}

fn process_data_b(data: &str) -> u32 {
    let gifter = AuntSue {
        name: None,
        features: hashmap!{
          "children".to_string() => 3,
          "cats".to_string() => 7,
          "samoyeds".to_string() => 2,
          "pomeranians".to_string() => 3,
          "akitas".to_string() => 0,
          "vizslas".to_string() => 0,
          "goldfish".to_string() => 5,
          "trees".to_string() => 3,
          "cars".to_string() => 2,
          "perfumes".to_string() => 1
        },
    };

    let mut aunts_sue = Vec::new();
    for line in data.lines() {
        let mut aunt_sue = aunt_parser(line).unwrap().1;
        aunts_sue.push(aunt_sue);
    }
    for aunt_sue in aunts_sue {
        if gifter.matches_b(&aunt_sue) {
            // println!("Found match: {:?}", aunt_sue);
            return aunt_sue.name.unwrap();
        }
    }
    0
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
    fn number(&self) -> String {
        String::from("16")
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
    // println!("{:?}", aunt_parser("Sue 1: cars: 9, akitas: 3, goldfish: 0"));
    // assert_eq!(1, 0);
}

#[test]
fn b() {
    // assert_eq!(process_data_b(""), 0);
}
