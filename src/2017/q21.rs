//-----------------------------------------------------
// Setup.

use aoc::Day;

use std::collections::HashSet;
use std::str::FromStr;

static INPUT: &str = include_str!("data/q21.data");

#[derive(Debug, Eq, Clone, Hash, PartialEq)]
struct Rule {
    size: usize,
    input: Vec<Vec<bool>>,
    output: Vec<Vec<bool>>,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Rule, ()> {
        let parts: Vec<_> = s.split(" => ").collect();
        let input = parts[0];
        Ok(Rule {
            size: if input.len() == 5 { 2 } else { 3 },
            input: get_state(input),
            output: get_state(parts[1]),
        })
    }
}

impl Rule {
    fn matches(&self, input: &[&[bool]]) -> Option<Vec<Vec<bool>>> {
        for i in self.input.iter().enumerate() {
            for j in i.1.iter().enumerate() {
                if input[i.0][j.0] != *j.1 {
                    return None;
                }
            }
        }
        Some(self.output.clone())
    }

    fn symmetric(&self) -> Rule {
        let mut input = self.input.clone();
        for (x, row) in self.input.iter().enumerate() {
            for cell in 0..row.len() {
                input[x][cell] = self.input[cell][x];
            }
        }
        Rule {
            size: self.size,
            input,
            output: self.output.clone(),
        }
    }

    fn flip(&self) -> Rule {
        let mut input = Vec::new();
        for row in &self.input {
            input.insert(0, row.clone());
        }
        Rule {
            size: self.size,
            input,
            output: self.output.clone(),
        }
    }
}

fn get_state(input: &str) -> Vec<Vec<bool>> {
    input
        .split('/')
        .map(|row| row.chars().map(|col| col == '#').collect())
        .collect()
}

// fn print_board(state: &[Vec<bool>]) {
//   for row in state {
//     let data: Vec<&str> = row.iter().map(|x| if *x {"#"} else {"."}).collect();
//     println!("{:?}", data.join(""));
//   }
// }

fn get_next<I: Iterator>(input: &mut [I]) -> Option<Vec<I::Item>> {
    let mut rv = Vec::new();
    for iter in input {
        if let Some(value) = iter.next() {
            rv.push(value);
        } else {
            return None;
        }
    }
    Some(rv)
}

fn step(state: &[Vec<bool>], rules: &[Rule]) -> Vec<Vec<bool>> {
    let mut pieces = Vec::new();
    let size = if state.len() % 2 == 0 { 2 } else { 3 };
    let active_rules: Vec<_> = rules.iter().filter(|r| r.size == size).collect();

    for rows in state.chunks(size) {
        let mut new_row = Vec::new();
        for _ in 0..=size {
            new_row.push(Vec::new());
        }
        let mut cols: Vec<_> = rows.iter().map(|row| row.chunks(size)).collect();

        while let Some(piece) = get_next(&mut cols) {
            for rule in &active_rules {
                if let Some(new_piece) = rule.matches(&piece) {
                    for i in 0..new_piece.len() {
                        new_row[i].extend(new_piece[i].clone());
                    }
                    break;
                }
            }
        }

        pieces.extend(new_row);
    }

    pieces
}

fn process_data(data: &str, iterations: usize) -> usize {
    let mut rules: Vec<Rule> = data.lines().map(|line| line.parse().unwrap()).collect();
    let mut all_rules = HashSet::new();
    for rule in rules {
        let mut transform = rule.clone();
        for _ in 0..4 {
            all_rules.insert(transform.clone());
            transform = transform.symmetric();
            all_rules.insert(transform.clone());
            transform = transform.flip();
        }
    }
    rules = all_rules.into_iter().collect();
    let mut state = get_state(".#./..#/###");
    for _i in 0..iterations {
        state = step(&state, &rules);
    }
    // print_board(&state);
    state
        .iter()
        .map(|row| {
            let new_row: Vec<_> = row.iter().filter(|&x| *x).collect();
            new_row.len()
        })
        .sum()
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
    fn number(&self) -> String {
        String::from("21")
    }

    fn a(&self) {
        print!("{}A: ", self.number());
        let result = process_data(INPUT, 5);
        println!("Result = {}", result);
    }

    fn b(&self) {
        print!("{}B: ", self.number());
        let result = process_data(INPUT, 18);
        println!("Result = {}", result);
    }
}

#[test]
fn a() {
    assert_eq!(
        process_data(
            "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#",
            2,
        ),
        12
    );
}

#[test]
fn b() {}
