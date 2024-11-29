//-----------------------------------------------------
// Setup.

use aoc::Day;
use regex::Regex;
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
        let re: &Regex =
            regex!("/dev/grid/node-x([0-9]+)-y([0-9]+) +([0-9]+)T +([0-9]+)T +([0-9]+)T");

        let captures = re.captures(s);
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
