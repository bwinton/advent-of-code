//-----------------------------------------------------
// Setup.

use std::collections::{HashMap, HashSet, VecDeque};

static INPUT: &str = include_str!("data/q20.data");

enum Cell {
    Wall,
    Empty,
    Unknown(char),
    Portal(Vec<(i32, i32)>),
}

impl Cell {
    fn new(x: char) -> Option<Self> {
        match x {
            '#' => Some(Cell::Wall),
            '.' => Some(Cell::Empty),
            ' ' => None,
            x => Some(Cell::Unknown(x)),
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

type Position = (i32, i32);

#[derive(Copy, Clone, Debug)]
struct State {
    position: Position,
    level: i32,
    steps: usize,
}

impl State {
    fn new(position: Position) -> Self {
        State {
            position,
            level: 0,
            steps: 0,
        }
    }
}

struct Board {
    cells: HashMap<Position, Cell>,
    max_x: i32,
    max_y: i32,
    start_position: Position,
    end_position: Position,
}

impl Board {
    fn new(data: &str) -> Self {
        let mut cells: HashMap<(i32, i32), Cell> = HashMap::new();
        let mut max_x = 0;
        let mut max_y = 0;

        // Parse the cells.HashMap
        for (y, line) in data.lines().enumerate() {
            for (x, cell) in line.chars().enumerate() {
                if let Some(cell) = Cell::new(cell) {
                    cells.insert((x as i32 - 2, y as i32 - 2), cell);
                }
                if x as i32 - 3 > max_x {
                    max_x = x as i32 - 3;
                }
            }
            max_y = y as i32 - 3;
        }

        // Convert the labels into Portals.
        let mut portals: HashMap<String, Vec<(i32, i32)>> = HashMap::new();
        for y in -2..max_y + 2 {
            for x in -2..max_x + 2 {
                if let Some(Cell::Unknown(first)) = cells.get(&(x, y)) {
                    // Found a label, look for the portal.
                    if let Some(Cell::Unknown(second)) = cells.get(&(x, y + 1)) {
                        // vertical
                        if let Some(Cell::Empty) = cells.get(&(x, y - 1)) {
                            portals
                                .entry(format!("{}{}", first, second))
                                .or_default()
                                .push((x, y - 1));
                        } else {
                            portals
                                .entry(format!("{}{}", first, second))
                                .or_default()
                                .push((x, y + 2));
                        }
                    }

                    if let Some(Cell::Unknown(second)) = cells.get(&(x + 1, y)) {
                        // horizontal
                        if let Some(Cell::Empty) = cells.get(&(x - 1, y)) {
                            portals
                                .entry(format!("{}{}", first, second))
                                .or_default()
                                .push((x - 1, y));
                        } else {
                            portals
                                .entry(format!("{}{}", first, second))
                                .or_default()
                                .push((x + 2, y));
                        }
                    }
                }
            }
        }

        let mut start_position = (-1, -1);
        let mut end_position = (-1, -1);

        for portal in portals {
            if portal.1.len() == 2 {
                for cell in &portal.1 {
                    cells.insert(*cell, Cell::Portal(portal.1.clone()));
                }
            } else if portal.0 == "AA" {
                start_position = portal.1[0];
            } else if portal.0 == "ZZ" {
                end_position = portal.1[0];
            }
        }

        Board {
            cells,
            max_x,
            max_y,
            start_position,
            end_position,
        }
    }
}

fn move_state(
    curr: &State,
    direction: Direction,
    board: &Board,
    seen: &mut HashSet<(Position, i32)>,
    states: &mut VecDeque<State>,
    move_level: bool,
) {
    let mut next = *curr;
    next.steps += 1;
    match direction {
        Direction::Up => {
            next.position.1 -= 1;
        }
        Direction::Down => {
            next.position.1 += 1;
        }
        Direction::Left => {
            next.position.0 -= 1;
        }
        Direction::Right => {
            next.position.0 += 1;
        }
    }
    match &board.cells[&next.position] {
        Cell::Empty => {
            if !seen.contains(&(next.position, next.level)) {
                seen.insert((next.position, next.level));
                states.push_front(next);
            }
        }
        Cell::Portal(x) => {
            next.position = if next.position == x[0] { x[1] } else { x[0] };
            if move_level {
                if next.position.0 == 0
                    || next.position.0 == board.max_x - 1
                    || next.position.1 == 0
                    || next.position.1 == board.max_y - 1
                {
                    next.level += 1;
                } else {
                    next.level -= 1;
                }
            }
            next.steps += 1;
            if next.level >= 0 && !seen.contains(&(next.position, next.level)) {
                seen.insert((next.position, next.level));
                states.push_front(next);
            }
        }
        _ => {}
    }
}

fn process_data_a(data: &str) -> usize {
    let board = Board::new(data);

    let mut states = VecDeque::new();
    states.push_front(State::new(board.start_position));
    let mut seen: HashSet<(Position, i32)> = HashSet::new();

    while let Some(curr) = states.pop_back() {
        if curr.position == board.end_position {
            return curr.steps;
        }
        for direction in &DIRECTIONS {
            move_state(&curr, *direction, &board, &mut seen, &mut states, false);
        }
    }
    0
}

fn process_data_b(data: &str) -> usize {
    let board = Board::new(data);

    let mut states = VecDeque::new();
    states.push_front(State::new(board.start_position));
    let mut seen: HashSet<(Position, i32)> = HashSet::new();

    while let Some(curr) = states.pop_back() {
        if curr.position == board.end_position && curr.level == 0 {
            return curr.steps;
        }
        for direction in &DIRECTIONS {
            move_state(&curr, *direction, &board, &mut seen, &mut states, true);
        }
    }

    // 5086 is too low.
    0
}

//-----------------------------------------------------
// Questions.

q_impl!("20");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(
            "         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       
"
        ),
        23
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(
            "         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       
"
        ),
        26
    );
}
