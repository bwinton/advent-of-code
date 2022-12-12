//-----------------------------------------------------
// Setup.

use std::{collections::HashMap, str};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, u64},
    multi::separated_list0,
    sequence::tuple,
    IResult,
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

fn aunt_name(i: &str) -> IResult<&str, u32> {
    let (input, (_, name, _)) = tuple((tag("Sue "), u64, tag(": ")))(i)?;
    Ok((input, name as u32))
}

fn feature(i: &str) -> IResult<&str, (String, u32)> {
    let (input, (name, _, amount)) = tuple((alpha1, tag(": "), u64))(i)?;
    Ok((input, (name.to_string(), amount as u32)))
}

fn aunt(i: &str) -> IResult<&str, AuntSue> {
    let (input, (name, features)) = tuple((aunt_name, separated_list0(tag(", "), feature)))(i)?;
    Ok((
        input,
        AuntSue {
            name: Some(name),
            features: features.iter().cloned().collect(),
        },
    ))
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
        let aunt_sue = aunt(line).unwrap().1;
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
        let aunt_sue = aunt(line).unwrap().1;
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
    // use pretty_assertions::assert_eq;

    // println!("{:?}", aunt_parser("Sue 1: cars: 9, akitas: 3, goldfish: 0"));
    // assert_eq!(1, 0);
}

#[test]
fn b() {
    // use pretty_assertions::assert_eq;

    // assert_eq!(process_data_b(""), 0);
}
