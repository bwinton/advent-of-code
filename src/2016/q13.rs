//-----------------------------------------------------
// Setup.

use aoc::Day;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

// static INPUT_NUMBER : i32 = 10;
// static INPUT_TARGET_X : i32 = 7;
// static INPUT_TARGET_Y : i32 = 4;
static INPUT_NUMBER: i32 = 1362;
static INPUT_TARGET_X: i32 = 31;
static INPUT_TARGET_Y: i32 = 39;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Eq)]
struct State {
  x: i32,
  y: i32,
  moves: i32,
  wall: bool,
  dist: i32,
}

impl State {
  pub fn new(x: i32, y: i32, moves: i32) -> State {
    let wall = cell_is_wall(x, y);
    let delta_x = INPUT_TARGET_X - x;
    let delta_y = INPUT_TARGET_Y - y;
    State {
      x,
      y,
      moves,
      wall,
      dist: delta_x * delta_x + delta_y * delta_y,
    }
  }
}

impl Ord for State {
  fn cmp(&self, other: &State) -> Ordering {
    let move_cmp = (-self.moves).cmp(&-other.moves);
    if move_cmp == Ordering::Equal { self.dist.cmp(&other.dist) } else { move_cmp }
  }
}

impl PartialOrd for State {
  fn partial_cmp(&self, other: &State) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for State {
  fn eq(&self, other: &State) -> bool {
    self.x == other.x && self.y == other.y && self.dist == other.dist
  }
}

fn cell_is_wall(x: i32, y: i32) -> bool {
  let mut number = x * x + 3 * x + 2 * x * y + y + y * y + INPUT_NUMBER;
  let mut count = 0;
  while number != 0 {
    number &= number - 1;
    count += 1;
  }
  count % 2 == 1
}

fn get_next_states(current: &State, seen: &[State]) -> Vec<State> {
  let mut rv = Vec::new();

  rv.push(State::new(current.x + 1, current.y, current.moves + 1));
  rv.push(State::new(current.x, current.y + 1, current.moves + 1));
  if current.x > 0 {
    rv.push(State::new(current.x - 1, current.y, current.moves + 1));
  }
  if current.y > 0 {
    rv.push(State::new(current.x, current.y - 1, current.moves + 1));
  }

  let mut temp = seen.to_vec();
  rv.retain(|item| {
    let missing = !temp.contains(item);
    if missing {
      temp.push(item.clone());
    }
    missing
  });

  rv
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
  fn number(&self) -> String {
    String::from("13")
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    println!();
    let mut result = 0;

    let mut next = BinaryHeap::new();
    let mut seen: Vec<State> = Vec::new();
    let initial_state = State::new(1, 1, 0);
    next.push(initial_state);

    while !next.is_empty() {
      let current = next.pop().unwrap();

      if current.x == INPUT_TARGET_X && current.y == INPUT_TARGET_Y {
        println!("WINNING!!!!   {:?}", current);
        result = current.moves;
        break;
      }

      seen.push(current.clone());
      let mut upcoming = seen.clone();
      upcoming.extend(next.clone().into_vec());
      let next_states = get_next_states(&current, &upcoming);
      for state in &next_states {
        if state.wall {
          seen.push(state.clone());
        } else {
          next.push(state.clone());
        }
      }
    }

    println!("Result = {}", result);
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    let mut result = 0;

    let mut next: Vec<State> = Vec::new();
    let mut seen: Vec<State> = Vec::new();
    let initial_state = State::new(1, 1, 0);
    next.push(initial_state);

    while !next.is_empty() {
      let current = next.remove(0);

      if current.moves > 50 {
        println!("Done!!!!   {:?}", current);
        result = seen.len();
        break;
      }

      seen.push(current.clone());
      let mut upcoming = seen.clone();
      upcoming.extend(next.clone());
      let next_states = get_next_states(&current, &upcoming);
      for state in &next_states {
        if !state.wall {
          next.push(state.clone());
        }
      }
    }
    println!("Result = {}", result);
  }
}
