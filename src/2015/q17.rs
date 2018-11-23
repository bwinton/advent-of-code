//-----------------------------------------------------
// Setup.

use aoc::Day;

use itertools::Itertools;

static INPUT: &'static str = include_str!("data/q17.data");

fn process_data_a(data: &str, amount: u32) -> usize {
  let containers: Vec<u32> = data.lines().map(|x| x.parse().unwrap()).collect();
  let mut count = 0;
  for len in 1..=containers.len() {
    for permutation in containers.iter().combinations(len) {
      // print!("{:?} = ", &permutation);
      let sum: u32 = permutation.into_iter().sum();
      // println!("{}", sum);
      if sum == amount {
        count += 1;
      }
    }
  }
  count
}

fn process_data_b(data: &str, amount: u32) -> usize {
  let containers: Vec<u32> = data.lines().map(|x| x.parse().unwrap()).collect();
  let mut smallest: Vec<Vec<_>> = Vec::new();
  for len in 1..=containers.len() {
    for permutation in containers.iter().combinations(len) {
      let sum: u32 = permutation.iter().cloned().sum();
      if sum == amount {
        if !smallest.is_empty() && permutation.len() < smallest[0].len() {
          smallest.clear();
        }
        if smallest.is_empty() || permutation.len() == smallest[0].len() {
          smallest.push(permutation.clone());
        }
      }
    }
  }
  smallest.len()
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
  fn number(&self) -> String {
    String::from("17")
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    let result = process_data_a(INPUT, 150);
    println!("Result = {}", result);
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    let result = process_data_b(INPUT, 150);
    println!("Result = {}", result);
  }
}

#[test]
fn a() {
  assert_eq!(
    process_data_a(
      "20
15
10
5
5",
      25,
    ),
    4
  );
}

#[test]
fn b() {
  assert_eq!(
    process_data_b(
      "20
15
10
5
5",
      25,
    ),
    3
  );
}
