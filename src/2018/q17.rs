//-----------------------------------------------------
// Setup.

use regex::Regex;
use std::collections::HashSet;

static INPUT: &str = include_str!("data/q17.data");

#[derive(Debug)]
struct Board {
    clay: HashSet<(i32, i32)>,
    water: HashSet<(i32, i32)>,
    sinks: HashSet<(i32, i32)>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Board {
    fn new(clay: HashSet<(i32, i32)>) -> Self {
        let water = HashSet::new();
        let sinks = HashSet::new();
        let min_y = *clay.iter().map(|(_x, y)| y).min().unwrap();
        let max_y = *clay.iter().map(|(_x, y)| y).max().unwrap();
        let min_x = *clay.iter().map(|(x, _y)| x).min().unwrap() - 1;
        let max_x = *clay.iter().map(|(x, _y)| x).max().unwrap() + 1;
        Board {
            clay,
            water,
            sinks,
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    fn get_next(&mut self, curr: (i32, i32), active: &[(i32, i32)]) -> Vec<(i32, i32)> {
        let mut rv = vec![];
        let below = (curr.0, curr.1 + 1);
        let left = (curr.0 - 1, curr.1);
        let right = (curr.0 + 1, curr.1);
        if self.clay.contains(&below) || self.water.contains(&below) {
            // There's something beneath us, so spread out.
            if !self.clay.contains(&left)
                && !self.water.contains(&left)
                && !self.sinks.contains(&left)
                && !active.contains(&left)
            {
                rv.push(left);
            }
            if !self.clay.contains(&right)
                && !self.water.contains(&right)
                && !self.sinks.contains(&right)
                && !active.contains(&right)
            {
                rv.push(right);
            }
        } else if !active.contains(&below) && !self.sinks.contains(&below) && below.1 <= self.max_y
        {
            rv.push(below);
        }
        if rv.is_empty() {
            if self.sinks.contains(&below)
                || self.sinks.contains(&left)
                || self.sinks.contains(&right)
                || below.1 > self.max_y
            {
                self.sinks.insert(curr);
                // If I'm a sink, then everything I'm connected to on my level is also a sink.
                let mut right = right;
                while !self.sinks.contains(&right) && self.water.contains(&right) {
                    self.water.remove(&right);
                    self.sinks.insert(right);
                    right = (right.0 + 1, right.1);
                }
                let mut left = left;
                while !self.sinks.contains(&left) && self.water.contains(&left) {
                    self.water.remove(&left);
                    self.sinks.insert(left);
                    left = (left.0 + 1, left.1);
                }
            } else {
                self.water.insert(curr);
            }
        }
        rv
    }
    fn print(&self, active: &[(i32, i32)]) {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                let mut printed = self.clay.contains(&(x, y));
                if printed {
                    print!("#");
                };
                if self.water.contains(&(x, y)) {
                    print!("~");
                    printed = true;
                }
                if self.sinks.contains(&(x, y)) {
                    print!("|");
                    printed = true;
                }
                if active.contains(&(x, y)) {
                    print!("?");
                    printed = true;
                }
                if !printed {
                    print!(" ");
                }
            }
            println!("\\");
        }
    }
}

fn parse_wall(s: &str) -> HashSet<(i32, i32)> {
    let x_re: &Regex = regex!(r"x=(\d+), y=(\d+)\.\.(\d+)");
    let y_re: &Regex = regex!(r"y=(\d+), x=(\d+)\.\.(\d+)");
    let mut rv = HashSet::new();

    if let Some(cap) = x_re.captures(s) {
        let x = cap[1].parse().unwrap();
        for y in cap[2].parse().unwrap()..=cap[3].parse().unwrap() {
            rv.insert((x, y));
        }
    } else if let Some(cap) = y_re.captures(s) {
        let y = cap[1].parse().unwrap();
        for x in cap[2].parse().unwrap()..=cap[3].parse().unwrap() {
            rv.insert((x, y));
        }
    } else {
        Board::new(rv.clone()).print(&[]);
        println!("Could not parse \"{}\"!!!", s);
    }

    rv
}

fn process_data_a(data: &str) -> usize {
    let mut clay = HashSet::new();
    for line in data.lines() {
        for cell in parse_wall(line) {
            clay.insert(cell);
        }
    }

    let mut board = Board::new(clay);

    let spring = (500, 0);
    let mut active = vec![(spring.0, spring.1)];
    while !active.is_empty() {
        let curr = active.pop().unwrap();
        let mut next = board.get_next(curr, &active);
        if !next.is_empty() {
            active.push(curr);
            active.append(&mut next);
        }
        active.sort_by_key(|&(x, y)| (y, x));
    }

    let min = board.min_y;
    let max = board.max_y;
    board.water.retain(|&(_x, y)| y >= min && y <= max);
    board.sinks.retain(|&(_x, y)| y >= min && y <= max);

    board.water.len() + board.sinks.len()
    // 63478 is also too high.
    // 36614 is too high.
    // 32558 still to high.
    // 32552
}

fn process_data_b(data: &str) -> usize {
    let mut clay = HashSet::new();
    for line in data.lines() {
        for cell in parse_wall(line) {
            clay.insert(cell);
        }
    }

    let mut board = Board::new(clay);

    let spring = (500, 0);
    let mut active = vec![(spring.0, spring.1)];
    while !active.is_empty() {
        let curr = active.pop().unwrap();
        let mut next = board.get_next(curr, &active);
        if !next.is_empty() {
            active.push(curr);
            active.append(&mut next);
        }
        active.sort_by_key(|&(x, y)| (y, x));
    }

    let min = board.min_y;
    let max = board.max_y;
    board.water.retain(|&(_x, y)| y >= min && y <= max);
    board.sinks.retain(|&(_x, y)| y >= min && y <= max);

    board.water.len()
}

//-----------------------------------------------------
// Questions.

q_impl!("17");

#[test]
fn a() {
    assert_eq!(
        process_data_a(
            "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504"
        ),
        57
    );

    assert_eq!(
        process_data_a(
            "x=499, y=5..15
y=15, x=500..516
x=517, y=6..15
x=508, y=3..10
x=510, y=3..10
y=11, x=508..510
"
        ),
        129
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(
            "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504"
        ),
        29
    );
}
