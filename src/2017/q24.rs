//-----------------------------------------------------
// Setup.

use aoc::Day;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::str::FromStr;

static INPUT : &'static str = "31/13
34/4
49/49
23/37
47/45
32/4
12/35
37/30
41/48
0/47
32/30
12/5
37/31
7/41
10/28
35/4
28/35
20/29
32/20
31/43
48/14
10/11
27/6
9/24
8/28
45/48
8/1
16/19
45/45
0/4
29/33
2/5
33/9
11/7
32/10
44/1
40/32
2/45
16/16
1/18
38/36
34/24
39/44
32/37
26/46
25/33
9/10
0/29
38/8
33/33
49/19
18/20
49/39
18/39
26/13
19/32";

#[derive(Clone)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
struct Pipe {
  input: usize,
  output: usize,
  strength: usize
}

impl FromStr for Pipe {
  type Err = ();

  fn from_str(s: &str) -> Result<Pipe, ()> {
    let mut values: Vec<usize> = s.split('/').map(|i| i.parse().unwrap()).collect();
    values.sort();
    Ok(Pipe{
      input: values[0],
      output: values[1],
      strength: values[0] + values[1]
    })
  }
}

impl Pipe {
  fn next(&self, expecting: usize) -> Option<usize> {
    match expecting {
      x if x == self.input => Some(self.output),
      x if x == self.output => Some(self.input),
      _ => None
    }
  }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
struct Bridge {
  pipes: Vec<Pipe>,
  remaining: Vec<Pipe>,
  strength: usize,
  expecting: usize
}

impl Ord for Bridge {
  fn cmp(&self, other: &Bridge) -> Ordering {
    let move_cmp = (self.strength).cmp(&other.strength);
    if move_cmp == Ordering::Equal {
      return self.pipes.len().cmp(&other.pipes.len());
    }
    move_cmp
  }
}

impl Bridge {
  pub fn new(pipes: &[Pipe]) -> Bridge {
    Bridge {
      pipes: Vec::new(),
      remaining: pipes.to_owned(),
      strength: 0,
      expecting: 0
    }
  }

  fn push(&mut self, index: usize, pipe: &Pipe) -> bool {
    if let Some(next) = pipe.next(self.expecting) {
      self.pipes.push(pipe.clone());
      self.remaining.remove(index);
      self.expecting = next;
      self.strength += pipe.strength;
      true
    } else {
      false
    }
  }
}

fn process_data_a(data: &str) -> usize {
  let mut pipes: Vec<Pipe> = Vec::new();
  for line in data.lines() {
    pipes.push(line.parse().unwrap());
  }
  // println!("{:?}", pipes);

  let mut next = BinaryHeap::new();
  next.push(Bridge::new(&pipes));

  let mut rv = 0;
  // let mut count = 0;
  while !next.is_empty() {
    let current = next.pop().unwrap();
    if current.strength > rv {
      rv = current.strength;
    }
    for (index, pipe) in current.remaining.iter().enumerate() {
      let mut temp = current.clone();
      if temp.push(index, pipe) {
        next.push(temp);
      }
    }
    // count += 1;
    // if count % 10000 == 0 {
    //   println!("\n{} ({}): {:?} ({:?}) / {:?}", count, rv, current.pipes.len(), current.strength, next.len());
    // }
  }

  rv
}

fn process_data_b(data: &str) -> usize {
  let mut pipes: Vec<Pipe> = Vec::new();
  for line in data.lines() {
    pipes.push(line.parse().unwrap());
  }

  let mut rv = Bridge::new(&pipes);

  let mut next = BinaryHeap::new();
  next.push(rv.clone());

  // let mut count = 0;
  while !next.is_empty() {
    let current = next.pop().unwrap();
    if current.pipes.len() > rv.pipes.len() ||
      (current.pipes.len() == rv.pipes.len() &&
       current.strength > rv.strength) {
      rv = current.clone();
    }
    for (index, pipe) in current.remaining.iter().enumerate() {
      let mut temp = current.clone();
      if temp.push(index, pipe) {
        next.push(temp);
      }
    }
    // count += 1;
    // if count % 10000 == 0 {
    //   println!("\n{} ({}): {:?} ({:?}) / {:?}", count, rv, current.pipes.len(), current.strength, next.len());
    // }
  }

  rv.strength
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
  assert_eq!(process_data_a("0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10"), 31);
}

#[test]
fn b() {
  assert_eq!(process_data_b("0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10"), 19);
}
