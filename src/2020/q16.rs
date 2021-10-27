//-----------------------------------------------------
// Setup.

use std::{collections::HashMap, ops::RangeInclusive};

use regex::Regex;

static INPUT: &str = include_str!("data/q16.data");

lazy_static! {
    // departure date: 26-827 or 843-970

    static ref RULE_RE: Regex = Regex::new(r"^(.*): ([0-9]*)-([0-9]*) or ([0-9]*)-([0-9]*)$").unwrap();
}

#[derive(Debug)]
struct Rule {
    name: String,
    range_one: RangeInclusive<usize>,
    range_two: RangeInclusive<usize>,
}

impl Rule {
    fn validate(&self, field: usize) -> bool {
        self.range_one.contains(&field) || self.range_two.contains(&field)
    }
}

#[derive(Debug)]
enum State {
    Rules,
    MyTicket,
    Nearby,
}

fn process_data_a(data: &str) -> usize {
    let mut rules = vec![];
    let mut nearby: Vec<Vec<usize>> = vec![];
    let mut state = State::Rules;
    for line in data.lines() {
        // Do something
        match state {
            State::Rules => {
                if line.is_empty() {
                    continue;
                } else if line == "your ticket:" {
                    state = State::MyTicket;
                    continue;
                }
                if let Some(captures) = RULE_RE.captures(line) {
                    let name = captures[1].to_string();
                    let v1 = captures[2].parse().unwrap();
                    let v2 = captures[3].parse().unwrap();
                    let range_one = v1..=v2;
                    let v1 = captures[4].parse().unwrap();
                    let v2 = captures[5].parse().unwrap();
                    let range_two = v1..=v2;
                    rules.push(Rule {
                        name,
                        range_one,
                        range_two,
                    });
                };
            }
            State::MyTicket => {
                if line.is_empty() {
                    continue;
                } else if line == "nearby tickets:" {
                    state = State::Nearby;
                    continue;
                }
            }
            State::Nearby => {
                nearby.push(line.split(',').map(|x| x.parse().unwrap()).collect());
            }
        }
    }

    let mut invalids = vec![];
    // println!("Rules: {:?}\nMe: {:?}\nNearby: {:?}", rules, my_ticket, nearby);
    for ticket in nearby {
        for field in ticket {
            let mut valid = false;
            for rule in &rules {
                valid |= rule.validate(field);
            }
            if !valid {
                invalids.push(field);
            }
        }
    }
    // println!("Invalid: {:?}", invalids);
    invalids.iter().sum()
}

fn process_data_b(data: &str) -> usize {
    let mut rules = vec![];
    let mut my_ticket: Vec<usize> = vec![];
    let mut nearby: Vec<Vec<usize>> = vec![];
    let mut state = State::Rules;
    for line in data.lines() {
        // Do something
        match state {
            State::Rules => {
                if line.is_empty() {
                    continue;
                } else if line == "your ticket:" {
                    state = State::MyTicket;
                    continue;
                }
                if let Some(captures) = RULE_RE.captures(line) {
                    let name = captures[1].to_string();
                    let v1 = captures[2].parse().unwrap();
                    let v2 = captures[3].parse().unwrap();
                    let range_one = v1..=v2;
                    let v1 = captures[4].parse().unwrap();
                    let v2 = captures[5].parse().unwrap();
                    let range_two = v1..=v2;
                    rules.push(Rule {
                        name,
                        range_one,
                        range_two,
                    });
                };
            }
            State::MyTicket => {
                if line.is_empty() {
                    continue;
                } else if line == "nearby tickets:" {
                    state = State::Nearby;
                    continue;
                }
                my_ticket = line.split(',').map(|x| x.parse().unwrap()).collect();
            }
            State::Nearby => {
                nearby.push(line.split(',').map(|x| x.parse().unwrap()).collect());
            }
        }
    }

    let mut invalids = vec![];
    // println!("Rules: {:?}\nMe: {:?}\nNearby: {:?}", rules, my_ticket, nearby);
    for ticket in &nearby {
        for field in ticket {
            let mut valid = false;
            for rule in &rules {
                valid |= rule.validate(*field);
            }
            if !valid {
                invalids.push(ticket.clone());
            }
        }
    }
    let valids: Vec<_> = nearby.iter().filter(|x| !invalids.contains(x)).collect();
    // println!("Valids: {:?}", &valids);

    let mut position_values = vec![];
    for i in 0..my_ticket.len() {
        let mut value = vec![];
        for ticket in &valids {
            value.push(ticket[i]);
        }
        position_values.push(value);
    }

    // println!("{:?}", position_values);

    let mut rule_positions: HashMap<String, Vec<usize>> = HashMap::new();
    for rule in rules {
        for (i, values) in position_values.iter().enumerate() {
            let mut valid = true;
            for value in values {
                valid &= rule.validate(*value);
            }
            if valid {
                rule_positions
                    .entry(rule.name.clone())
                    .or_insert_with(Vec::new)
                    .push(i);
            }
        }
    }
    // println!("{:?}", &rule_positions);

    let mut final_positions: HashMap<String, usize> = HashMap::new();
    while !rule_positions.is_empty() {
        let mut found = None;
        for (name, positions) in &rule_positions {
            if positions.len() == 1 {
                found = Some((name.clone(), positions[0]));
                final_positions.insert(name.clone(), positions[0]);
            }
        }
        if let Some((name, position)) = found {
            rule_positions.remove(&name);
            for (_, positions) in rule_positions.iter_mut() {
                positions.retain(|&x| x != position);
            }
        } else {
            panic!("Could not find a unique rule in {:?}!", rule_positions);
        }
    }

    // println!("{:?}", &final_positions);

    final_positions
        .iter()
        .filter(|(name, _)| name.starts_with("departure"))
        .map(|(_, &value)| my_ticket[value])
        .product()
    // 61*89*197*193*157*181
}

//-----------------------------------------------------
// Questions.

q_impl!("16");

#[test]
fn a() {
    assert_eq!(
        process_data_a(
            "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
"
        ),
        71
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(
            "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"
        ),
        1
    );

    assert_eq!(
        process_data_b(
            "class: 1-3 or 5-7
    row: 6-11 or 33-44
    seat: 13-40 or 45-50

    your ticket:
    7,1,14

    nearby tickets:
    7,3,47
    40,4,50
    55,2,20
    38,6,12"
        ),
        1
    );
}
