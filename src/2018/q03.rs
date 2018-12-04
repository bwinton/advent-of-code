//-----------------------------------------------------
// Setup.

use regex::Regex;
use std::collections::HashSet;
use std::cmp::{max, min};
use std::str::FromStr;

static INPUT : &'static str = include_str!("data/q03.data");

#[derive(Clone, Debug)]
struct Square {
    id: String,
    top: u32,
    left: u32,
    bottom: u32,
    right: u32,
}

impl Square {
  fn contains(self, x: u32, y: u32) -> bool {
    x >= self.left && x < self.right && y >= self.top && y < self.bottom
  }
}

impl FromStr for Square {
    type Err = ();

    fn from_str(s: &str) -> Result<Square, ()> {
      // #1 @ 1,3: 4x4
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(#\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        }
        let captures = RE.captures(s);

        match captures {
            Some(cap) => {
              let id = cap[1].to_owned();
              let left: u32 = cap[2].parse().unwrap();
              let top: u32 = cap[3].parse().unwrap();
              let width: u32 = cap[4].parse().unwrap();
              let height: u32 = cap[5].parse().unwrap();

              Ok(Square { id, top, left,
                bottom: top + height,
                right: left + width })
            },
            None => Err(()),
        }
    }
}


fn process_data_a(data: &str) -> u32 {
  let mut squares = vec![];
  for line in data.lines() {
    let square: Square = line.parse().unwrap();
    squares.push(square);
  }
  let mut min_x = squares[0].left;
  let mut min_y = squares[0].top;
  let mut max_x = squares[0].right;
  let mut max_y = squares[0].bottom;

  for square in &squares {
    min_x = min(min_x, square.left);
    min_y = min(min_y, square.top);
    max_x = max(max_x, square.right);
    max_y = max(max_y, square.bottom);
  }

  let mut common_squares = 0;

  for x in min_x..=max_x+1 {
    for y in min_y..=max_y+1 {
      let mut found = false;
      for square in squares.clone() {
        if square.contains(x, y) {
          if found {
            common_squares += 1;
            break;
          }
          found = true;
        }
      }
    }
  }
  common_squares
}

fn process_data_b(data: &str) -> String {
  let mut squares = vec![];
  for line in data.lines() {
    let square: Square = line.parse().unwrap();
    squares.push(square);
  }

  let mut unseen = HashSet::new();
  for square in &squares {
    unseen.insert(square.id.clone());
  }

  let mut min_x = squares[0].left;
  let mut min_y = squares[0].top;
  let mut max_x = squares[0].right;
  let mut max_y = squares[0].bottom;

  for square in &squares {
    min_x = min(min_x, square.left);
    min_y = min(min_y, square.top);
    max_x = max(max_x, square.right);
    max_y = max(max_y, square.bottom);
  }

  for x in min_x..=max_x+1 {
    for y in min_y..=max_y+1 {
      let mut seen = vec![];
      for square in squares.clone() {
        let id = square.id.clone();
        if square.contains(x, y) {
            seen.push(id);
        }
      }
      if seen.len() > 1 {
        for item in seen {
          unseen.remove(&item);
        }
      }
    }
  }
  unseen.iter().next().unwrap().clone()
}

//-----------------------------------------------------
// Questions.

q_impl!("3");

#[test]
fn a() {
  assert_eq!(process_data_a("#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"), 4);
}

#[test]
fn b() {
  assert_eq!(process_data_b("#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"), "#3".to_owned());
}
