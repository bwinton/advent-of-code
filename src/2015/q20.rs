//-----------------------------------------------------
// Setup.

use aoc::Day;

use std;
use std::collections::HashSet;

static INPUT: usize = 33_100_000;

define_iterator!(FactorsIter (
    // &seen: Vec<HashSet<usize>> = Vec::new(),
    &curr: usize = 1
  ) -> Option<HashSet<usize>> {

  let mut factors = HashSet::new();
  let upper_limit = (*curr as f64).sqrt() as usize + 1;
  for i in 1..upper_limit {
    if *curr % i == 0 {
      factors.insert(i);
      factors.insert(*curr / i);
    }
  }
  *curr += 1;
  Some(factors)
});


fn process_data_a(data: usize) -> usize {
  for (i, factors) in FactorsIter::default().enumerate() {
    let value: usize = factors.iter().sum();
    if value as usize * 10 >= data {
      return i + 1;
    }
  }
  0
}

fn process_data_b(data: usize) -> usize {
  for (i, factors) in FactorsIter::default().enumerate() {
    let house = i + 1;
    let value: usize = factors.iter().filter(|&elf| house <= *elf * 51).sum();
    if value as usize * 11 >= data {
      return house;
    }
  }
  0
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
  fn number(&self) -> String {
    String::from("20")
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
  let mut iter = FactorsIter::default();
  assert_eq!(iter.next().unwrap(), hashset![1]);
  assert_eq!(iter.next().unwrap(), hashset![1, 2]);
  assert_eq!(iter.next().unwrap(), hashset![1, 3]);
  assert_eq!(iter.next().unwrap(), hashset![1, 2, 4]);
  assert_eq!(iter.next().unwrap(), hashset![1, 5]);
  assert_eq!(iter.next().unwrap(), hashset![1, 2, 3, 6]);
  assert_eq!(iter.next().unwrap(), hashset![1, 7]);
  assert_eq!(iter.next().unwrap(), hashset![1, 2, 4, 8]);
  assert_eq!(iter.next().unwrap(), hashset![1, 3, 9]);
}

#[test]
fn b() {
  // assert_eq!(process_data_b(""), 0);
}
