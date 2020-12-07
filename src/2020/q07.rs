//-----------------------------------------------------
// Setup.

use std::collections::{HashMap, HashSet, VecDeque};

use glue::prelude::{
    alphabetic, digit, eoi, find_all, find_separated, is, optional, take, take_all, Parser,
};
use glue::types::MapParserResult;

static INPUT: &str = include_str!("data/q07.data");

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Bag {
    symbol: String,
    quantity: usize,
}

// shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.

fn bag_parser<'a>() -> impl Parser<'a, Bag> {
    move |ctx| {
        find_all((
            take_all((
                take(1.., is(alphabetic)),
                is(" "),
                take(1.., is(alphabetic)),
            )),
            is(" bag"),
            optional(is("s")),
        ))
        .parse(ctx)
        .map_result(|(name, _, _)| Bag {
            quantity: 1,
            symbol: name.to_owned(),
        })
    }
}

fn multi_bag_parser<'a>() -> impl Parser<'a, Bag> {
    move |ctx| {
        find_all((take(1.., is(digit)), is(" "), bag_parser()))
            .parse(ctx)
            .map_result(|(quantity, _, mut bag)| {
                let quantity: &str = quantity;
                // println!("Found bag {} {:?}", quantity, bag);
                bag.quantity = quantity.parse().unwrap();
                bag
            })
    }
}

fn rule_parser<'a>() -> impl Parser<'a, (Bag, Vec<Bag>)> {
    move |ctx| {
        // faded maroon bags contain no other bags.
        find_all((
            bag_parser(),
            is(" contain "),
            optional(is("no other bags")),
            optional(find_separated(1.., multi_bag_parser(), is(", "))),
            is("."),
        ))
        .parse(ctx)
        .map_result(|(source, _, _none, dests, _)| {
            // println!("Found rule {:?} {:?} {:?}", source, none, dests);
            (source, dests.unwrap_or_default())
        })
    }
}

fn parser<'a>() -> impl Parser<'a, HashMap<String, Vec<Bag>>> {
    move |ctx| {
        find_all((find_separated(1.., rule_parser(), is('\n')), eoi()))
            .parse(ctx)
            .map_result(|(rules, end)| {
                println!("End == {:?}", end);
                let mut rv = HashMap::new();
                for (key, value) in rules {
                    if let Some(previous) = rv.insert(key.symbol, value) {
                        println!("Duplicate Key!!! {:?}", previous);
                    };
                }
                rv
            })
    }
}

fn process_data_a(data: &str) -> usize {
    let rules = parser().parse(data).unwrap().1;

    // println!("{:?}\n{:?}", rules.len(), rules);
    let mut stack = VecDeque::new();
    stack.push_front("shiny gold".to_string());
    let mut gold_holders = HashSet::new();
    while !stack.is_empty() {
        let curr = stack.pop_back().unwrap();
        // println!("Processing {}", &curr);
        for (key, values) in &rules.clone() {
            for bag in values {
                if bag.symbol == curr {
                    // println!("Found! Adding {} to {:?}", &key, gold_holders);
                    if !gold_holders.contains(key) {
                        stack.push_front(key.clone());
                        gold_holders.insert(key.clone());
                    }
                }
            }
        }
    }
    gold_holders.len()
}

fn process_data_b(data: &str) -> usize {
    let rules = parser().parse(data).unwrap().1;

    // println!("{:?}\n{:?}", rules.len(), rules);
    let mut stack = VecDeque::new();
    stack.push_front((1, "shiny gold".to_string()));
    let mut rv: usize = 0;
    while !stack.is_empty() {
        let (mult, curr) = stack.pop_back().unwrap();
        // println!("Processing {:?}, {:?}", (mult, &curr), rules.get(&curr));
        rv += mult;
        for bag in &rules[&curr] {
            stack.push_front((mult * bag.quantity, bag.symbol.clone()));
        }
    }
    rv - 1
}

//-----------------------------------------------------
// Questions.

q_impl!("7");

#[test]
fn a() {
    assert_eq!(
        process_data_a(
            "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
        ),
        4
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(
            "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
        ),
        32
    );
    assert_eq!(
        process_data_b(
            "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."
        ),
        126
    );
}
