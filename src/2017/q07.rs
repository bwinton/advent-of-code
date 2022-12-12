//-----------------------------------------------------
// Setup.

use aoc::Day;

use regex::Regex;
use std::{collections::HashMap, str::FromStr, string::ToString};

static INPUT: &str = include_str!("data/q07.data");

#[derive(Clone, Debug)]
struct Disc {
    name: String,
    weight: usize,
    sum: usize,
    holdings: Vec<String>,
}

impl FromStr for Disc {
    type Err = ();

    fn from_str(s: &str) -> Result<Disc, ()> {
        let main_re: &Regex = regex!(r"^([a-z]+) \((\d+)\)( -> (([a-z]+(, )?)+))?$");
        let mut rv = Disc {
            name: "".to_string(),
            weight: 0,
            sum: 0,
            holdings: Vec::new(),
        };
        let cap = main_re.captures(s);
        match cap {
            None => Err(()),
            Some(x) => {
                rv.name = x[1].to_string();
                rv.weight = x[2].parse().unwrap();
                if let Some(rest) = x.get(4) {
                    rv.holdings = rest.as_str().split(", ").map(ToString::to_string).collect();
                }
                Ok(rv)
            }
        }
    }
}

fn process_data_a(data: &str) -> String {
    let mut discs = HashMap::new();
    for line in data.lines() {
        let disc: Disc = line.parse().unwrap();
        discs.insert(disc.name.to_string(), disc);
    }
    let mut roots = discs.clone();
    for disc in discs.values() {
        for child in &disc.holdings {
            roots.remove(child);
        }
    }
    roots.keys().next().unwrap().to_string()
}

fn get_sums(discs: &mut HashMap<String, Disc>) -> (HashMap<String, Disc>, Option<usize>) {
    let mut rv = discs.clone();
    let lookup = discs.clone();
    for disc in discs.values() {
        let mut new = disc.clone();
        if new.sum != 0 {
            continue;
        }
        if new.holdings.is_empty() {
            new.sum = new.weight;
            rv.insert(disc.name.to_string(), new);
            continue;
        }
        let child_sums: Vec<&usize> = new.holdings.iter().map(|key| &lookup[key].sum).collect();
        if child_sums.iter().all(|x| *x != &0) && !child_sums.iter().all(|x| *x == child_sums[0]) {
            let temp: Vec<_> = child_sums
                .iter()
                .enumerate()
                .filter(|i| *i.1 != child_sums[0])
                .collect();
            let mut regular = *child_sums[0];
            let mut odd = temp[0];
            if temp.len() > 1 {
                regular = **odd.1;
                odd = (0, &child_sums[0]);
            }
            let diff = **odd.1 as i32 - regular as i32;
            let new_weight = &lookup[&new.holdings[odd.0]].weight;
            // println!("Found it: {:?}", new);
            // println!("  c: {:?}, o: {:?}\n  {:?}", child_sums, odd, &lookup[&new.holdings[odd.0]]);
            // println!("  d: {:?}, n: {:?}", diff, new_weight);
            return (rv, Some((*new_weight as i32 - diff) as usize));
        }
        if child_sums.iter().all(|x| *x != &0) {
            new.sum = child_sums.iter().map(|x| **x).sum();
            new.sum += new.weight;
            rv.insert(disc.name.to_string(), new);
        }
    }
    (rv, None)
}

fn process_data_b(data: &str) -> usize {
    let mut discs = HashMap::new();
    for line in data.lines() {
        let disc: Disc = line.parse().unwrap();
        discs.insert(disc.name.to_string(), disc.clone());
    }
    let mut rv;
    while discs.values().any(|d| d.sum == 0) {
        rv = get_sums(&mut discs);
        discs = rv.0;
        match rv.1 {
            None => {}
            Some(value) => {
                return value;
            }
        }
    }
    unreachable!();
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
    fn number(&self) -> String {
        String::from("7")
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
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(
            "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)",
        ),
        "tknk"
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(
            "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)",
        ),
        60
    );
}
