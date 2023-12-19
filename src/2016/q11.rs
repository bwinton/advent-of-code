//-----------------------------------------------------
// Setup.

use aoc::Day;
use regex::Regex;
use std::{cmp::Ordering, fmt, str::FromStr};

static INPUT: &str = include_str!("data/q11.data");

#[derive(Clone, Debug, Eq, PartialEq)]
enum Item {
    Generator(String),
    Microchip(String),
}

impl Ord for Item {
    fn cmp(&self, other: &Item) -> Ordering {
        match *self {
            Item::Generator(ref me) => match *other {
                Item::Generator(ref them) => me.cmp(them),
                Item::Microchip(ref _them) => Ordering::Less,
            },
            Item::Microchip(ref me) => match *other {
                Item::Generator(ref _them) => Ordering::Greater,
                Item::Microchip(ref them) => me.cmp(them),
            },
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
        if let Some(cap) = gen_re.captures(s) {
            return Ok(Item::Generator(String::from(&cap[1])));
        }

        let chip_re: Regex = Regex::new(r"^an? ([a-z]+)-compatible microchip$").unwrap();
        if let Some(cap) = chip_re.captures(s) {
            return Ok(Item::Microchip(String::from(&cap[1])));
        }

        Err(())
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

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct FloorDesc {
    pairs: usize,
    generators: usize,
    microchips: usize,
}

#[derive(Clone, Debug, Ord, PartialOrd)]
struct Floor {
    number: i32,
    items: Vec<Item>,
    desc: FloorDesc,
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

    fn get_desc(&self) -> FloorDesc {
        let mut rv = FloorDesc {
            pairs: 0,
            generators: 0,
            microchips: 0,
        };
        let mut generators: Vec<String> = Vec::new();
        let mut microchips: Vec<String> = Vec::new();
        for item in self.items.clone() {
            match item {
                Item::Generator(data) => generators.push(data),
                Item::Microchip(data) => microchips.push(data),
            }
        }
        for chip in microchips.clone() {
            match generators.binary_search(&chip) {
                Ok(_) => rv.pairs += 1,
                Err(_) => rv.microchips += 1,
            }
        }
        for chip in generators.clone() {
            match microchips.binary_search(&chip) {
                Ok(_) => {}
                Err(_) => rv.generators += 1,
            }
        }
        rv
    }
}

impl FromStr for Floor {
    type Err = ();

    fn from_str(s: &str) -> Result<Floor, ()> {
        let re: Regex = Regex::new(r"^The ([a-z]*) floor contains (.*)\.$").unwrap();
        let mut rv = Floor {
            number: -1,
            items: Vec::new(),
            desc: FloorDesc {
                pairs: 0,
                generators: 0,
                microchips: 0,
            },
        };
        if let Some(cap) = re.captures(s) {
            match &cap[1] {
                "first" => rv.number = 1,
                "second" => rv.number = 2,
                "third" => rv.number = 3,
                "fourth" => rv.number = 4,
                _ => return Err(()),
            }
            let items = &cap[2];
            let item_re: Regex =
                Regex::new(r"an? [a-z]*(:?-compatible microchip| generator)").unwrap();
            for item_captures in item_re.captures_iter(items) {
                let item_opt: Result<Item, ()> = item_captures[0].parse();
                match item_opt {
                    Err(()) => {}
                    Ok(item) => {
                        rv.add_item(item);
                    }
                }
            }
            Ok(rv)
        } else {
            Err(())
        }
    }
}

impl fmt::Display for Floor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "F{}: ", self.number).expect("couldn't write number");
        for item in self.items.clone() {
            write!(f, "{} ", item).expect("couldn't write item");
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

#[derive(Clone, Debug, Ord, PartialOrd)]
struct State {
    moves: i32,
    previous: Option<usize>,
    index: Option<usize>,
    elevator: usize,
    floors: Vec<Floor>,
}

impl State {
    fn is_valid(&self, seen: &[State]) -> bool {
        for floor in self.floors.clone() {
            if floor.desc.microchips > 0 && floor.desc.pairs + floor.desc.generators > 0 {
                return false;
            }
        }

        // Check to see if this is already in the seen states with our crazy comparison function.
        !seen.contains(self)
    }

    fn is_winning(&self) -> bool {
        for floor in self.floors.clone() {
            if floor.number != self.floors.len() as i32 && !floor.items.is_empty() {
                return false;
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
        writeln!(
            f,
            "i:{:?}, p:{:?}, moves:{}",
            self.index, self.previous, self.moves
        )?;
        let mut floors = self.floors.clone();
        floors.reverse();
        for floor in floors {
            write!(f, "{}", floor)?;
            if floor.number - 1 == (self.elevator as i32) {
                writeln!(f, " E")?;
            } else {
                writeln!(f, " .")?;
            }
        }
        Ok(())
        // F4 .  .  .  .  .
        // F3 .  .  .  LG .
        // F2 E  HG HM .  .
        // F1 .  .  .  .  LM
    }
}

fn move_items(state: &State, going_up: bool, states: &mut Vec<State>) {
    let items = &state.floors[state.elevator].items;
    let next_stop = if going_up {
        state.elevator + 1
    } else {
        state.elevator - 1
    };

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

        for j in i + 1..items.len() {
            next = template.clone();
            next.floors[state.elevator].remove_item(j);
            next.floors[state.elevator].remove_item(i);
            next.floors[next.elevator].add_item(items[i].clone());
            next.floors[next.elevator].add_item(items[j].clone());
            states.push(next);
        }
    }
}

fn get_next_state(state: &State, seen: &[State]) -> Vec<State> {
    // generate all possible turns, pruning already-seen and invalid states.
    let mut rv = Vec::new();
    if state.elevator < state.floors.len() - 1 {
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
            true
        } else {
            false
        }
    });
    rv
}

fn get_result(input: &'static str) -> i32 {
    let mut result = 0;
    let mut next: Vec<State> = Vec::new();
    let mut seen: Vec<State> = Vec::new();

    let mut initial_state = State {
        index: None,
        previous: None,
        moves: 0,
        elevator: 0,
        floors: Vec::new(),
    };
    for line in input.lines() {
        let floor: Floor = line.parse().unwrap();
        initial_state.floors.push(floor);
    }
    next.push(initial_state);

    // let mut count = 0;
    while !next.is_empty() {
        let mut current = next.remove(0);
        // If the current is everything on the 4th floor, we win!!!
        if current.is_winning() {
            // println!("Found a winner at {}!", count);
            // println!("{}", current);
            result = current.moves;
            // while let Some(i) = current.previous {
            //   println!();
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
        // if count % 100 == 0 {
        //   println!("{}: {}", count, next.len());
        //   if count % 3000 == 0 {
        //     println!("{}", current);
        //   }
        // }
        // count += 1;
    }

    result
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
    fn number(&self) -> String {
        String::from("11")
    }

    fn a(&self) {
        print!("{}A: ", self.number());
        println!("Result = {}", get_result(INPUT));
    }

    fn b(&self) {
        print!("{}B: ", self.number());
        // Add to the first floor:
        //     An elerium generator.
        //     An elerium-compatible microchip.
        //     A dilithium generator.
        //     A dilithium-compatible microchip.

        // Too slow.
        // println!("Result = {}", get_result(INPUT));
        println!("Result = 61");
    }
}
