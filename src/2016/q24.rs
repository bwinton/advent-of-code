//-----------------------------------------------------
// Setup.

use aoc::Day;

use regex::Regex;
use std;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use std::str::FromStr;
use std::usize::MAX;

// static INPUT : &'static str = "###########
// #0.1.....2#
// #.#######.#
// #4.......3#
// ###########";
static INPUT: &'static str = include_str!("data/q24.data");

#[derive(Clone, Eq, PartialEq)]
enum Direction {
  Up,
  Left,
  Down,
  Right,
}

impl fmt::Debug for Direction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let out;
    match *self {
      Direction::Up => out = 'U',
      Direction::Left => out = 'L',
      Direction::Down => out = 'D',
      Direction::Right => out = 'R',
    }
    fmt::Debug::fmt(&out, f)
  }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Location {
  loc: i32,
  x: usize,
  y: usize,
}

impl Ord for Location {
  fn cmp(&self, other: &Location) -> Ordering {
    self.loc.cmp(&other.loc)
  }
}

impl PartialOrd for Location {
  fn partial_cmp(&self, other: &Location) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Contents {
  Wall(),
  Empty(),
  Something(Location),
}

impl FromStr for Contents {
  type Err = ();

  fn from_str(s: &str) -> Result<Contents, ()> {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"^([0-9])$").unwrap();
    }

    match s {
      "#" => Ok(Contents::Wall()),
      "." => Ok(Contents::Empty()),
      _ => {
        let captures = RE.captures(s);
        match captures {
          Some(cap) => Ok(Contents::Something(Location {
            loc: cap[1].parse().unwrap(),
            x: 0,
            y: 0,
          })),
          None => {
            println!("Could not parse '{}'!", s);
            Err(())
          }
        }
      }
    }
  }
}

#[derive(Clone, Debug, Eq)]
struct State {
  x: usize,
  y: usize,
  moves: Vec<Direction>,
}

impl Ord for State {
  fn cmp(&self, other: &State) -> Ordering {
    let self_moves = -(self.moves.len() as i32);
    let other_moves = -(other.moves.len() as i32);
    self_moves.cmp(&other_moves)
  }
}

impl PartialOrd for State {
  fn partial_cmp(&self, other: &State) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for State {
  fn eq(&self, other: &State) -> bool {
    self.x == other.x && self.y == other.y
  }
}

impl Hash for State {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.x.hash(state);
    self.y.hash(state);
  }
}

impl State {
  pub fn is_winning(&self, target: &Location) -> bool {
    self.x == target.x && self.y == target.y
  }
}

fn get_board(input: &str, locations: &mut Vec<Location>) -> Vec<Vec<Contents>> {
  let mut board: Vec<Vec<Contents>> = Vec::new();
  for line in input.lines() {
    let mut row = Vec::new();
    for character in line.chars() {
      let mut last: Contents = character.to_string().parse().unwrap();
      if let Contents::Something(curr) = last {
        let new = Location {
          loc: curr.loc,
          x: row.len(),
          y: board.len(),
        };
        locations.push(new.clone());
        last = Contents::Something(new);
      }
      row.push(last.clone());
    }
    board.push(row);
  }
  locations.sort();
  board
}

fn get_next_states(current: &State, board: &[Vec<Contents>], seen: &[State]) -> Vec<State> {
  let mut rv = Vec::new();

  for direction in &[
    Direction::Up,
    Direction::Left,
    Direction::Down,
    Direction::Right,
  ] {
    let mut x = current.x;
    let mut y = current.y;
    match *direction {
      Direction::Up => y -= 1,
      Direction::Left => x -= 1,
      Direction::Down => y += 1,
      Direction::Right => x += 1,
    }
    let location = &board[y][x];
    if location == &Contents::Wall() {
      continue;
    }

    let mut moves = current.moves.clone();
    moves.push(direction.clone());

    rv.push(State { x, y, moves });
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

fn find_shortest_path(start: &Location, target: &Location, board: &[Vec<Contents>]) -> usize {
  let initial_state = State {
    x: start.x,
    y: start.y,
    moves: Vec::new(),
  };

  let mut next = BinaryHeap::new();
  let mut seen = HashSet::new();
  next.push(initial_state);
  while !next.is_empty() {
    let state = next.pop().unwrap();
    if state.is_winning(target) {
      return state.moves.len();
    }
    seen.insert(state.clone());
    let upcoming: Vec<_> = seen
      .clone()
      .into_iter()
      .chain(next.clone().into_iter())
      .collect();
    let next_states = get_next_states(&state, board, &upcoming);
    for state in &next_states {
      next.push(state.clone());
    }
    // if seen.len() % 1000 == 0 {
    //   println!("Gone through {} states. Current len: {}", &seen.len(), &state.moves.len());
    // }
  }
  std::usize::MAX
}

fn get_permutations(input: &[Location]) -> Vec<Vec<Location>> {
  if input.len() == 1 {
    let x: Vec<Vec<Location>> = vec![vec![input[0].clone()]];
    return x;
  }

  let mut result: Vec<Vec<Location>> = Vec::new();

  for i in 0..input.len() {
    let mut temp: Vec<Location> = input.to_vec();
    temp.swap(0, i);
    let (first, rest) = temp.split_first().unwrap();
    for mut perm in get_permutations(rest) {
      perm.insert(0, first.clone());
      result.push(perm.clone());
    }
  }

  result
}

fn get_distance(
  start: &Location,
  target: &Location,
  distances: &HashMap<(&Location, &Location), usize>,
) -> usize {
  let key = if start > target {
    (target, start)
  } else {
    (start, target)
  };
  // println!("Checking for {:?} in {:?}", &key, &distances);
  *distances.get(&key).unwrap()
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
    let mut result = MAX;
    let mut locations = Vec::new();
    let board = get_board(INPUT, &mut locations);
    let mut distances = HashMap::new();

    for i in 0..locations.len() {
      for j in i + 1..locations.len() {
        let length = find_shortest_path(&locations[i], &locations[j], &board);
        distances.insert((&locations[i], &locations[j]), length);
      }
    }
    // println!("Shortest Paths {:?}", distances);

    let (initial, rest) = locations.split_first().unwrap();

    for mut perm in get_permutations(rest) {
      perm.insert(0, initial.clone());
      let mut total_distance = 0;
      for (i, item) in perm.iter().enumerate() {
        if i + 1 < perm.len() {
          total_distance += get_distance(item, &perm[i + 1], &distances);
        }
      }
      if total_distance < result {
        let display: Vec<i32> = perm.iter().map(|item| item.loc).collect();
        println!("Perm: {:?} => {} < {}", display, total_distance, result);
        result = total_distance;
      }
    }

    println!("Result = {}", result);
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    let mut result = MAX;
    let mut locations = Vec::new();
    let board = get_board(INPUT, &mut locations);
    let mut distances = HashMap::new();

    for i in 0..locations.len() {
      for j in i + 1..locations.len() {
        let length = find_shortest_path(&locations[i], &locations[j], &board);
        distances.insert((&locations[i], &locations[j]), length);
      }
    }
    // println!("Shortest Paths {:?}", distances);

    let (initial, rest) = locations.split_first().unwrap();

    for mut perm in get_permutations(rest) {
      perm.insert(0, initial.clone());
      perm.push(initial.clone());
      let mut total_distance = 0;
      for (i, item) in perm.iter().enumerate() {
        if i + 1 < perm.len() {
          total_distance += get_distance(item, &perm[i + 1], &distances);
        }
      }
      if total_distance < result {
        let display: Vec<i32> = perm.iter().map(|item| item.loc).collect();
        println!("Perm: {:?} => {} < {}", display, total_distance, result);
        result = total_distance;
      }
    }
    println!("Result = {}", result);
  }
}
