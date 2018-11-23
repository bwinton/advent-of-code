//-----------------------------------------------------
// Setup.

use aoc::Day;

use itertools::Itertools;
use permutohedron::Heap;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::usize;

static INPUT: &'static str = include_str!("data/q09.data");

fn parse(data: &str) -> (HashSet<String>, HashMap<[String; 2], usize>) {
  lazy_static! {
    static ref RE: Regex = Regex::new("^([A-Za-z]+) to ([A-Za-z]+) = ([0-9]+)$").unwrap();
  }
  let mut cities: HashSet<String> = HashSet::new();
  let mut distances: HashMap<[String; 2], usize> = HashMap::new();
  for line in data.lines() {
    let cap = RE.captures(line);
    match cap {
      None => println!("Unknown format: {}", line),
      Some(x) => {
        let mut key = [x[1].to_string(), x[2].to_string()];
        key.sort();
        for city in &key {
          cities.insert(city.to_string());
        }
        distances.insert(key, x[3].parse().unwrap());
      },
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

fn process_data_a(data: &str) -> (Vec<String>, usize) {
  let (city_set, distances) = parse(data);
  let mut cities: Vec<String> = city_set.into_iter().collect();
  let heap = Heap::new(&mut cities);
  let mut min_distance = (Vec::new(), usize::MAX);
  for perm in heap {
    let dist = get_distance(&perm, &distances);
    if (dist < min_distance.1) || (dist == min_distance.1 && perm < min_distance.0){
      min_distance = (perm, dist);
    }
  }
  min_distance
}

fn process_data_b(data: &str) -> (Vec<String>, usize) {
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

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
  fn number(&self) -> String {
    String::from("9")
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    let result = process_data_a(INPUT);
    println!("Result = {}", result.1);
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    let result = process_data_b(INPUT);
    println!("Result = {}", result.1);
  }
}

#[test]
fn a() {
  assert_eq!(
    process_data_a(
      "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141",
    ),
    (vec!["Belfast".to_string(), "Dublin".to_string(), "London".to_string()], 605)
  );
}

#[test]
fn b() {
  assert_eq!(
    process_data_b(
      "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141",
    ),
    (vec!["Belfast".to_string(), "London".to_string(), "Dublin".to_string()], 982)
  );
}
