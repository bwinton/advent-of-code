//-----------------------------------------------------
// Setup.

use aoc::Day;

use nom;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str;
use std::str::FromStr;

static INPUT : &'static str = "Begin in state A.
Perform a diagnostic checksum after 12656374 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state C.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state D.

In state C:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state D.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the right.
    - Continue with state C.

In state D:
  If the current value is 0:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the right.
    - Continue with state E.

In state E:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state C.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state F.

In state F:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state E.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
";

#[derive(Clone)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
struct Action {
  test: bool,
  write: bool,
  direction: i32,
  next: char
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
struct State {
  name: char,
  actions: HashMap<bool, Action>
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
struct Machine {
  tape: HashSet<i32>,
  position: i32,
  state: char,
  checksum: usize,
  steps: usize,
  states: HashMap<char, State>
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

named!(digits<usize>, map_res!(
  map_res!(
    nom::digit,
    str::from_utf8
  ),
  FromStr::from_str
));

named!(machine_name_parser<char>, do_parse!(
  tag!("Begin in state ") >>
  name: take!(1) >>
  tag!(".\n") >>
  (name[0] as char)
));

named!(machine_checksum_parser<usize>, do_parse!(
  tag!("Perform a diagnostic checksum after ") >>
  number: digits >>
  tag!(" steps.\n") >>
  (number)
));

named!(state_name_parser<char>, do_parse!(
  tag!("In state ") >>
  name: take!(1) >>
  tag!(":\n") >>
  (name[0] as char)
));

named!(action_test_parser<bool>, do_parse!(
  tag!("  If the current value is ") >>
  number: digits >>
  tag!(":\n") >>
  (number == 1)
));

named!(action_write_parser<bool>, do_parse!(
  tag!("    - Write the value ") >>
  number: digits >>
  tag!(".\n") >>
  (number == 1)
));

named!(direction<i32>, alt!(
  tag!("left") => {|_| -1} |
  tag!("right") => {|_| 1}
));

named!(action_move_parser<i32>, do_parse!(
  tag!("    - Move one slot to the ") >>
  number: direction >>
  tag!(".\n") >>
  (number)
));

named!(action_next_parser<char>, do_parse!(
  tag!("    - Continue with state ") >>
  name: take!(1) >>
  tag!(".\n") >>
  (name[0] as char)
));

named!(action_parser<Action>, do_parse!(
  test: action_test_parser >>
  write: action_write_parser >>
  direction: action_move_parser >>
  next: action_next_parser >>
  (Action {
    test: test,
    write: write,
    direction: direction,
    next: next
  })
));

named!(state_parser<State>, do_parse!(
  tag!("\n") >>
  name: state_name_parser >>
  actions: many1!(action_parser) >>
  (State {
    name: name,
    actions: actions.iter().cloned().map(|x| (x.test, x)).collect()
  })
));

named!(machine_parser<Machine>, complete!(do_parse!(
  name: machine_name_parser >>
  checksum: machine_checksum_parser >>
  states: many1!(state_parser) >>
  (Machine {
    tape: HashSet::new(),
    position: 0,
    state: name,
    checksum: checksum,
    steps: 0,
    states: states.iter().cloned().map(|x| (x.name, x)).collect()
  }))
));



fn process_data_a(data: &str) -> usize {
  let mut machine = machine_parser(data.as_bytes()).unwrap().1;
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
  assert_eq!(machine_name_parser("Begin in state A.\n".as_bytes()).unwrap().1, 'A');
  assert_eq!(machine_checksum_parser("Perform a diagnostic checksum after 6 steps.\n".as_bytes()).unwrap().1, 6);
  assert_eq!(machine_checksum_parser("Perform a diagnostic checksum after 12656374 steps.\n".as_bytes()).unwrap().1, 12656374);
  assert_eq!(state_name_parser("In state A:\n".as_bytes()).unwrap().1, 'A');
  assert_eq!(action_test_parser("  If the current value is 0:\n".as_bytes()).unwrap().1, false);
  assert_eq!(action_test_parser("  If the current value is 1:\n".as_bytes()).unwrap().1, true);
  assert_eq!(action_write_parser("    - Write the value 0.\n".as_bytes()).unwrap().1, false);
  assert_eq!(action_write_parser("    - Write the value 1.\n".as_bytes()).unwrap().1, true);
  assert_eq!(action_move_parser("    - Move one slot to the left.\n".as_bytes()).unwrap().1, -1);
  assert_eq!(action_move_parser("    - Move one slot to the right.\n".as_bytes()).unwrap().1, 1);
  assert_eq!(action_next_parser("    - Continue with state A.\n".as_bytes()).unwrap().1, 'A');
  assert_eq!(action_next_parser("    - Continue with state B.\n".as_bytes()).unwrap().1, 'B');

  let action_a0 = Action {
    test: false,
    write: true,
    direction: 1,
    next: 'B'
  };

  let action_a1 = Action {
    test: true,
    write: false,
    direction: -1,
    next: 'B'
  };

  let state_a = State {
    name: 'A',
    actions: hashmap!{
      false => action_a0.clone(),
      true => action_a1.clone()
    }
  };

  let action_b0 = Action {
    test: false,
    write: true,
    direction: -1,
    next: 'A'
  };

  let action_b1 = Action {
    test: true,
    write: true,
    direction: 1,
    next: 'A'
  };

  let state_b = State {
    name: 'B',
    actions: hashmap!{
      false => action_b0.clone(),
      true => action_b1.clone()
    }
  };

  let machine = Machine {
    tape: HashSet::new(),
    position: 0,
    state: 'A',
    checksum: 6,
    steps: 0,
    states: hashmap!{
      'A' => state_a.clone(),
      'B' => state_b.clone()
    }
  };

  assert_eq!(action_parser("  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
".as_bytes()).unwrap().1, action_a0);
  assert_eq!(action_parser("  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.
".as_bytes()).unwrap().1, action_a1);
  assert_eq!(action_parser("  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
".as_bytes()).unwrap().1, action_b0);
  assert_eq!(action_parser("  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
".as_bytes()).unwrap().1, action_b1);

  assert_eq!(state_parser("
In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.
".as_bytes()).unwrap().1, state_a);

  assert_eq!(state_parser("
In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
".as_bytes()).unwrap().1, state_b);

  assert_eq!(machine_parser("Begin in state A.
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
".as_bytes()).unwrap().1, machine);

  assert_eq!(process_data_a("Begin in state A.
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
"), 3);
}

#[test]
fn b() {
  assert_eq!(process_data_b(""), 0);
}