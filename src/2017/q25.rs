//-----------------------------------------------------
// Setup.

use aoc::Day;

use std::collections::HashMap;
use std::collections::HashSet;
use std::str;

use glue::prelude::{alphabetic, any, digit, find, find_all, find_any, is, take, Parser};
use glue::types::MapParserResult;

static INPUT: &str = include_str!("data/q25.data");

#[derive(Clone, Debug, Eq, PartialEq)]
struct Action {
    test: bool,
    write: bool,
    direction: i32,
    next: char,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    name: char,
    actions: HashMap<bool, Action>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Machine {
    tape: HashSet<i32>,
    position: i32,
    state: char,
    checksum: usize,
    steps: usize,
    states: HashMap<char, State>,
}

impl Machine {
    fn step(&mut self) {
        self.steps += 1;
        let value = self.tape.contains(&self.position);
        let action = &self.states[&self.state].actions[&value];
        // println!("{} ({}@{}) : {:?}", self.steps, value, self.position, action);

        if action.write {
            self.tape.insert(self.position);
        } else {
            self.tape.remove(&self.position);
        }
        self.position += action.direction;
        self.state = action.next;
    }
}

fn machine_name_parser<'a>() -> impl Parser<'a, char> {
    move |ctx| {
        find_all((is("Begin in state "), is(alphabetic), is(".\n")))
            .parse(ctx)
            .map_result(|(_, state, _)| state.chars().next().unwrap())
    }
}

fn machine_checksum_parser<'a>() -> impl Parser<'a, usize> {
    move |ctx| {
        find_all((
            is("Perform a diagnostic checksum after "),
            take(1.., is(digit)),
            is(" steps.\n"),
        ))
        .parse(ctx)
        .map_result(|(_, number, _)| number.parse().unwrap())
    }
}

fn state_name_parser<'a>() -> impl Parser<'a, char> {
    move |ctx| {
        find_all((is("In state "), is(any), is(":\n")))
            .parse(ctx)
            .map_result(|(_, name, _)| name.chars().next().unwrap())
    }
}

fn action_test_parser<'a>() -> impl Parser<'a, bool> {
    move |ctx| {
        find_all((
            is("  If the current value is "),
            take(1.., is(digit)),
            is(":\n"),
        ))
        .parse(ctx)
        .map_result(|(_, number, _)| number == "1")
    }
}

fn action_write_parser<'a>() -> impl Parser<'a, bool> {
    move |ctx| {
        find_all((
            is("    - Write the value "),
            take(1.., is(digit)),
            is(".\n"),
        ))
        .parse(ctx)
        .map_result(|(_, number, _)| number == "1")
    }
}

fn action_move_parser<'a>() -> impl Parser<'a, i32> {
    move |ctx| {
        find_all((
            is("    - Move one slot to the "),
            find_any((is("left"), is("right"))),
            is(".\n"),
        ))
        .parse(ctx)
        .map_result(|(_, found, _)| if found == "left" { -1 } else { 1 })
    }
}

fn action_next_parser<'a>() -> impl Parser<'a, char> {
    move |ctx| {
        find_all((is("    - Continue with state "), is(any), is(".\n")))
            .parse(ctx)
            .map_result(|(_, state, _)| state.chars().next().unwrap())
    }
}

fn action_parser<'a>() -> impl Parser<'a, Action> {
    move |ctx| {
        find_all((
            action_test_parser(),
            action_write_parser(),
            action_move_parser(),
            action_next_parser(),
        ))
        .parse(ctx)
        .map_result(|(test, write, direction, next)| Action {
            test,
            write,
            direction,
            next,
        })
    }
}

fn state_parser<'a>() -> impl Parser<'a, State> {
    move |ctx| {
        find_all((is("\n"), state_name_parser(), find(1.., action_parser())))
            .parse(ctx)
            .map_result(|(_, name, actions)| State {
                name,
                actions: actions.iter().cloned().map(|x| (x.test, x)).collect(),
            })
    }
}

fn machine_parser<'a>() -> impl Parser<'a, Machine> {
    move |ctx| {
        find_all((
            machine_name_parser(),
            machine_checksum_parser(),
            find(1.., state_parser()),
        ))
        .parse(ctx)
        .map_result(|(state, checksum, states)| Machine {
            tape: HashSet::new(),
            position: 0,
            state,
            checksum,
            steps: 0,
            states: states.iter().cloned().map(|x| (x.name, x)).collect(),
        })
    }
}

fn process_data_a(data: &str) -> usize {
    let mut machine = machine_parser().parse(data).unwrap().1;
    while machine.steps < machine.checksum {
        machine.step();
    }
    // println!("{:?}\nReturning: {}", machine, machine.tape.len());
    machine.tape.len()
}

fn process_data_b(_data: &str) -> i32 {
    0
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
    fn number(&self) -> String {
        String::from("25")
    }

    fn a(&self) {
        print!("{}A: ", self.number());
        let result = process_data_a(INPUT);
        println!("Result = {}", result);
    }

    fn b(&self) {
        print!("{}B: ", self.number());
        let result = process_data_b(INPUT);
        println!("Result = {}", result);
    }
}

#[test]
fn a() {
    let action_a0 = Action {
        test: false,
        write: true,
        direction: 1,
        next: 'B',
    };

    let action_a1 = Action {
        test: true,
        write: false,
        direction: -1,
        next: 'B',
    };

    let state_a = State {
        name: 'A',
        actions: hashmap![
          false => action_a0.clone(),
          true => action_a1.clone()
        ],
    };

    let action_b0 = Action {
        test: false,
        write: true,
        direction: -1,
        next: 'A',
    };

    let action_b1 = Action {
        test: true,
        write: true,
        direction: 1,
        next: 'A',
    };

    let state_b = State {
        name: 'B',
        actions: hashmap![
          false => action_b0.clone(),
          true => action_b1.clone()
        ],
    };

    let machine = Machine {
        tape: HashSet::new(),
        position: 0,
        state: 'A',
        checksum: 6,
        steps: 0,
        states: hashmap![
          'A' => state_a.clone(),
          'B' => state_b.clone()
        ],
    };

    assert_eq!(
        process_data_a(
            "Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
"
        ),
        3
    );
}

#[test]
fn b() {
    assert_eq!(process_data_b(""), 0);
}
