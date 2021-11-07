//-----------------------------------------------------
// Setup.

use aoc::Day;
use crypto::{digest::Digest, md5::Md5};
use std::{cmp::Ordering, collections::BinaryHeap};

// static INPUT : &'static str = "hijkl";
// static INPUT : &'static str = "ihgpwlah";
// static INPUT : &'static str = "kglvqrro";
// static INPUT : &'static str = "ulqzkmiv";
static INPUT: &str = "yjjvjgan";

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Clone, Debug)]
struct State {
    x: usize,
    y: usize,
    path: String,
    open_doors: Vec<Direction>,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.path.len().cmp(&self.path.len())
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        self.path == other.path
    }
}

impl Eq for State {}

impl State {
    pub fn new(path: &str, x: usize, y: usize) -> State {
        // println!("{:?}", path);
        let mut hasher = Md5::new();
        hasher.input(INPUT.as_bytes());
        hasher.input(path.as_bytes());
        let mut output = [0; 16]; // An MD5 is 16 bytes
        hasher.result(&mut output);
        hasher.reset();
        let mut open_doors = Vec::new();
        if output[0] >> 4 > 10 && y > 0 {
            open_doors.push(Direction::Up);
        }
        if output[0] & 0xf > 10 && y < 3 {
            open_doors.push(Direction::Down);
        }
        if output[1] >> 4 > 10 && x > 0 {
            open_doors.push(Direction::Left);
        }
        if output[1] & 0xf > 10 && x < 3 {
            open_doors.push(Direction::Right);
        }

        State {
            x,
            y,
            path: path.to_string(),
            open_doors,
        }
    }

    pub fn is_winning(&self) -> bool {
        self.x == 3 && self.y == 3
    }
}

fn get_next_states(state: &State) -> Vec<State> {
    let mut rv = Vec::new();
    if state.x == 3 && state.y == 3 {
        return rv;
    }
    for direction in state.open_doors.clone() {
        match direction {
            Direction::Up => {
                let mut path = state.path.clone();
                path.push('U');
                rv.push(State::new(&path, state.x, state.y - 1));
            }
            Direction::Down => {
                let mut path = state.path.clone();
                path.push('D');
                rv.push(State::new(&path, state.x, state.y + 1));
            }
            Direction::Left => {
                let mut path = state.path.clone();
                path.push('L');
                rv.push(State::new(&path, state.x - 1, state.y));
            }
            Direction::Right => {
                let mut path = state.path.clone();
                path.push('R');
                rv.push(State::new(&path, state.x + 1, state.y));
            }
        }
    }
    rv
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
        let mut result = String::from("No path found.");
        let mut next = BinaryHeap::new();
        next.push(State::new(&String::from(""), 0, 0));
        while !next.is_empty() {
            let state = next.pop().unwrap();
            if state.is_winning() {
                result = state.path;
                break;
            }
            let next_states = get_next_states(&state);
            for upcoming in &next_states {
                next.push(upcoming.clone());
            }
        }
        // println!("{:?}", next);
        println!("Result = {}", result);
    }

    fn b(&self) {
        print!("{}B: ", self.number());
        let mut result = 0;
        let mut next = BinaryHeap::new();
        next.push(State::new(&String::from(""), 0, 0));
        while !next.is_empty() {
            let state = next.pop().unwrap();
            if state.is_winning() && state.path.len() > result {
                result = state.path.len();
            }
            let next_states = get_next_states(&state);
            for upcoming in &next_states {
                next.push(upcoming.clone());
            }
        }
        // println!("{:?}", next);
        println!("Result = {}", result);
    }
}
