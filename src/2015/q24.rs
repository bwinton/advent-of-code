//-----------------------------------------------------
// Setup.

use aoc::Day;

use itertools::Itertools;
use std::collections::HashSet;
use std::u32;

static INPUT: &'static str = "1
2
3
7
11
13
17
19
23
31
37
41
43
47
53
59
61
67
71
73
79
83
89
97
101
103
107
109
113";
// static INPUT : &'static str = "1
// 2
// 3
// 4
// 5
// 7
// 8
// 9
// 10
// 11";

struct PowersetIter {
  items: Vec<usize>,
  len: usize,
  combinations: Vec<Vec<usize>>,
  curr: usize,
}

impl PowersetIter {
  fn new(items: &[usize]) -> Self {
    PowersetIter {
      items: items.to_vec(),
      len: 0,
      combinations: vec![],
      curr: 0,
    }
  }
}

impl Iterator for PowersetIter {
  type Item = Vec<usize>;
  fn next(&mut self) -> Option<Self::Item> {
    if self.curr >= self.combinations.len() {
      self.len += 1;
      self.curr = 0;
      self.combinations = self
        .items
        .clone()
        .into_iter()
        .combinations(self.len)
        .collect();
    }
    if self.len > self.items.len() {
      None
    } else {
      // println!("{:?}", self.combinations[self.curr]);
      let curr = self.curr;
      self.curr += 1;
      Some(self.combinations[curr].clone())
    }
  }
}

fn split(
  first: &[usize],
  all: &[usize],
  target: usize,
  groups: usize,
  seen: &mut HashSet<Vec<usize>>,
) -> Vec<Vec<Vec<usize>>> {
  if seen.contains(first) || first.iter().sum::<usize>() != target || groups == 0 {
    return vec![];
  }
  seen.insert(first.to_vec());
  if groups == 1 {
    return vec![vec![first.to_vec()]];
  }
  let rest: Vec<usize> = all
    .into_iter()
    .filter(|x| !first.contains(x))
    .cloned()
    .collect();
  // println!("{}Splitting {:?}/{:?} / {}:{}", "  ".repeat(4 - groups), first, rest, target, groups);
  let mut rv = vec![];
  let powerset = PowersetIter::new(&rest);
  for next in powerset {
    // println!("{}Next: {:?}", "  ".repeat(4 - groups), next);
    for mut group in split(&next, &rest, target, groups - 1, seen) {
      group.insert(0, first.to_vec());
      group.iter_mut().for_each(|x| x.sort());
      group.sort_unstable_by(|a, b| {
        a.len().cmp(&b.len()).then_with(|| {
          a.iter().product::<usize>().cmp(
            &b.iter().product::<usize>(),
          )
        })
      });
      // println!("{}Found {:?} {}", "  ".repeat(4 - groups), group, groups);
      rv.push(group);
    }
  }

  rv
}

fn process_data(data: &str, groups: usize) -> usize {
  let packages: Vec<usize> = data.lines().map(|i| i.parse().unwrap()).collect();
  let sum = packages.iter().sum::<usize>();
  let target = sum / groups;
  let mut rv = (u32::MAX as usize, u32::MAX as usize);
  let mut seen = HashSet::new();
  let powerset = PowersetIter::new(&packages);
  for first in powerset {
    for groups in split(&first, &packages, target, groups, &mut seen) {
      let curr = (groups[0].len(), groups[0].iter().product::<usize>());
      // println!("{:?} = {:?} < {:?}", groups, curr, rv);
      if curr < rv {
        rv = curr;
        return rv.1;
      }
    }
  }
  rv.1
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
  fn number(&self) -> String {
    String::from("24")
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    let result = process_data(INPUT, 3);
    println!("Result = {}", result);
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    let result = process_data(INPUT, 4);
    println!("Result = {}", result);
  }
}

#[test]
fn a() {
  assert_eq!(
    process_data(
      "1
2
3
4
5
7
8
9
10
11",
      3,
    ),
    99
  );
}

#[test]
fn b() {
  assert_eq!(
    process_data(
      "1
2
3
4
5
7
8
9
10
11",
      4,
    ),
    44
  );
}
