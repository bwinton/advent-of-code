//-----------------------------------------------------
// Setup.

use aoc::Day;
use regex::Regex;
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

// Filesystem              Size  Used  Avail  Use%
// static INPUT : &'static str = "/dev/grid/node-x0-y0   10T    8T     2T   80%
// /dev/grid/node-x0-y1   11T    6T     5T   54%
// /dev/grid/node-x0-y2   32T   28T     4T   87%
// /dev/grid/node-x1-y0    9T    7T     2T   77%
// /dev/grid/node-x1-y1    8T    0T     8T    0%
// /dev/grid/node-x1-y2   11T    7T     4T   63%
// /dev/grid/node-x2-y0   10T    6T     4T   60%
// /dev/grid/node-x2-y1    9T    8T     1T   88%
// /dev/grid/node-x2-y2    9T    6T     3T   66%";
static INPUT: &str = include_str!("data/q22.data");

#[derive(Clone, Debug, PartialEq, Eq)]
struct Node {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
    avail: usize,
    goal: bool,
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Node, ()> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new("/dev/grid/node-x([0-9]+)-y([0-9]+) +([0-9]+)T +([0-9]+)T +([0-9]+)T")
                    .unwrap();
        }

        let captures = RE.captures(s);
        match captures {
            Some(cap) => Ok(Node {
                x: cap[1].parse().unwrap(),
                y: cap[2].parse().unwrap(),
                size: cap[3].parse().unwrap(),
                used: cap[4].parse().unwrap(),
                avail: cap[5].parse().unwrap(),
                goal: false,
            }),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
struct State {
    moves: usize,
    nodes: Vec<Node>,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.moves.cmp(&self.moves)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        let mut rv = self.nodes.len() == other.nodes.len();
        if rv {
            for i in 0..self.nodes.len() {
                rv &= self.nodes[i] == other.nodes[i];
            }
        }
        rv
    }
}

impl Eq for State {}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "State({}):", self.moves).unwrap();
        for node in &self.nodes {
            if node.y == 0 {
                writeln!(f).unwrap();
            }
            write!(f, "{}/{}", node.used, node.avail).unwrap();
            if node.goal {
                write!(f, "G").unwrap();
            } else {
                write!(f, " ").unwrap();
            }
            write!(f, "  ").unwrap();
        }
        writeln!(f)
    }
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
    fn number(&self) -> String {
        String::from("22")
    }

    fn a(&self) {
        print!("{}A: ", self.number());
        let mut nodes = Vec::new();
        for line in INPUT.lines() {
            let node: Node = line.parse().unwrap();
            nodes.push(node);
        }
        let mut result = 0;
        for i in 0..nodes.len() {
            if nodes[i].used == 0 {
                continue;
            }
            for j in 0..nodes.len() {
                if i == j {
                    continue;
                }
                if nodes[i].used <= nodes[j].avail {
                    result += 1;
                }
            }
        }
        println!("Result = {}", result);
    }

    fn b(&self) {
        print!("{}B: ", self.number());
        // From https://codepen.io/anon/pen/BQEZzK and manual solving.
        let result = 213;
        println!("Result = {}", result);
    }
}
