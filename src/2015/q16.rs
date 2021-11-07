//-----------------------------------------------------
// Setup.

use std::{collections::HashMap, str};

use glue::{
    prelude::{alphabetic, digit, find_all, find_separated, is, take, Parser},
    types::MapParserResult,
};

static INPUT: &str = include_str!("data/q16.data");

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

fn aunt_name_parser<'a>() -> impl Parser<'a, u32> {
    move |ctx| {
        find_all((is("Sue "), take(1.., is(digit)), is(": ")))
            .parse(ctx)
            .map_result(|(_, name, _)| name.parse().unwrap())
    }
}

fn feature_parser<'a>() -> impl Parser<'a, (String, u32)> {
    move |ctx| {
        find_all((take(1.., is(alphabetic)), is(": "), take(1.., is(digit))))
            .parse(ctx)
            .map_result(|(name, _, amount)| (name.to_string(), amount.parse().unwrap()))
    }
}

fn aunt_parser<'a>() -> impl Parser<'a, AuntSue> {
    move |ctx| {
        find_all((
            aunt_name_parser(),
            find_separated(0.., feature_parser(), is(", ")),
        ))
        .parse(ctx)
        .map_result(|(name, features)| AuntSue {
            name: Some(name),
            features: features.iter().cloned().collect(),
        })
    }
}

fn process_data_a(data: &str) -> u32 {
    let gifter = AuntSue {
        name: None,
        features: hashmap![
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
        ],
    };
    let mut aunts_sue = Vec::new();
    for line in data.lines() {
        let aunt_sue = aunt_parser().parse(line).unwrap().1;
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
        features: hashmap![
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
        ],
    };

    let mut aunts_sue = Vec::new();
    for line in data.lines() {
        let aunt_sue = aunt_parser().parse(line).unwrap().1;
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

q_impl!("16");

#[test]
fn a() {
    // println!("{:?}", aunt_parser("Sue 1: cars: 9, akitas: 3, goldfish: 0"));
    // assert_eq!(1, 0);
}

#[test]
fn b() {
    // assert_eq!(process_data_b(""), 0);
}
