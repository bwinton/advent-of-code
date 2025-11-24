//-----------------------------------------------------
// Setup.

use aoc::Day;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, u64},
    multi::many1,
};

use std::{
    collections::{HashMap, HashSet},
    str,
};

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

fn machine_name(i: &str) -> IResult<&str, char> {
    let (input, (_, state, _)) = (tag("Begin in state "), alpha1, tag(".\n")).parse(i)?;
    Ok((input, state.chars().next().unwrap()))
}

fn machine_checksum(i: &str) -> IResult<&str, usize> {
    let (input, (_, number, _)) = (
        tag("Perform a diagnostic checksum after "),
        u64,
        tag(" steps.\n"),
    )
        .parse(i)?;
    Ok((input, number as usize))
}

fn state_name(i: &str) -> IResult<&str, char> {
    let (input, (_, state, _)) = (tag("In state "), alpha1, tag(":\n")).parse(i)?;
    Ok((input, state.chars().next().unwrap()))
}

fn action_test(i: &str) -> IResult<&str, bool> {
    let (input, (_, number, _)) = (tag("  If the current value is "), u64, tag(":\n")).parse(i)?;
    Ok((input, number == 1))
}

fn action_write(i: &str) -> IResult<&str, bool> {
    let (input, (_, number, _)) = (tag("    - Write the value "), u64, tag(".\n")).parse(i)?;
    Ok((input, number == 1))
}

fn action_move(i: &str) -> IResult<&str, i32> {
    let (input, (_, found, _)) = (
        tag("    - Move one slot to the "),
        alt((tag("left"), tag("right"))),
        tag(".\n"),
    )
        .parse(i)?;
    Ok((input, if found == "left" { -1 } else { 1 }))
}

fn action_next(i: &str) -> IResult<&str, char> {
    let (input, (_, state, _)) =
        (tag("    - Continue with state "), alpha1, tag(".\n")).parse(i)?;
    Ok((input, state.chars().next().unwrap()))
}

fn action(i: &str) -> IResult<&str, Action> {
    let (input, (test, write, direction, next)) =
        (action_test, action_write, action_move, action_next).parse(i)?;
    Ok((
        input,
        Action {
            test,
            write,
            direction,
            next,
        },
    ))
}

fn state(i: &str) -> IResult<&str, State> {
    let (input, (_, name, actions)) = (line_ending, state_name, many1(action)).parse(i)?;
    Ok((
        input,
        State {
            name,
            actions: actions.iter().cloned().map(|x| (x.test, x)).collect(),
        },
    ))
}

fn machine(i: &str) -> IResult<&str, Machine> {
    let (input, (state, checksum, states)) =
        (machine_name, machine_checksum, many1(state)).parse(i)?;
    Ok((
        input,
        Machine {
            tape: HashSet::new(),
            position: 0,
            state,
            checksum,
            steps: 0,
            states: states.iter().cloned().map(|x| (x.name, x)).collect(),
        },
    ))
}

fn process_data_a(data: &str) -> usize {
    let mut machine = machine(data).unwrap().1;
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
    // let action_a0 = Action {
    //     test: false,
    //     write: true,
    //     direction: 1,
    //     next: 'B',
    // };

    // let action_a1 = Action {
    //     test: true,
    //     write: false,
    //     direction: -1,
    //     next: 'B',
    // };

    // let state_a = State {
    //     name: 'A',
    //     actions: hashmap![
    //       false => action_a0.clone(),
    //       true => action_a1.clone()
    //     ],
    // };

    // let action_b0 = Action {
    //     test: false,
    //     write: true,
    //     direction: -1,
    //     next: 'A',
    // };

    // let action_b1 = Action {
    //     test: true,
    //     write: true,
    //     direction: 1,
    //     next: 'A',
    // };

    // let state_b = State {
    //     name: 'B',
    //     actions: hashmap![
    //       false => action_b0.clone(),
    //       true => action_b1.clone()
    //     ],
    // };

    // let machine = Machine {
    //     tape: HashSet::new(),
    //     position: 0,
    //     state: 'A',
    //     checksum: 6,
    //     steps: 0,
    //     states: hashmap![
    //       'A' => state_a.clone(),
    //       'B' => state_b.clone()
    //     ],
    // };

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
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b(""), 0);
}
