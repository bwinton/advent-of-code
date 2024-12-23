//-----------------------------------------------------
// Setup.

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

static INPUT: &str = include_str!("data/q23.data");

fn process_data_a(data: &str) -> usize {
    let mut groups = HashMap::new();
    let mut computers = HashSet::new();
    for line in data.lines() {
        let (a, b) = line.split_once('-').unwrap();
        computers.insert(a);
        computers.insert(b);
        groups.entry(a).or_insert_with(HashSet::new).insert(b);
        groups.entry(b).or_insert_with(HashSet::new).insert(a);
    }
    let mut threes = HashSet::new();
    for computer in computers {
        let connections = &groups[computer];
        for (&a, &b) in connections.iter().tuple_combinations() {
            if groups[&a].contains(&b) {
                let mut three = [computer, a, b];
                three.sort();
                threes.insert(three);
            }
        }
    }
    threes
        .iter()
        .filter(|three| three.iter().any(|&c| c.starts_with('t')))
        .count()
}

fn process_data_b(data: &str) -> String {
    let mut groups = HashMap::new();
    let mut computers = HashSet::new();
    for line in data.lines() {
        let (a, b) = line.split_once('-').unwrap();
        computers.insert(a);
        computers.insert(b);
        groups.entry(a).or_insert_with(HashSet::new).insert(b);
        groups.entry(b).or_insert_with(HashSet::new).insert(a);
    }
    let mut max_group = HashSet::new();
    for computer in computers {
        let connections = &groups[computer];
        let mut connected = HashSet::new();
        connected.insert(computer);
        for &new in connections {
            if connected
                .iter()
                .all(|&existing| groups[&existing].contains(&new))
            {
                connected.insert(new);
            }
        }
        if connected.len() > max_group.len() {
            max_group = connected;
        }
    }
    max_group.iter().sorted().join(",")
}

//-----------------------------------------------------
// Questions.

q_impl!("23");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "
        kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn
    "
        )),
        7
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "
        kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn
        "
        )),
        "co,de,ka,ta"
    );
}
