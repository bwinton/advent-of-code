//-----------------------------------------------------
// Setup.

use aoc::Day;
use std::str::FromStr;

// static INPUT : &'static str = "..^^.";
// static LENGTH_A : usize = 3;
// static INPUT : &'static str = ".^^.^.^^^^";
// static LENGTH_A : usize = 10;
static INPUT: &'static str = ".^.^..^......^^^^^...^^^...^...^....^^.^...^.^^^^....^...^^.^^^...^^^^.^^.^.^^..^.^^^..^^^^^^.^^^..^";
static LENGTH_A: usize = 40;
static LENGTH_B: usize = 400_000;

#[derive(Clone)]
#[derive(Debug)]
struct Row {
  data: String,
  safe_count: usize,
}

impl FromStr for Row {
  type Err = ();

  fn from_str(s: &str) -> Result<Row, ()> {
    let mut safe_count = 0;
    for c in s.chars() {
      if c == '.' {
        safe_count += 1;
      }
    }
    Ok(Row {
      data: s.to_string(),
      safe_count,
    })
  }
}

fn is_a_trap(cells: &str) -> char {
  if cells == "^^." || cells == ".^^" || cells == "^.." || cells == "..^" {
    '^'
  } else {
    '.'
  }
}

fn get_next_row(row: &Row) -> Row {
  let mut safe_count = 0;
  let mut data: Vec<char> = Vec::new();
  let mut temp = vec!['.'];
  temp.extend(&mut row.data.chars());
  temp.push('.');
  let previous: String = temp.into_iter().collect();
  for i in 0..row.data.len() {
    let curr = is_a_trap(&previous[i..i + 3]);
    if curr == '.' {
      safe_count += 1;
    }
    data.push(curr);
  }

  Row {
    data: data.into_iter().collect(),
    safe_count,
  }
}

fn get_result(length: usize) -> usize {
  let mut result = 0;
  let mut rows = Vec::new();
  let mut row: Row = INPUT.parse().unwrap();
  rows.push(row.clone());
  for _ in 0..length - 1 {
    row = get_next_row(&row);
    rows.push(row.clone());
  }
  for row in rows {
    result += row.safe_count;
  }
  result
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
  fn number(&self) -> String {
    String::from("18")
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    println!("Result = {}", get_result(LENGTH_A));
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    println!("Result = {}", get_result(LENGTH_B));
  }
}
