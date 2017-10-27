//-----------------------------------------------------
// Setup.

use day;
use regex::Regex;
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

// static A_INPUT : &'static str = "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
// The second floor contains a hydrogen generator.
// The third floor contains a lithium generator.
// The fourth floor contains nothing relevant.";
static A_INPUT : &'static str = "The first floor contains a strontium generator, a strontium-compatible microchip, a plutonium generator, and a plutonium-compatible microchip.
The second floor contains a thulium generator, a ruthenium generator, a ruthenium-compatible microchip, a curium generator, and a curium-compatible microchip.
The third floor contains a thulium-compatible microchip.
The fourth floor contains nothing relevant.";
static B_INPUT : &'static str = "The first floor contains a strontium generator, a strontium-compatible microchip, a plutonium generator, an elerium generator, an elerium-compatible microchip, a dilithium generator, a dilithium-compatible microchip, and a plutonium-compatible microchip.
The second floor contains a thulium generator, a ruthenium generator, a ruthenium-compatible microchip, a curium generator, and a curium-compatible microchip.
The third floor contains a thulium-compatible microchip.
The fourth floor contains nothing relevant.";


#[derive(Clone)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
enum Item {
  Generator(String),
  Microchip(String)
}

impl Ord for Item {
  fn cmp(&self, other: &Item) -> Ordering {
    match *self {
      Item::Generator(ref me) => {
        match *other {
          Item::Generator(ref them) => {
            return me.cmp(&them);
          },
          Item::Microchip(ref _them) => {
            return Ordering::Less;
          }
        }
      },
      Item::Microchip(ref me) => {
        match *other {
          Item::Generator(ref _them) => {
            return Ordering::Greater;
          },
          Item::Microchip(ref them) => {
            return me.cmp(&them);
          }
        }
      }
    }
  }
}

impl PartialOrd for Item {
  fn partial_cmp(&self, other: &Item) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl FromStr for Item {
  type Err = ();

  fn from_str(s: &str) -> Result<Item, ()> {
    let gen_re: Regex = Regex::new(r"^an? ([a-z]+) generator$").unwrap();
    let gen_captures = gen_re.captures(s);
    match gen_captures {
      Some(cap) => {
        return Ok(Item::Generator(String::from(cap.at(1).unwrap_or(""))));
      },
      None => {}
    }

    let chip_re: Regex = Regex::new(r"^an? ([a-z]+)-compatible microchip$").unwrap();
    let chip_captures = chip_re.captures(s);
    match chip_captures {
      Some(cap) => {
        return Ok(Item::Microchip(String::from(cap.at(1).unwrap_or(""))));
      },
      None => {}
    }
    return Err(());
  }
}

impl fmt::Display for Item {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Item::Generator(ref me) => {
        write!(f, "{}G", me.to_uppercase().chars().next().unwrap_or('?'))
      }
      Item::Microchip(ref me) => {
        write!(f, "{}M", me.to_uppercase().chars().next().unwrap_or('?'))
      }
    }
  }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
struct FloorDesc {
  pairs: usize,
  generators: usize,
  microchips: usize
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Ord)]
#[derive(PartialOrd)]
struct Floor {
  number: i32,
  items: Vec<Item>,
  desc: FloorDesc
}

impl Floor {
  fn add_item(&mut self, item: Item) {
    self.items.push(item);
    self.items.sort();
    self.desc = self.get_desc();
  }

  fn remove_item(&mut self, i: usize) {
    self.items.remove(i);
    self.desc = self.get_desc();
  }

  fn get_desc(&self) -> FloorDesc{
    let mut rv = FloorDesc{pairs:0, generators:0, microchips:0};
    let mut generators : Vec<String> = Vec::new();
    let mut microchips : Vec<String> = Vec::new();
    for item in self.items.clone() {
      match item {
        Item::Generator(data) => {generators.push(data)},
        Item::Microchip(data) => {microchips.push(data)}
      }
    }
    for chip in microchips.clone() {
      match generators.binary_search(&chip) {
        Ok(_) => {rv.pairs += 1},
        Err(_) => {rv.microchips += 1}
      }
    }
    for chip in generators.clone() {
      match microchips.binary_search(&chip) {
        Ok(_) => {},
        Err(_) => {rv.generators += 1}
      }
    }
    rv
  }
}

impl FromStr for Floor {
  type Err = ();

  fn from_str(s: &str) -> Result<Floor, ()> {
    let re: Regex = Regex::new(r"^The ([a-z]*) floor contains (.*)\.$").unwrap();
    let mut rv = Floor{number: -1, items: Vec::new(), desc: FloorDesc{pairs: 0, generators: 0, microchips: 0}};
    let captures = re.captures(s);
    match captures {
      None => return Err(()),
      Some(cap) => {
        match cap.at(1).unwrap_or("").as_ref() {
          "first" => {rv.number = 1},
          "second" => {rv.number = 2},
          "third" => {rv.number = 3},
          "fourth" => {rv.number = 4},
          _ => {return Err(())}
        }
        let items = cap.at(2).unwrap_or("");
        let item_re: Regex = Regex::new(r"an? [a-z]*(:?-compatible microchip| generator)").unwrap();
        for item_captures in item_re.captures_iter(items) {
          let item_opt :Result<Item, ()> = item_captures.at(0).unwrap().parse();
          match item_opt {
            Err(()) => {},
            Ok(item) => {
              rv.add_item(item);
            }
          }
        }
      }
    }
    return Ok(rv);
  }
}

impl fmt::Display for Floor {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut result = write!(f, "F{}: ", self.number);
    if result.is_err() {
      return result;
    }
    for item in self.items.clone() {
      result = write!(f, "{} ", item);
      if result.is_err() {
        return result;
      }
    }
    write!(f, "")
  }
}

impl PartialEq for Floor {
  fn eq(&self, other: &Floor) -> bool {
    self.desc == other.desc
  }
}

impl Eq for Floor {}


#[derive(Clone)]
#[derive(Debug)]
#[derive(Ord)]
#[derive(PartialOrd)]
struct State {
  moves: i32,
  previous: Option<usize>,
  index: Option<usize>,
  elevator: usize,
  floors: Vec<Floor>
}

impl State {
  fn is_valid(&self, seen: &Vec<State>) -> bool {
    for floor in self.floors.clone() {
      if floor.desc.microchips > 0 && floor.desc.pairs + floor.desc.generators > 0 {
        return false;
      }
    }

    // Check to see if this is already in the seen states with our crazy comparison function.
    return !seen.contains(self);
  }

  fn is_winning(&self) -> bool {
    for floor in self.floors.clone() {
      if floor.number != self.floors.len() as i32 {
        if floor.items.len() != 0 {
          return false;
        }
      }
    }
    true
  }
}

impl PartialEq for State {
  fn eq(&self, other: &State) -> bool {
    let equal_floors = self.floors == other.floors;
    self.elevator == other.elevator && equal_floors
  }
}

impl Eq for State {}

impl fmt::Display for State {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut result = writeln!(f, "i:{:?}, p:{:?}, moves:{}", self.index, self.previous, self.moves);
    if result.is_err() {
      return result;
    }
    let mut floors = self.floors.clone();
    floors.reverse();
    for floor in floors {
      result = write!(f, "{}", floor);
      if result.is_err() {
        return result;
      }
      if floor.number - 1 == (self.elevator as i32) {
        result = writeln!(f, " E");
        if result.is_err() {
          return result;
        }
      } else {
        result = writeln!(f, " .");
        if result.is_err() {
          return result;
        }
      }
    }
    result
    // F4 .  .  .  .  .
    // F3 .  .  .  LG .
    // F2 E  HG HM .  .
    // F1 .  .  .  .  LM
  }
}

fn move_items(state: &State, going_up: bool, states: &mut Vec<State>) {
  let items = &state.floors[state.elevator].items;
  let mut next_stop = state.elevator + 1;
  if !going_up {
    next_stop = state.elevator - 1;
  }

  let mut template = state.clone();
  template.moves += 1;
  template.elevator = next_stop;
  template.previous = state.index;
  template.index = None;
  let mut next;

  for i in 0..items.len() {
    next = template.clone();
    next.floors[state.elevator].remove_item(i);
    next.floors[next.elevator].add_item(items[i].clone());
    states.push(next);

    for j in i+1..items.len() {
      next = template.clone();
      next.floors[state.elevator].remove_item(j);
      next.floors[state.elevator].remove_item(i);
      next.floors[next.elevator].add_item(items[i].clone());
      next.floors[next.elevator].add_item(items[j].clone());
      states.push(next);
    }
  }
}

fn get_next_state(state: &State, seen: &Vec<State>) -> Vec<State> {
  // generate all possible turns, pruning already-seen and invalid states.
  let mut rv = Vec::new();
  if state.elevator < state.floors.len()-1 {
    move_items(state, true, rv.as_mut());
  }
  // }
  if state.elevator > 0 {
    move_items(state, false, rv.as_mut());
  }
  let mut temp = seen.to_vec();
  rv.retain(|item| {
    if item.is_valid(&temp) {
      temp.push(item.clone());
      return true;
    }
    return false;
  });
  rv
}

fn get_result(input: &'static str) -> i32 {
    let mut result = 0;
    let mut next : Vec<State> = Vec::new();
    let mut seen : Vec<State> = Vec::new();

    let mut initial_state = State{index: None, previous: None, moves: 0, elevator: 0, floors:Vec::new()};
    for line in input.lines() {
      let floor : Floor = line.parse().unwrap();
      initial_state.floors.push(floor);
    }
    next.push(initial_state);

    let mut count = 0;
    while next.len() > 0 {
      let mut current = next.remove(0);
      // If the current is everything on the 4th floor, we win!!!
      if current.is_winning() {
        println!("Found a winner at {}!", count);
        println!("{}", current);
        result = current.moves;
        // while let Some(i) = current.previous {
        //   println!("");
        //   current = seen[i].clone();
        //   println!("{}", current);
        // }
        break;
      }
      current.index = Some(seen.len());
      seen.push(current.clone());
      let mut upcoming = seen.clone();
      upcoming.extend(next.clone());
      next.append(&mut get_next_state(&current, &upcoming));
      if count % 100 == 0 {
        // println!("{}: {}", count, next.len());
        if count % 3000 == 0 {
          println!("{}", current);
        }
      }
      count += 1;
    }

    return result;
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl day::Day for Q {
  fn number(&self) -> String {
    return String::from("11");
  }

  fn a(&self) {
    println!("{}A: ", self.number());
    println!("Result = {}", get_result(A_INPUT));
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    println!("Result = {}", get_result(B_INPUT));
  }
}
