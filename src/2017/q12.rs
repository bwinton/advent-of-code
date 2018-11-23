//-----------------------------------------------------
// Setup.

use aoc::Day;

use regex::Regex;
use std::collections::HashSet;

static INPUT: &'static str = include_str!("data/q12.data");

fn parse_lines(data: &str) -> Vec<HashSet<u32>> {
  let mut rv: Vec<HashSet<u32>> = Vec::new();
  lazy_static! {
    static ref MAIN_RE: Regex = Regex::new(r"^(\d+) <-> (.*)$").unwrap();
  }
  for line in data.lines() {
    let cap = MAIN_RE.captures(line).unwrap();
    let mut dests: HashSet<u32> = cap[2].split(", ").map(|x| x.parse().unwrap()).collect();
    dests.insert(cap[1].parse().unwrap());
    for item in &mut rv {
      if !dests.is_disjoint(item) {
        dests.extend(item.iter());
        item.clear();
      }
    }
    rv.retain(|i| !i.is_empty());
    rv.push(dests);
  }
  rv
}

fn process_data_a(data: &str) -> usize {
  let groups = parse_lines(data);
  let rv = groups.iter().find(|group| group.contains(&0)).unwrap();
  rv.len()
}

fn process_data_b(data: &str) -> usize {
  let groups = parse_lines(data);
  groups.len()
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
  fn number(&self) -> String {
    String::from("12")
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    let result = process_data_a(INPUT);
    println!("Result = {}", result);
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    let result = process_data_b(INPUT);
    println!("Result = {}", result);
  }
}

#[test]
fn a() {
  assert_eq!(
    process_data_a(
      "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5",
    ),
    6
  );
  assert_eq!(
    process_data_a(
      "0 <-> 0
1 <-> 10
2 <-> 21
3 <-> 10, 21",
    ),
    1
  );
}

#[test]
fn b() {
  assert_eq!(
    process_data_b(
      "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5",
    ),
    2
  );
  assert_eq!(
    process_data_b(
      "0 <-> 0
1 <-> 10
2 <-> 21
3 <-> 10, 21",
    ),
    2
  );
}
