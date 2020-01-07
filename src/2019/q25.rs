//-----------------------------------------------------
// Setup.

#[cfg(test)]
use std::io;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

use crate::intcode::{Intcode, IntcodeError, State};

// use itertools::Itertools;
use glue::prelude::{
    alphabetic, alphanumeric, eoi, find, find_all, is, numeric, one_of, optional, take, take_all,
    take_any, Parser,
};
use glue::types::MapParserResult;

static INPUT: &str = include_str!("data/q25.data");

const RUN_TAPE_LIMIT: usize = 15000;

#[cfg(test)]
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
            // println!("{:?}", &input);
            machine.inputs.extend(input.chars().map(|x| x as i128));
        }
    }
    0
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Room {
    name: String,
    doors: Vec<String>,
    items: Option<Vec<String>>,
}

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

    // fn as_item(self) -> Option<String> {
    //     match self {
    //         MoveResult::Item(item) => Some(item),
    //         _ => None
    //     }
    // }
}

// == Hull Breach ==
fn room_name_parser<'a>() -> impl Parser<'a, String> {
    move |ctx| {
        find_all((
            is("=="),
            take(1.., take_any((is(alphabetic), is(one_of(" -"))))),
            is("=="),
        ))
        .parse(ctx)
        .map_result(|(_, title, _)| title.trim().to_owned())
    }
}

// Doors here lead:\n- north\n- east\n- south\n
fn doors_parser<'a>() -> impl Parser<'a, Vec<String>> {
    move |ctx| {
        find_all((
            is("\nDoors here lead:\n"),
            find(
                0..,
                take_any((
                    is("- north\n"),
                    is("- east\n"),
                    is("- south\n"),
                    is("- west\n"),
                )),
            ),
        ))
        .parse(ctx)
        .map_result(|(_, doors)| {
            let mut rv = vec![];
            for door in doors {
                rv.push(door[2..].to_owned());
            }
            rv
        })
    }
}

// Items here:\n- spool of cat6\n
fn items_parser<'a>() -> impl Parser<'a, Vec<String>> {
    move |ctx| {
        find_all((
            is("\nItems here:\n"),
            find(
                0..,
                take_all((
                    is("- "),
                    take(0.., take_any((is(alphanumeric), is(one_of(" ."))))),
                    is('\n'),
                )),
            ),
        ))
        .parse(ctx)
        .map_result(|(_, items)| {
            let mut rv = vec![];
            for item in items {
                rv.push(item[2..].to_owned());
            }
            rv
        })
    }
}

fn room_parser<'a>() -> impl Parser<'a, Room> {
    move |ctx| {
        find_all((
            is("\n\n"),
            room_name_parser(),
            is('\n'),
            take(1.., take_any((is(alphanumeric), is(one_of(" .,-?':;"))))),
            is("\n"),
            doors_parser(),
            optional(items_parser()),
        ))
        .parse(ctx)
        .map_result(|(_, name, _, _comment, _, doors, items)| {
            // println!("Room : {}\n  {}\n  {:?}\n  {:?}", name, _comment, doors, items);
            Room { name, doors, items }
        })
    }
}

fn take_parser<'a>() -> impl Parser<'a, String> {
    move |ctx| {
        find_all((
            is("You take the "),
            take(0.., take_any((is(alphanumeric), is(one_of(" "))))),
            is(".\n"),
        ))
        .parse(ctx)
        .map_result(|(_, item, _)| item.to_owned())
    }
}

fn electromagnet_parser<'a>() -> impl Parser<'a, String> {
    move |ctx| {
        { is("The giant electromagnet is stuck to you.  You can\'t move!!\n") }
            .parse(ctx)
            .map_result(|_| "ElectroMagnet".to_owned())
    }
}

fn move_parser<'a>() -> impl Parser<'a, MoveResult> {
    move |ctx| {
        find_all((
            is("\n"),
            optional(room_parser()),
            optional(take_parser()),
            optional(electromagnet_parser()),
            is("\nCommand?\n"),
            eoi(),
        ))
        .parse(ctx)
        .map_result(|(_, room, item, electromagnet, _, _)| {
            if let Some(room) = room {
                MoveResult::Room(room)
            } else if let Some(item) = item {
                MoveResult::Item(item)
            } else if let Some(electromagnet) = electromagnet {
                MoveResult::Error(electromagnet)
            } else {
                MoveResult::Error("Missing room and item.".to_owned())
            }
        })
    }
}

fn win_parser<'a>() -> impl Parser<'a, String> {
    move |ctx| {
        find_all((
            is("\n\n\n== Pressure-Sensitive Floor ==\nAnalyzing...\n\nDoors here lead:\n- east\n\nA loud, robotic voice says \"Analysis complete! You may proceed.\" and you enter the cockpit.\nSanta notices your small droid, looks puzzled for a moment, realizes what has happened, and radios your ship directly.\n\"Oh, hello! You should be able to get in by typing "),
            take(0.., is(numeric)),
            is(" on the keypad at the main airlock.\"\n")
        )).parse(ctx)
        .map_result(|(_, code, _)| { code.to_owned() })
    }
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
        .extend(command.chars().map(|x| x as i128));
    // if curr.room.name == END_STATE_NAME {
    //     println!("Move {}: {} -> {:?}, {:?}", curr.steps.len(), curr.room.name, command, curr.keys);
    // }
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
        let output = win_parser().parse(&output);
        if output.is_err() {
            return None;
        }
        next.code = output.unwrap().1;
        return Some(next);
    }

    let output = move_parser().parse(&output);
    if output.is_err() {
        if let Err(_output) = output {
            // if command != "take giant electromagnet\n" {
            //     println!("  ERROR: {} -> {}, {:?} {:?}", curr.room.name, command, curr.keys, _output.0.bounds);
            //     println!("       : {:?}", &_output.0.input);
            //     println!("       :  {}{}", " ".repeat(_output.0.bounds.start), "^".repeat(_output.0.bounds.len()));
            // }
            return None;
        }
    }
    match output.unwrap().1 {
        MoveResult::Room(room) => {
            next.room = room;
        }
        MoveResult::Item(item) => {
            let mut items = next.room.items.unwrap();
            items.retain(|x| x != &format!("{}\n", item));
            next.room.items = Some(items);
            next.keys.insert(item);
        }
        MoveResult::Error(_error) => {
            // println!("ERROR!!! {}", error);
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
    // println!("Initial state: {:?}", state);
    let outputs = &mut machine.outputs;
    let output: String = outputs.iter().map(|x| *x as u8 as char).rev().collect();
    outputs.clear();
    let output = move_parser().parse(&output);
    if output.is_err() {
        if let Err(_output) = output {
            // println!("  ERROR: {:?}", &output.0.input);
            // println!("       :  {}{}", " ".repeat(output.0.bounds.start), "^".repeat(output.0.bounds.len()));
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
        if let Some(items) = &curr.room.items.clone() {
            for item in items {
                let command = format!("take {}", item);
                if let Some(next) = move_one_state(&curr, &command, &mut seen) {
                    // println!("Move {}: {} from {} to {}\n", next.steps.len(), command, curr.room.name, next.room.name);
                    states.push(next);
                }
            }
        }

        // Otherwise, try to move.
        for command in &curr.room.doors {
            if let Some(next) = move_one_state(&curr, command, &mut seen) {
                // println!("Move {}: {} from {} to {}\n", next.steps.len(), command, curr.room.name, next.room.name);
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
    let ints: Vec<i128> = INPUT
        .split(',')
        .map(|i| i.parse::<i128>().unwrap())
        .collect();
    run_machine_interactive(ints);

    assert_eq!(process_data_a(""), "0");
}

#[test]
fn b() {
    let result = is("\n\n").parse("\n\n");
    println!("{:?}", result);
    let result = room_name_parser().parse("== Engineering ==");
    println!("{:?}", result);
    let result = is('\n').parse("\n");
    println!("{:?}", result);
    let result = take(1.., take_any((is(alphanumeric), is(one_of(" .,-?':;")))))
        .parse("You see a whiteboard with plans for Springdroid v2.");
    println!("{:?}", result);
    let result = is('\n').parse("\n");
    println!("{:?}", result);
    let result = doors_parser().parse("\nDoors here lead:\n- north\n- east\n- west\n");
    println!("{:?}", result);
    let result = optional(items_parser()).parse("\nItems here:\n- ornament\n");
    println!("{:?}", result);

    let result = room_parser().parse("\n\n== Engineering ==\nYou see a whiteboard with plans for Springdroid v2.\n\nDoors here lead:\n- north\n- east\n- west\n\nItems here:\n- ornament\n");
    println!("{:?}", result);
    if let Err(error) = result {
        println!("Error: {}", &error.0.input[error.0.bounds]);
    }
    assert!(false);
}
