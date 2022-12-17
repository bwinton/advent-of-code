//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q16.data");

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: u32,
    tunnels: Vec<String>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct StateA {
    max_remaining: u32,
    released_pressure: u32,
    minute: u32,
    valve: String,
    opened: Vec<String>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct StateB {
    released_pressure: u32,
    minute: u32,
    max_remaining: u32,
    valves: [String; 2],
    opened: Vec<String>,
}

fn valve(i: &str) -> IResult<&str, Valve> {
    let (input, (name, _, flow_rate, _, tunnels)) = preceded(
        tag("Valve "),
        tuple((
            alpha1,
            tag(" has flow rate="),
            complete::u32,
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "),
            )),
            separated_list1(tag(", "), alpha1),
        )),
    )(i)?;
    Ok((
        input,
        Valve {
            name: name.to_owned(),
            flow_rate,
            tunnels: tunnels.iter().map(|&s| s.to_owned()).collect(),
        },
    ))
}

fn parser(i: &str) -> IResult<&str, Vec<Valve>> {
    let (input, list) = separated_list1(line_ending, valve)(i)?;
    Ok((input, list))
}

fn get_distance(
    name: &String,
    valves: &HashMap<String, Valve>,
    flow_rates: &HashMap<String, u32>,
) -> HashMap<String, u32> {
    let mut distances = HashMap::new();
    distances.insert(name.clone(), 0);
    let mut stack = VecDeque::new();
    stack.push_back(name.clone());
    while !stack.is_empty() {
        let valve = stack.pop_front().unwrap();
        let d = distances[&valve];
        for tunnel in &valves[&valve].tunnels {
            let distance = distances.get(tunnel).copied().unwrap_or(u32::MAX);
            if d + 1 < distance {
                distances.insert(tunnel.clone(), d + 1);
                stack.push_back(tunnel.clone());
            }
        }
    }
    distances
        .into_iter()
        .filter_map(|(k, v)| {
            if flow_rates[&k] > 0 && &k != name {
                Some((k, v + 1))
            } else {
                None
            }
        })
        .collect()
}

fn path_value(
    visited: &[String],
    total_time: u32,
    distances: &HashMap<String, HashMap<String, u32>>,
    flow_rates: &HashMap<String, u32>,
) -> u32 {
    let mut rv = 0;
    let mut time = total_time;
    for i in 0..visited.len() - 1 {
        time -= distances[&visited[i]][&visited[i + 1]];
        rv += time * flow_rates[&visited[i + 1]];
    }
    rv
}

fn explore(
    current: &str,
    visited: Vec<String>,
    value: u32,
    remaining_time: i32,
    total_time: u32,
    distances: &HashMap<String, HashMap<String, u32>>,
    flow_rates: &HashMap<String, u32>,
    cache: &mut HashMap<(Vec<String>, String), u32>,
) -> u32 {
    let mut key = (visited.clone(), current.to_owned());
    key.0.sort();

    if let Some(&cache_hit) = cache.get(&key) {
        if cache_hit < value {
            cache.insert(key, value);
        } else {
            return cache_hit;
        }
    } else {
        cache.insert(key, value);
    }

    let mut subpath_values = vec![value];

    for (valve, distance) in distances[current]
        .iter()
        .filter(|&(valve, distance)| {
            !visited.contains(valve) && remaining_time - (*distance as i32) >= 0
        })
        .sorted_by(|&a, &b| match a.1.cmp(b.1) {
            Ordering::Equal => a.0.cmp(b.0),
            x => x,
        })
    {
        let subpath_remaining = remaining_time - (*distance as i32);

        let mut visited = visited.clone();
        visited.push(valve.clone());
        let sub_value = path_value(&visited, total_time, distances, flow_rates);

        let subpath = explore(
            valve,
            visited,
            sub_value,
            subpath_remaining,
            total_time,
            distances,
            flow_rates,
            cache,
        );
        subpath_values.push(subpath);
    }

    // return max(subpath_values)
    subpath_values.into_iter().max().unwrap()
}

fn process_data_a(data: &str) -> u32 {
    let valves = parser(data).unwrap().1;
    let valves: HashMap<String, Valve> = valves.into_iter().map(|v| (v.name.clone(), v)).collect();
    let flow_rates: HashMap<String, u32> = valves
        .iter()
        .map(|(k, v)| (k.clone(), v.flow_rate))
        .collect();

    // Figure out the distances to valves with flow_rates > 0.
    let mut distances: HashMap<String, HashMap<String, u32>> = valves
        .iter()
        .filter_map(|(name, valve)| {
            if valve.flow_rate == 0 {
                return None;
            }
            Some((name.clone(), get_distance(name, &valves, &flow_rates)))
        })
        .collect();
    distances.insert(
        "AA".to_string(),
        get_distance(&"AA".to_owned(), &valves, &flow_rates),
    );

    let mut cache = HashMap::new();
    explore(
        "AA",
        vec!["AA".to_owned()],
        0,
        30,
        30,
        &distances,
        &flow_rates,
        &mut cache,
    )

    // 1986!
}

fn best_pair_for(
    visited: &[String],
    optimal_path_values: &HashMap<Vec<String>, u32>,
) -> Option<(Vec<String>, Vec<String>)> {
    let v_set: HashSet<String> = HashSet::from_iter(visited.to_owned());
    let options = optimal_path_values
        .iter()
        .filter(|&(key, _)| {
            let key = HashSet::from_iter(key.clone());
            v_set.intersection(&key).count() == 0
        })
        .max_by_key(|&(_, v)| *v);
    options.map(|(pair, _)| (visited.to_owned(), pair.clone()))
}

fn process_data_b(data: &str) -> u32 {
    let valves = parser(data).unwrap().1;
    let valves: HashMap<String, Valve> = valves.into_iter().map(|v| (v.name.clone(), v)).collect();
    let flow_rates: HashMap<String, u32> = valves
        .iter()
        .map(|(k, v)| (k.clone(), v.flow_rate))
        .collect();

    let mut distances: HashMap<String, HashMap<String, u32>> = valves
        .iter()
        .filter_map(|(name, valve)| {
            if valve.flow_rate == 0 {
                return None;
            }
            Some((name.clone(), get_distance(name, &valves, &flow_rates)))
        })
        .collect();
    distances.insert(
        "AA".to_string(),
        get_distance(&"AA".to_owned(), &valves, &flow_rates),
    );

    // Populate the cache.
    let mut cache = HashMap::new();
    explore(
        "AA",
        vec!["AA".to_owned()],
        0,
        26,
        26,
        &distances,
        &flow_rates,
        &mut cache,
    );

    let mut optimal_path_values = HashMap::new();
    for (key, value) in cache {
        let key: Vec<_> = key.0.iter().skip(1).cloned().collect();

        if value != 0 {
            if let Some(&cache_hit) = optimal_path_values.get(&key) {
                if cache_hit < value {
                    optimal_path_values.insert(key, value);
                }
            } else {
                optimal_path_values.insert(key, value);
            }
        }
    }

    let optimal_pairs: HashMap<Vec<String>, Vec<String>> = optimal_path_values
        .keys()
        .filter_map(|path| best_pair_for(path, &optimal_path_values))
        .collect();

    optimal_pairs
        .iter()
        .map(|(p1, p2)| optimal_path_values[p1] + optimal_path_values[p2])
        .max()
        .unwrap()
}

//-----------------------------------------------------
// Questions.

q_impl!("16");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    Valve BB has flow rate=13; tunnels lead to valves CC, AA
    Valve CC has flow rate=2; tunnels lead to valves DD, BB
    Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    Valve EE has flow rate=3; tunnels lead to valves FF, DD
    Valve FF has flow rate=0; tunnels lead to valves EE, GG
    Valve GG has flow rate=0; tunnels lead to valves FF, HH
    Valve HH has flow rate=22; tunnel leads to valve GG
    Valve II has flow rate=0; tunnels lead to valves AA, JJ
    Valve JJ has flow rate=21; tunnel leads to valve II
    "
        )),
        1651
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    Valve BB has flow rate=13; tunnels lead to valves CC, AA
    Valve CC has flow rate=2; tunnels lead to valves DD, BB
    Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    Valve EE has flow rate=3; tunnels lead to valves FF, DD
    Valve FF has flow rate=0; tunnels lead to valves EE, GG
    Valve GG has flow rate=0; tunnels lead to valves FF, HH
    Valve HH has flow rate=22; tunnel leads to valve GG
    Valve II has flow rate=0; tunnels lead to valves AA, JJ
    Valve JJ has flow rate=21; tunnel leads to valve II
    "
        )),
        1707
    );
}
