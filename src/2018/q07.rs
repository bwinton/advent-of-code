//-----------------------------------------------------
// Setup.

use regex::Regex;
use std::char;
use std::collections::HashMap;
use std::collections::HashSet;

static INPUT: &'static str = include_str!("data/q07.data");

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();
}

fn process_data_a(data: &str) -> String {
    let mut dep_graph = HashMap::new();
    let mut nodes = HashSet::new();
    for line in data.lines() {
        if let Some(cap) = RE.captures(line) {
            nodes.insert(cap[1].chars().next().unwrap());
            nodes.insert(cap[2].chars().next().unwrap());
            let mut dependencies = dep_graph
                .entry(cap[2].chars().next().unwrap())
                .or_insert_with(|| vec![]);
            dependencies.push(cap[1].chars().next().unwrap());
            dependencies.sort();
        }
    }
    let mut rv = vec![];

    while !nodes.is_empty() {
        let mut current = char::MAX;
        for node in &nodes {
            if !dep_graph.contains_key(&node.clone()) && current > *node {
                current = *node;
            }
        }
        for value in dep_graph.values_mut() {
            value.retain(|x| x != &current);
        }
        dep_graph.retain(|_, v| !v.is_empty());
        nodes.remove(&current);
        rv.push(current);
    }
    rv.iter().collect()
}

fn get_timing(data: &str, workers: usize, delay: i32) -> i32 {
    let mut dep_graph = HashMap::new();
    let mut nodes = HashSet::new();
    for line in data.lines() {
        if let Some(cap) = RE.captures(line) {
            nodes.insert(cap[1].chars().next().unwrap());
            nodes.insert(cap[2].chars().next().unwrap());
            let mut dependencies = dep_graph
                .entry(cap[2].chars().next().unwrap())
                .or_insert_with(|| vec![]);
            dependencies.push(cap[1].chars().next().unwrap());
            dependencies.sort();
        }
    }
    let mut rv = -1;
    let mut workers = vec![(char::MAX, -1)].repeat(workers);

    while !nodes.is_empty() {
        // step the time, and dec the workers.
        rv += 1;
        for worker in &mut workers {
            worker.1 -= 1;
        }
        if workers.iter().all(|worker| worker.1 > 0) {
            continue;
        }

        for worker in workers
            .iter()
            .filter(|(current, time)| *time <= 0 && current != &char::MAX)
        {
            for value in dep_graph.values_mut() {
                value.retain(|x| x != &worker.0);
            }
            dep_graph.retain(|_, v| !v.is_empty());
            nodes.remove(&worker.0);
        }

        for (_i, worker) in workers.iter_mut().enumerate() {
            if worker.1 <= 0 {
                let mut current = char::MAX;
                for node in &nodes {
                    if !dep_graph.contains_key(&node.clone()) && current > *node {
                        current = *node;
                    }
                }
                if current != char::MAX {
                    worker.0 = current;
                    worker.1 = delay + (current as i32) - ('A' as i32) + 1;
                    nodes.remove(&worker.0);
                }
            }
        }
    }
    rv + workers.iter().map(|(_, time)| time).max().unwrap()
}

fn process_data_b(data: &str) -> i32 {
    // 1130 is too high…
    get_timing(data, 5, 60)
}

//-----------------------------------------------------
// Questions.

q_impl!("7");

#[test]
fn a() {
    assert_eq!(
        process_data_a(
            "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."
        ),
        "CABDFE"
    );
}

#[test]
fn b() {
    assert_eq!(
        get_timing(
            "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.",
            2,
            0
        ),
        15
    );
}
