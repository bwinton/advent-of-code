//-----------------------------------------------------
// Setup.

use aoc::Day;

use itertools::Itertools;
use permutohedron::Heap;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::usize;

static INPUT: &'static str = "Faerun to Tristram = 65
Faerun to Tambi = 129
Faerun to Norrath = 144
Faerun to Snowdin = 71
Faerun to Straylight = 137
Faerun to AlphaCentauri = 3
Faerun to Arbre = 149
Tristram to Tambi = 63
Tristram to Norrath = 4
Tristram to Snowdin = 105
Tristram to Straylight = 125
Tristram to AlphaCentauri = 55
Tristram to Arbre = 14
Tambi to Norrath = 68
Tambi to Snowdin = 52
Tambi to Straylight = 65
Tambi to AlphaCentauri = 22
Tambi to Arbre = 143
Norrath to Snowdin = 8
Norrath to Straylight = 23
Norrath to AlphaCentauri = 136
Norrath to Arbre = 115
Snowdin to Straylight = 101
Snowdin to AlphaCentauri = 84
Snowdin to Arbre = 96
Straylight to AlphaCentauri = 107
Straylight to Arbre = 14
AlphaCentauri to Arbre = 46";

fn parse(data: &str) -> (HashSet<&str>, HashMap<[&str; 2], usize>) {
  lazy_static! {
    static ref RE: Regex = Regex::new("^([A-Za-z]+) to ([A-Za-z]+) = ([0-9]+)$").unwrap();
  }
  let mut cities: HashSet<&str> = HashSet::new();
  let mut distances: HashMap<[&str; 2], usize> = HashMap::new();
  for line in data.lines() {
    let cap = RE.captures(line);
    match cap {
      None => println!("Unknown format: {}", line),
      Some(x) => {
        let mut key = [x.at(1).unwrap(), x.at(2).unwrap()];
        key.sort();
        for city in &key {
          cities.insert(city);
        }
        distances.insert(key, x.at(3).unwrap().parse().unwrap());
      },
    }
  }
  (cities, distances)
}

fn get_distance(perm: &[&str], distances: &HashMap<[&str; 2], usize>) -> usize {
  let mut rv = 0;
  let lookup = distances.clone();
  for pair in perm.iter().tuple_windows::<(_, _)>() {
    let mut key = [*pair.0, *pair.1];
    key.sort();
    // println!("  => {:?}", &key);
    // println!("  => {:?}, {:?}", key, lookup[&key]);
    rv += lookup[&key];
  }
  rv
}

fn process_data_a(data: &str) -> (Vec<&str>, usize) {
  let (city_set, distances) = parse(data);
  let mut cities: Vec<&str> = city_set.into_iter().collect();
  let heap = Heap::new(&mut cities);
  let mut min_distance = (Vec::new(), usize::MAX);
  for perm in heap {
    let dist = get_distance(&perm, &distances);
    if dist < min_distance.1 {
      min_distance = (perm, dist);
    }
  }
  min_distance
}

fn process_data_b(data: &str) -> (Vec<&str>, usize) {
  let (city_set, distances) = parse(data);
  let mut cities: Vec<&str> = city_set.into_iter().collect();
  let heap = Heap::new(&mut cities);
  let mut max_distance = (Vec::new(), 0);
  for perm in heap {
    // println!("{:?}", perm);
    let dist = get_distance(&perm, &distances);
    if dist > max_distance.1 {
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
    (vec!["London", "Dublin", "Belfast"], 605)
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
    (vec!["Belfast", "London", "Dublin"], 982)
  );
}
