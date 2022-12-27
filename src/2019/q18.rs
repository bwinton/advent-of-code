//-----------------------------------------------------
// Setup.

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    hash::{Hash, Hasher},
};

static INPUT: &str = include_str!("data/q18.data");

#[derive(PartialEq, Eq)]
enum Cell {
    Open,
    Entrance,
    Wall,
    Door(char),
    Key(char),
}

impl Cell {
    fn parse(cell: char) -> Option<Self> {
        match cell {
            '.' => Some(Cell::Open),
            '#' => Some(Cell::Wall),
            '@' => Some(Cell::Entrance),
            x if ('a'..='z').contains(&x) => Some(Cell::Key(x.to_ascii_uppercase())),
            x if ('A'..='Z').contains(&x) => Some(Cell::Door(x)),
            _ => None,
        }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

type Position = (usize, usize);
type Positions = Vec<Position>;

#[derive(Clone, Debug)]
struct State {
    steps: usize,
    position: Positions,
    keys: HashSet<char>,
}

impl State {
    fn new(position: Positions) -> Self {
        State {
            steps: 0,
            position,
            keys: HashSet::new(),
        }
    }
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        self.steps == other.steps && self.position == other.position && self.keys == other.keys
    }
}

impl Eq for State {}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.steps.hash(state);
        self.position.hash(state);
        let keys: Vec<_> = self.keys.iter().collect();
        keys.hash(state);
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        let ordering = other.steps.cmp(&self.steps);
        if ordering != Ordering::Equal {
            return ordering;
        }
        self.keys.len().cmp(&other.keys.len())
    }
}

fn move_one_state(
    curr: &State,
    direction: Direction,
    board: &HashMap<Position, Cell>,
    seen: &mut HashSet<(Position, Vec<char>)>,
    states: &mut BinaryHeap<State>,
) {
    let mut next = curr.clone();
    next.steps += 1;
    match direction {
        Direction::Up => {
            next.position[0].1 -= 1;
        }
        Direction::Down => {
            next.position[0].1 += 1;
        }
        Direction::Left => {
            next.position[0].0 -= 1;
        }
        Direction::Right => {
            next.position[0].0 += 1;
        }
    }
    let mut keys: Vec<char> = next.keys.clone().into_iter().collect();
    keys.sort_unstable();
    let mut test_keys = keys.clone();
    match board[&next.position[0]] {
        Cell::Open | Cell::Entrance => {
            if !seen.contains(&(next.position[0], test_keys)) {
                seen.insert((next.position[0], keys));
                states.push(next);
            }
        }
        Cell::Door(x) if next.keys.contains(&x) => {
            if !seen.contains(&(next.position[0], test_keys)) {
                seen.insert((next.position[0], keys));
                states.push(next);
            }
        }
        Cell::Key(x) => {
            next.keys.insert(x);
            test_keys.push(x);
            keys.push(x);
            keys.sort_unstable();
            if !seen.contains(&(next.position[0], test_keys)) {
                seen.insert((next.position[0], keys));
                states.push(next);
            }
        }
        _ => {}
    }
}

fn move_state(
    curr: &State,
    machine: usize,
    direction: Direction,
    board: &HashMap<Position, Cell>,
    seen: &mut HashSet<(Positions, Vec<char>)>,
    states: &mut Vec<BinaryHeap<State>>,
) {
    let mut next = curr.clone();
    next.steps += 1;
    match direction {
        Direction::Up => {
            next.position[machine].1 -= 1;
        }
        Direction::Down => {
            next.position[machine].1 += 1;
        }
        Direction::Left => {
            next.position[machine].0 -= 1;
        }
        Direction::Right => {
            next.position[machine].0 += 1;
        }
    }
    let mut keys: Vec<char> = next.keys.clone().into_iter().collect();
    keys.sort_unstable();
    match board[&next.position[machine]] {
        Cell::Open | Cell::Entrance => {
            if !seen.contains(&(next.position.clone(), keys.clone())) {
                seen.insert((next.position.clone(), keys));
                states[machine].push(next);
            }
        }
        Cell::Door(x) if next.keys.contains(&x) => {
            if !seen.contains(&(next.position.clone(), keys.clone())) {
                seen.insert((next.position.clone(), keys));
                states[machine].push(next);
            }
        }
        Cell::Key(x) => {
            next.keys.insert(x);
            keys.push(x);
            keys.sort_unstable();
            if !seen.contains(&(next.position.clone(), keys.clone())) {
                seen.insert((next.position.clone(), keys));
                for state in states {
                    state.push(next.clone());
                }
            }
        }
        _ => {}
    }
}

fn process_data_a(data: &str) -> usize {
    let mut start_position = (0, 0);
    let mut board: HashMap<Position, Cell> = HashMap::new();
    let mut key_count = 0;
    for (y, line) in data.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            let cell = Cell::parse(cell).unwrap();
            match cell {
                Cell::Entrance => {
                    start_position = (x, y);
                }
                Cell::Key(_) => {
                    key_count += 1;
                }
                _ => {}
            }
            board.insert((x, y), cell);
        }
    }

    let mut states = BinaryHeap::from(vec![State::new(vec![start_position])]);
    let mut seen: HashSet<(Position, Vec<char>)> = HashSet::new();
    while !states.is_empty() {
        let curr = states.pop().unwrap();
        if curr.keys.len() == key_count {
            return curr.steps;
        }

        // Otherwise, try to move.
        for direction in &DIRECTIONS {
            move_one_state(&curr, *direction, &board, &mut seen, &mut states);
        }
    }

    // 5208 is too high.
    0
}

fn process_data_b(data: &str) -> usize {
    let mut start_position = (0, 0);
    let mut board: HashMap<Position, Cell> = HashMap::new();
    let mut key_count = 0;
    for (y, line) in data.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            let cell = Cell::parse(cell).unwrap();
            match cell {
                Cell::Entrance => {
                    start_position = (x, y);
                }
                Cell::Key(_) => {
                    key_count += 1;
                }
                _ => {}
            }
            board.insert((x, y), cell);
        }
    }

    // Update the map…
    // ...     @#@
    // .@.  => ###
    // ...     @#@
    board.insert(start_position, Cell::Wall);
    board.insert((start_position.0 - 1, start_position.1), Cell::Wall);
    board.insert((start_position.0 + 1, start_position.1), Cell::Wall);
    board.insert((start_position.0, start_position.1 - 1), Cell::Wall);
    board.insert((start_position.0, start_position.1 + 1), Cell::Wall);
    board.insert((start_position.0 - 1, start_position.1 - 1), Cell::Entrance);
    board.insert((start_position.0 - 1, start_position.1 + 1), Cell::Entrance);
    board.insert((start_position.0 + 1, start_position.1 - 1), Cell::Entrance);
    board.insert((start_position.0 + 1, start_position.1 + 1), Cell::Entrance);

    let start = BinaryHeap::from(vec![State::new(vec![
        (start_position.0 - 1, start_position.1 - 1),
        (start_position.0 - 1, start_position.1 + 1),
        (start_position.0 + 1, start_position.1 - 1),
        (start_position.0 + 1, start_position.1 + 1),
    ])]);
    let mut states = vec![start.clone(), start.clone(), start.clone(), start];
    let mut seen = HashSet::new();
    loop {
        for machine in 0..4 {
            while !states[machine].is_empty() {
                let curr = states[machine].pop().unwrap();
                if curr.keys.len() == key_count {
                    return curr.steps;
                }
                // Otherwise, try to move.
                for direction in &DIRECTIONS {
                    move_state(&curr, machine, *direction, &board, &mut seen, &mut states);
                }
            }
        }
    }

    // 5208 is too high.
}

//-----------------------------------------------------
// Questions.

q_impl!("18");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(
            "#########
#b.A.@.a#
#########"
        ),
        8
    );
    assert_eq!(
        process_data_a(
            "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################"
        ),
        86
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(
            "#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#Ab#
#######"
        ),
        8
    );
}
