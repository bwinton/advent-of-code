//-----------------------------------------------------
// Setup.

use std::collections::hash_map::Entry;
use std::collections::HashMap;

static INPUT: &'static str = include_str!("data/q03.data");

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct House {
    x: i32,
    y: i32,
}

impl House {
    pub fn new(x: i32, y: i32) -> House {
        House { x, y }
    }
}

fn next_state(entry: &Entry<House, u32>, character: char) -> House {
    let mut rv = *entry.key();
    match character {
        '^' => rv.y -= 1,
        '>' => rv.x += 1,
        'v' => rv.y += 1,
        '<' => rv.x -= 1,
        _ => panic!("Unknown char! \"{}\"", character),
    }
    rv
}

fn process_data_a(data: &str) -> u32 {
    let mut state = House::new(0, 0);
    let mut board = HashMap::new();
    *board.entry(state).or_insert(0) += 1;
    for character in data.chars() {
        // println!("{:?}, {}", state, character);
        state = next_state(&board.entry(state), character);
        *board.entry(state).or_insert(0) += 1;
    }
    // println!("{:?}", board);
    board.len() as u32
}

fn process_data_b(data: &str) -> u32 {
    let mut santa = House::new(0, 0);
    let mut robot = House::new(0, 0);
    let mut board = HashMap::new();
    *board.entry(santa).or_insert(0) += 1;
    *board.entry(robot).or_insert(0) += 1;
    let mut curr = santa;
    for character in data.chars() {
        // println!("{:?}, {}", curr, character);
        let mut next = next_state(&board.entry(curr), character);
        *board.entry(next).or_insert(0) += 1;
        if curr == santa {
            santa = next;
            curr = robot;
        } else {
            robot = next;
            curr = santa;
        }
    }
    // println!("{:?}", board);
    board.len() as u32
}

//-----------------------------------------------------
// Questions.

q_impl!("3");

#[test]
fn a() {
    assert_eq!(process_data_a(">"), 2);
    assert_eq!(process_data_a("^>v<"), 4);
    assert_eq!(process_data_a("^v^v^v^v^v"), 2);
}

#[test]
fn b() {
    assert_eq!(process_data_b("^v"), 3);
    assert_eq!(process_data_b("^>v<"), 3);
    assert_eq!(process_data_b("^v^v^v^v^v"), 11);
}
