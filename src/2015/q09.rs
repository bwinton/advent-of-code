//-----------------------------------------------------
// Setup.

use itertools::Itertools;
use permutohedron::Heap;
use regex::Regex;
use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("data/q09.data");

fn parse(data: &str) -> (HashSet<String>, HashMap<[String; 2], usize>) {
    let re: &Regex = regex!("^([A-Za-z]+) to ([A-Za-z]+) = ([0-9]+)$");
    let mut cities: HashSet<String> = HashSet::new();
    let mut distances: HashMap<[String; 2], usize> = HashMap::new();
    for line in data.lines() {
        let cap = re.captures(line);
        match cap {
            None => println!("Unknown format: {}", line),
            Some(x) => {
                let mut key = [x[1].to_string(), x[2].to_string()];
                key.sort();
                for city in &key {
                    cities.insert(city.to_string());
                }
                distances.insert(key, x[3].parse().unwrap());
            }
        }
    }
    (cities, distances)
}

fn get_distance(perm: &[String], distances: &HashMap<[String; 2], usize>) -> usize {
    let mut rv = 0;
    let lookup = distances.clone();
    for pair in perm.iter().tuple_windows::<(_, _)>() {
        let mut key = [pair.0.clone(), pair.1.clone()];
        key.sort();
        // println!("  => {:?}", &key);
        // println!("  => {:?}, {:?}", key, lookup[&key]);
        rv += lookup[&key];
    }
    rv
}

fn process_data_a_impl(data: &str) -> (Vec<String>, usize) {
    let (city_set, distances) = parse(data);
    let mut cities: Vec<String> = city_set.into_iter().collect();
    let heap = Heap::new(&mut cities);
    let mut min_distance = (Vec::new(), usize::MAX);
    for perm in heap {
        let dist = get_distance(&perm, &distances);
        if (dist < min_distance.1) || (dist == min_distance.1 && perm < min_distance.0) {
            min_distance = (perm, dist);
        }
    }
    min_distance
}

fn process_data_b_impl(data: &str) -> (Vec<String>, usize) {
    let (city_set, distances) = parse(data);
    let mut cities: Vec<String> = city_set.into_iter().collect();
    let heap = Heap::new(&mut cities);
    let mut max_distance = (Vec::new(), 0);
    for perm in heap {
        let dist = get_distance(&perm, &distances);
        if (dist > max_distance.1) || (dist == max_distance.1 && perm < max_distance.0) {
            max_distance = (perm, dist);
        }
    }
    max_distance
}

fn process_data_a(data: &str) -> usize {
    process_data_a_impl(data).1
}

fn process_data_b(data: &str) -> usize {
    process_data_b_impl(data).1
}

//-----------------------------------------------------
// Questions.

q_impl!("9");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a_impl(
            "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141",
        ),
        (
            vec![
                "Belfast".to_string(),
                "Dublin".to_string(),
                "London".to_string()
            ],
            605
        )
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b_impl(
            "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141",
        ),
        (
            vec![
                "Belfast".to_string(),
                "London".to_string(),
                "Dublin".to_string()
            ],
            982
        )
    );
}
