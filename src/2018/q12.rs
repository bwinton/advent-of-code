//-----------------------------------------------------
// Setup.

use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::default::Default;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::{FromStr, Lines};

static INPUT: &str = include_str!("data/q12.data");

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    min: i32,
    max: i32,
    cells: HashSet<i32>,
}

impl Default for State {
    fn default() -> Self {
        State {
            min: i32::max_value(),
            max: i32::min_value(),
            cells: HashSet::new(),
        }
    }
}

impl FromStr for State {
    type Err = ();

    fn from_str(s: &str) -> Result<State, ()> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"initial state: ([#.]+)").unwrap();
        }

        if let Some(cap) = RE.captures(s) {
            let mut state = State::default();
            for (i, c) in cap[1].chars().enumerate() {
                if c == '#' {
                    state.min = min(state.min, i as i32);
                    state.max = max(state.max, i as i32);
                    state.cells.insert(i as i32);
                }
            }
            return Ok(state);
        }

        Err(())
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // write!(f, "{}: ", self.min)?;
        // for i in self.min..=self.max {
        for i in -3..=35 {
            write!(f, "{}", if self.cells.contains(&i) { "#" } else { "." },)?;
        }
        // write!(f, ": {}", self.max)
        write!(f, ": {}", self.sum())
        // Ok(())
    }
}

impl State {
    fn step(&self, rules: &HashSet<Vec<bool>>) -> State {
        let mut state = State::default();

        for i in self.min - 4..=self.max {
            let mut key = vec![];
            for x in i..=i + 4 {
                key.push(self.cells.contains(&x));
            }
            if rules.contains(&key) {
                state.min = min(state.min, i + 2);
                state.max = max(state.max, i + 2);
                state.cells.insert(i + 2);
            }
        }
        state
    }

    fn sum(&self) -> i32 {
        self.cells.iter().sum()
    }
}

fn get_rules(lines: Lines) -> HashSet<Vec<bool>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([#.]{5}) => ([#.])").unwrap();
    }

    let mut rv = HashSet::new();
    for line in lines {
        let cap = RE.captures(line).unwrap();
        if cap[2].starts_with('#') {
            rv.insert(cap[1].chars().map(|x| x == '#').collect());
        }
    }
    rv
}

fn process_data_a(data: &str) -> i32 {
    let mut lines = data.lines();
    let mut state: State = lines.next().unwrap().parse().unwrap();
    lines.next();
    let rules = get_rules(lines);
    for _ in 0..20 {
        state = state.step(&rules);
    }
    state.sum()
}

fn process_data_b(data: &str) -> i64 {
    let mut lines = data.lines();
    let mut state: State = lines.next().unwrap().parse().unwrap();
    lines.next();
    let rules = get_rules(lines);
    let mut temp = state.sum();
    for _ in 0..90_i64 {
        temp = state.sum();
        state = state.step(&rules);
    }

    i64::from(state.sum()) + (50_000_000_000_i64 - 90) * i64::from(state.sum() - temp)
}

//-----------------------------------------------------
// Questions.

q_impl!("12");

#[test]
fn a() {
    assert_eq!(
        process_data_a(
            "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #"
        ),
        325
    );
}

#[test]
fn b() {
    // assert_eq!(process_data_b(""), 0);
}
