//-----------------------------------------------------
// Setup.

use std::collections::{HashMap, HashSet, VecDeque};

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, i32, line_ending},
    combinator::{eof, opt},
    multi::{separated_list0, separated_list1},
    sequence::{terminated, tuple},
};

static INPUT: &str = include_str!("data/q07.data");

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Bag {
    symbol: String,
    quantity: usize,
}

// shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
fn bag(i: &str) -> IResult<&str, Bag> {
    let (input, (mod1, space, mod2, _, _)) =
        tuple((alpha1, tag(" "), alpha1, tag(" bag"), opt(tag("s"))))(i)?;

    let name = format!("{}{}{}", mod1, space, mod2);

    Ok((input, Bag {
        quantity: 1,
        symbol: name,
    }))
}

fn multi_bag(i: &str) -> IResult<&str, Bag> {
    let (input, (quantity, _, mut bag)) = tuple((i32, tag(" "), bag))(i)?;
    bag.quantity = quantity as usize;
    Ok((input, bag))
}

fn rule(i: &str) -> IResult<&str, (Bag, Vec<Bag>)> {
    // faded maroon bags contain no other bags.
    let (input, (source, _, dests, _)) = tuple((
        bag,
        tag(" contain "),
        alt((
            tag("no other bags").map(|_| Vec::new()),
            separated_list1(tag(", "), multi_bag),
        )),
        tag("."),
    ))(i)?;

    Ok((input, (source, dests)))
}

fn parser(i: &str) -> IResult<&str, HashMap<String, Vec<Bag>>> {
    let (input, rules) = terminated(separated_list0(line_ending, rule), eof)(i)?;

    let mut rv = HashMap::new();
    for (key, value) in rules {
        if let Some(previous) = rv.insert(key.symbol, value) {
            println!("Duplicate Key!!! {:?}", previous);
        };
    }

    Ok((input, rv))
}

fn process_data_a(data: &str) -> usize {
    let rules = parser(data).unwrap().1;

    let mut stack = VecDeque::new();
    stack.push_front("shiny gold".to_string());
    let mut gold_holders = HashSet::new();
    while !stack.is_empty() {
        let curr = stack.pop_back().unwrap();
        for (key, values) in &rules.clone() {
            for bag in values {
                if bag.symbol == curr && !gold_holders.contains(key) {
                    stack.push_front(key.clone());
                    gold_holders.insert(key.clone());
                }
            }
        }
    }
    gold_holders.len()
}

fn process_data_b(data: &str) -> usize {
    let rules = parser(data).unwrap().1;

    let mut stack = VecDeque::new();
    stack.push_front((1, "shiny gold".to_string()));
    let mut rv: usize = 0;
    while !stack.is_empty() {
        let (mult, curr) = stack.pop_back().unwrap();
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
    use pretty_assertions::assert_eq;

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
    use pretty_assertions::assert_eq;

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
