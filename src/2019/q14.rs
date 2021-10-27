//-----------------------------------------------------
// Setup.

use std::collections::{HashMap, VecDeque};

use glue::prelude::{alphabetic, digit, find, find_all, find_separated, is, take, Parser};
use glue::types::MapParserResult;

static INPUT: &str = include_str!("data/q14.data");

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Element<'a> {
    symbol: &'a str,
    quantity: usize,
}

fn element_parser<'a>() -> impl Parser<'a, Element<'a>> {
    move |ctx| {
        find_all((take(1.., is(digit)), is(" "), take(1.., is(alphabetic))))
            .parse(ctx)
            .map_result(|(quantity, _, symbol)| Element {
                quantity: quantity.parse().unwrap(),
                symbol,
            })
    }
}

fn rule_parser<'a>() -> impl Parser<'a, (Element<'a>, Vec<Element<'a>>)> {
    move |ctx| {
        find_all((
            find_separated(1.., element_parser(), is(", ")),
            is(" => "),
            element_parser(),
            is("\n"),
        ))
        .parse(ctx)
        .map_result(|(sources, _, dest, _)| (dest, sources))
    }
}

fn parser<'a>() -> impl Parser<'a, HashMap<&'a str, (usize, Vec<Element<'a>>)>> {
    move |ctx| {
        find(1.., rule_parser()).parse(ctx).map_result(|rules| {
            let mut rv = HashMap::new();
            for (key, value) in rules {
                if let Some(previous) = rv.insert(key.symbol, (key.quantity, value)) {
                    println!("Duplicate Key!!! {:?}", previous);
                };
            }
            rv
        })
    }
}

fn process_data_a(data: &str) -> usize {
    let rules = parser().parse(data).unwrap().1;
    let mut total_ore = 0;
    let mut requests = VecDeque::new();
    let mut leftovers = HashMap::new();
    for element in rules.keys() {
        leftovers.insert(*element, 0);
    }
    requests.push_back(Element {
        quantity: 1,
        symbol: "FUEL",
    });
    while let Some(mut target) = requests.pop_front() {
        if target.symbol == "ORE" {
            total_ore += target.quantity;
            continue;
        }
        let has = leftovers[target.symbol];
        if target.quantity <= has {
            // We already have enough, so take that amount.
            leftovers.insert(target.symbol, has - target.quantity);
            continue;
        } else {
            target.quantity -= has;
            leftovers.insert(target.symbol, 0);
        }
        let mut multiplier = 1;
        while (multiplier * rules[target.symbol].0) < target.quantity {
            multiplier += 1;
        }
        // Put the rest in the leftovers…
        leftovers.insert(
            target.symbol,
            multiplier * rules[target.symbol].0 - target.quantity,
        );
        for rule in &rules[target.symbol].1 {
            let mut multiplied_rule = *rule;
            multiplied_rule.quantity *= multiplier;
            requests.push_back(multiplied_rule);
        }
    }
    total_ore
}

fn munge_data_b(data: &str, base_size: usize) -> usize {
    let rules = parser().parse(data).unwrap().1;
    let mut total_fuel = 0;
    let mut total_ore: i128 = 1_000_000_000_000;
    let mut requests = VecDeque::new();
    let mut leftovers = HashMap::new();
    for element in rules.keys() {
        leftovers.insert(*element, 0);
    }
    requests.push_back(Element {
        quantity: total_ore as usize / base_size,
        symbol: "FUEL",
    });
    while total_ore as usize / base_size > 0 {
        let mut target = requests.pop_front().unwrap_or(Element {
            quantity: total_ore as usize / base_size,
            symbol: "FUEL",
        });
        if target.symbol == "ORE" {
            total_ore -= target.quantity as i128;
            continue;
        } else if target.symbol == "FUEL" {
            total_fuel += target.quantity;
            // println!("Getting another {} fuel.", target.quantity);
        }
        let has = leftovers[target.symbol];
        if target.quantity <= has {
            // We already have enough, so take that amount.
            leftovers.insert(target.symbol, has - target.quantity);
            continue;
        } else {
            target.quantity -= has;
            leftovers.insert(target.symbol, 0);
        }
        let mut multiplier = target.quantity / rules[target.symbol].0;
        while (multiplier * rules[target.symbol].0) < target.quantity {
            multiplier += 1;
        }
        // Put the rest in the leftovers…
        leftovers.insert(
            target.symbol,
            multiplier * rules[target.symbol].0 - target.quantity,
        );
        for rule in &rules[target.symbol].1 {
            let mut multiplied_rule = *rule;
            multiplied_rule.quantity *= multiplier;
            requests.push_back(multiplied_rule);
        }
    }
    total_fuel
}

fn process_data_b(data: &str) -> usize {
    munge_data_b(data, 136_771)
}

//-----------------------------------------------------
// Questions.

q_impl!("14");

#[test]
fn a() {
    assert_eq!(
        process_data_a(
            "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL
"
        ),
        31
    );
    assert_eq!(
        process_data_a(
            "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL
"
        ),
        165
    );
    assert_eq!(
        process_data_a(
            "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
"
        ),
        13_312
    );
    assert_eq!(
        process_data_a(
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF
"
        ),
        180_697
    );
    assert_eq!(
        process_data_a(
            "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX
"
        ),
        2_210_736
    );
}

#[test]
fn b() {
    assert_eq!(
        munge_data_b(
            "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
",
            13_312
        ),
        82_892_753
    );
    assert_eq!(
        munge_data_b(
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF
",
            180_697
        ),
        5_586_022
    );
    assert_eq!(
        munge_data_b(
            "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX
",
            2_210_736
        ),
        460_664
    );
}
