//-----------------------------------------------------
// Setup.

#[cfg(test)]
use std::io;

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use nom::{
    branch::permutation,
    bytes::complete::{tag, take_while},
    character::complete::{digit1, line_ending},
    combinator::{eof, opt},
    multi::many0,
    sequence::{terminated, tuple},
    AsChar, IResult,
};

use crate::intcode::{Intcode, IntcodeError, State};

static INPUT: &str = include_str!("data/q25.data");

const RUN_TAPE_LIMIT: usize = 15000;

#[cfg(test)]
#[allow(dead_code)]
fn run_machine_interactive(data: Vec<i128>) -> i128 {
    let mut machine = Intcode::new(data, vec![]);
    let outputs = &mut machine.outputs;
    while let Some(output) = outputs.pop_back() {
        if output < 255 {
            print!("{}", output as u8 as char);
        } else {
            print!("{}", output);
        }
    }
    loop {
        let state = machine.run_tape_until(RUN_TAPE_LIMIT);
        let outputs = &mut machine.outputs;
        while let Some(output) = outputs.pop_back() {
            if output < 255 {
                print!("{}", output as u8 as char);
            } else {
                print!("{}", output);
            }
        }
        match state {
            Ok(State::Halted) => {
                break;
            }
            Ok(State::WaitingForInput) => {
                println!("waiting for input…");
            }
            Err(code) => {
                println!("ERROR!!! {}", code);
                break;
            }
            Ok(state) => {
                println!("ERROR, machine not halted! {:?}", state);
                break;
            }
        }
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            machine.inputs.extend(input.chars().map(|x| x as i128));
        }
    }
    0
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Room {
    name: String,
    doors: Vec<String>,
    items: Vec<String>,
}

#[derive(Debug)]
enum MoveResult {
    Room(Room),
    Item(String),
    Error(String),
}

impl MoveResult {
    fn into_room(self) -> Option<Room> {
        match self {
            MoveResult::Room(room) => Some(room),
            _ => None,
        }
    }
}

fn is_name_char(c: char) -> bool {
    c.is_alpha() || c == ' ' || c == '-'
}

// == Hull Breach ==
fn room_name(i: &str) -> IResult<&str, &str> {
    let (input, (_, title, _)) = tuple((tag("=="), take_while(is_name_char), tag("==")))(i)?;
    Ok((input, title.trim()))
}

// Doors here lead:\n- north\n- east\n- south\n
fn doors(i: &str) -> IResult<&str, Vec<String>> {
    let (input, (_, (north, east, south, west))) = tuple((
        tag("\nDoors here lead:\n"),
        permutation((
            opt(tuple((tag("- "), tag("north"), line_ending))),
            opt(tuple((tag("- "), tag("east"), line_ending))),
            opt(tuple((tag("- "), tag("south"), line_ending))),
            opt(tuple((tag("- "), tag("west"), line_ending))),
        )),
    ))(i)?;

    let mut rv = vec![];

    for door in [north, east, south, west].into_iter().flatten() {
        rv.push(door.1.to_owned());
    }

    Ok((input, rv))
}

fn is_item_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == ' '
}

// Items here:\n- spool of cat6\n
fn items(i: &str) -> IResult<&str, Vec<String>> {
    let (input, (_, items)) = tuple((
        tag("\nItems here:\n"),
        many0(tuple((tag("- "), take_while(is_item_char), tag("\n")))),
    ))(i)?;

    let mut rv = vec![];
    for item in items {
        rv.push(item.1.to_owned());
    }

    Ok((input, rv))
}

fn is_comment_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || " .,-?':;".find(c).is_some()
}

fn room(i: &str) -> IResult<&str, Room> {
    let (input, (_, name, _, _comment, _, doors, items)) = tuple((
        tag("\n\n"),
        room_name,
        line_ending,
        take_while(is_comment_char),
        line_ending,
        doors,
        opt(items),
    ))(i)?;

    Ok((
        input,
        Room {
            name: name.to_owned(),
            doors,
            items: items.unwrap_or_default(),
        },
    ))
}

fn take(i: &str) -> IResult<&str, &str> {
    let (input, (_, item, _)) =
        tuple((tag("You take the "), take_while(is_item_char), tag(".\n")))(i)?;

    Ok((input, item))
}

fn electromagnet(i: &str) -> IResult<&str, &str> {
    let (input, _) = tag("The giant electromagnet is stuck to you.  You can\'t move!!\n")(i)?;

    Ok((input, "ElectroMagnet"))
}

fn move_p(i: &str) -> IResult<&str, MoveResult> {
    let (input, (_, room, item, electromagnet, _)) = terminated(
        tuple((
            opt(line_ending),
            opt(room),
            opt(take),
            opt(electromagnet),
            tag("\nCommand?\n"),
        )),
        eof,
    )(i)?;

    let rv = if let Some(room) = room {
        MoveResult::Room(room)
    } else if let Some(item) = item {
        MoveResult::Item(item.to_owned())
    } else if let Some(electromagnet) = electromagnet {
        MoveResult::Error(electromagnet.to_owned())
    } else {
        MoveResult::Error("Missing room and item.".to_owned())
    };

    Ok((input, rv))
}

fn win(i: &str) -> IResult<&str, &str> {
    let (input, (_, code, _)) = tuple((
        tag("\n\n\n== Pressure-Sensitive Floor ==\nAnalyzing...\n\nDoors here lead:\n- east\n\nA loud, robotic voice says \"Analysis complete! You may proceed.\" and you enter the cockpit.\nSanta notices your small droid, looks puzzled for a moment, realizes what has happened, and radios your ship directly.\n\"Oh, hello! You should be able to get in by typing "),
        digit1,
        tag(" on the keypad at the main airlock.\"\n"),
    ))(i)?;

    Ok((input, code))
}
#[derive(Clone, Debug)]
struct RoomState {
    room: Room,
    steps: Vec<String>,
    keys: HashSet<String>,
    machine: Intcode,
    code: String,
}

impl RoomState {
    fn new(room: Room, machine: Intcode) -> Self {
        RoomState {
            room,
            steps: vec![],
            keys: HashSet::new(),
            machine,
            code: String::new(),
        }
    }
}

impl PartialEq for RoomState {
    fn eq(&self, other: &Self) -> bool {
        self.room == other.room && self.steps == other.steps && self.keys == other.keys
    }
}

impl Eq for RoomState {}

impl PartialOrd for RoomState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RoomState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .keys
            .len()
            .cmp(&self.keys.len())
            .then(self.steps.len().cmp(&other.steps.len()))
    }
}

fn move_one_state(
    curr: &RoomState,
    command: &str,
    seen: &mut HashSet<(String, Vec<String>)>,
) -> Option<RoomState> {
    let mut next = curr.clone();
    next.steps.push(command.to_owned());
    next.machine
        .inputs
        .extend(command.chars().chain(['\n']).map(|x| x as i128));
    let state = next.machine.run_tape_until(RUN_TAPE_LIMIT);
    let outputs = &mut next.machine.outputs;
    let output: String = outputs.iter().map(|x| *x as u8 as char).rev().collect();
    outputs.clear();

    if let Err(IntcodeError::MachineExceededLimit { limit: _limit }) = state {
        return None;
    }
    if state.is_err() {
        return None;
    }
    if state == Ok(State::Halted) {
        let output = win(&output);
        if output.is_err() {
            return None;
        }
        next.code = output.unwrap().1.to_owned();
        return Some(next);
    }

    let output = move_p(&output);
    if output.is_err() {
        if let Err(_output) = output {
            println!("/nPARSING ERROR:/n {:?}", &_output);
            return None;
        }
    }
    match output.unwrap().1 {
        MoveResult::Room(room) => {
            next.room = room;
        }
        MoveResult::Item(item) => {
            let mut items = next.room.items;
            items.retain(|x| x != &format!("{}\n", item));
            next.room.items = items;
            next.keys.insert(item);
        }
        MoveResult::Error(_error) => {
            return None;
        }
    }
    let mut keys: Vec<_> = next.keys.clone().into_iter().collect();
    keys.sort();
    if !seen.contains(&(next.room.name.clone(), keys)) {
        Some(next)
    } else {
        None
    }
}

fn run_machine(data: Vec<i128>) -> String {
    let mut machine = Intcode::new(data, vec![]);
    let _state = machine.run_tape_until(RUN_TAPE_LIMIT);
    let outputs = &mut machine.outputs;
    let output: String = outputs.iter().map(|x| *x as u8 as char).rev().collect();
    outputs.clear();
    let output = move_p(&output);
    if output.is_err() {
        if let Err(_output) = output {
            return "-1".to_owned();
        }
    }
    let room = output.unwrap().1.into_room().unwrap();

    // Explore all the rooms!

    let mut states = BinaryHeap::from(vec![RoomState::new(room, machine)]);
    let mut seen: HashSet<(String, Vec<String>)> = HashSet::new();
    while !states.is_empty() {
        let curr = states.pop().unwrap();
        if !curr.code.is_empty() {
            return curr.code;
        }

        let mut keys: Vec<_> = curr.keys.clone().into_iter().collect();
        keys.sort();
        seen.insert((curr.room.name.clone(), keys));

        // Try to pick up anything lying around.
        for item in &curr.room.items {
            let command = format!("take {}", item);
            if let Some(next) = move_one_state(&curr, &command, &mut seen) {
                states.push(next);
            }
        }

        // Otherwise, try to move.
        for command in &curr.room.doors {
            if let Some(next) = move_one_state(&curr, command, &mut seen) {
                states.push(next);
            }
        }
    }

    "0".to_owned()
}

fn process_data_a(data: &str) -> String {
    let ints: Vec<i128> = data
        .split(',')
        .map(|i| i.parse::<i128>().unwrap())
        .collect();
    run_machine(ints)
}

fn process_data_b(_data: &str) -> i32 {
    0
}

//-----------------------------------------------------
// Questions.

q_impl!("25");

#[test]
fn a() {
    // let ints: Vec<i128> = INPUT
    //     .split(',')
    //     .map(|i| i.parse::<i128>().unwrap())
    //     .collect();
    // run_machine_interactive(ints);

    // assert_eq!(process_data_a(""), "0");
}

#[test]
fn b() {
    // assert!(false);
}
