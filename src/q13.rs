//-----------------------------------------------------
// Setup.

use day;

use itertools::Itertools;
use permutohedron::Heap;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::i32;

static INPUT : &'static str = "Alice would lose 2 happiness units by sitting next to Bob.
Alice would lose 62 happiness units by sitting next to Carol.
Alice would gain 65 happiness units by sitting next to David.
Alice would gain 21 happiness units by sitting next to Eric.
Alice would lose 81 happiness units by sitting next to Frank.
Alice would lose 4 happiness units by sitting next to George.
Alice would lose 80 happiness units by sitting next to Mallory.
Bob would gain 93 happiness units by sitting next to Alice.
Bob would gain 19 happiness units by sitting next to Carol.
Bob would gain 5 happiness units by sitting next to David.
Bob would gain 49 happiness units by sitting next to Eric.
Bob would gain 68 happiness units by sitting next to Frank.
Bob would gain 23 happiness units by sitting next to George.
Bob would gain 29 happiness units by sitting next to Mallory.
Carol would lose 54 happiness units by sitting next to Alice.
Carol would lose 70 happiness units by sitting next to Bob.
Carol would lose 37 happiness units by sitting next to David.
Carol would lose 46 happiness units by sitting next to Eric.
Carol would gain 33 happiness units by sitting next to Frank.
Carol would lose 35 happiness units by sitting next to George.
Carol would gain 10 happiness units by sitting next to Mallory.
David would gain 43 happiness units by sitting next to Alice.
David would lose 96 happiness units by sitting next to Bob.
David would lose 53 happiness units by sitting next to Carol.
David would lose 30 happiness units by sitting next to Eric.
David would lose 12 happiness units by sitting next to Frank.
David would gain 75 happiness units by sitting next to George.
David would lose 20 happiness units by sitting next to Mallory.
Eric would gain 8 happiness units by sitting next to Alice.
Eric would lose 89 happiness units by sitting next to Bob.
Eric would lose 69 happiness units by sitting next to Carol.
Eric would lose 34 happiness units by sitting next to David.
Eric would gain 95 happiness units by sitting next to Frank.
Eric would gain 34 happiness units by sitting next to George.
Eric would lose 99 happiness units by sitting next to Mallory.
Frank would lose 97 happiness units by sitting next to Alice.
Frank would gain 6 happiness units by sitting next to Bob.
Frank would lose 9 happiness units by sitting next to Carol.
Frank would gain 56 happiness units by sitting next to David.
Frank would lose 17 happiness units by sitting next to Eric.
Frank would gain 18 happiness units by sitting next to George.
Frank would lose 56 happiness units by sitting next to Mallory.
George would gain 45 happiness units by sitting next to Alice.
George would gain 76 happiness units by sitting next to Bob.
George would gain 63 happiness units by sitting next to Carol.
George would gain 54 happiness units by sitting next to David.
George would gain 54 happiness units by sitting next to Eric.
George would gain 30 happiness units by sitting next to Frank.
George would gain 7 happiness units by sitting next to Mallory.
Mallory would gain 31 happiness units by sitting next to Alice.
Mallory would lose 32 happiness units by sitting next to Bob.
Mallory would gain 95 happiness units by sitting next to Carol.
Mallory would gain 91 happiness units by sitting next to David.
Mallory would lose 66 happiness units by sitting next to Eric.
Mallory would lose 75 happiness units by sitting next to Frank.
Mallory would lose 99 happiness units by sitting next to George.";

fn parse(data: &str) -> (HashSet<String>, HashMap<Vec<String>, i32>) {
  lazy_static! {
    static ref SEATING: Regex = Regex::new(r"^([A-Za-z]+) would (gain|lose) (\d+) happiness units by sitting next to ([A-Za-z]+).$").unwrap();
  }
  let mut happiness = HashMap::new();
  let mut people = HashSet::new();

  for line in data.lines() {
    if let Some(cap) = SEATING.captures(line) {
      let mut key = vec![cap[1].to_string(), cap[4].to_string()];
      key.sort();
      for name in key.clone() {
        people.insert(name);
      }
      let mut value: i32 = cap[3].parse().unwrap();
      if let "lose" = &cap[2] {
        value = -value;
      }
      let mut entry = happiness.entry(key).or_insert(0);
      *entry += value;
    }
  }
  (people, happiness)
}

fn get_happiness(perm: &[String], first: Option<&String>, distances: &HashMap<Vec<String>, i32>) -> i32 {
  let mut rv = 0;
  let lookup = distances.clone();
  let mut people: Vec<String> = perm.into_iter().map(|x| x.to_string()).collect();
  if let Some(person) = first {
    people.insert(0, person.clone());
    people.push(person.clone());
  }
  for pair in people.iter().tuple_windows::<(_, _)>() {
    let mut key = vec![pair.0.clone(), pair.1.clone()];
    key.sort();
    // println!("  => {:?}", &key);
    // println!("  => {:?}, {:?}", key, lookup[&key]);
    rv += lookup[&key];
  }
  // println!("{:?} = {}", people, rv);
  rv
}

fn process_data_a(data: &str) -> (Vec<String>, i32) {
  let (names, happiness) = parse(data);
  let mut name_iter = names.into_iter();
  let first = name_iter.next().unwrap();
  // println!("{:?}", first);
  let mut people: Vec<String> = name_iter.collect();
  let heap = Heap::new(&mut people);
  let mut max_happiness = (Vec::new(), i32::MIN);
  for perm in heap {
    let value = get_happiness(&perm, Some(&first), &happiness);
    if value > max_happiness.1 {
      max_happiness = (perm, value);
    }
  }
  max_happiness
}

fn process_data_b(data: &str) -> (Vec<String>, i32) {
  let (names, happiness) = parse(data);
  let mut people: Vec<String> = names.into_iter().collect();
  let heap = Heap::new(&mut people);
  let mut max_happiness = (Vec::new(), i32::MIN);
  for perm in heap {
    let value = get_happiness(&perm, None, &happiness);
    if value > max_happiness.1 {
      max_happiness = (perm, value);
    }
  }
  max_happiness
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl day::Day for Q {
  fn number(&self) -> String {
    String::from("13")
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
  assert_eq!(process_data_a("Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.").1, 331);
}

#[test]
fn b() {
}
