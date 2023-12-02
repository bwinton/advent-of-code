//-----------------------------------------------------
// Setup.

use aoc::Day;

use std::iter::FromIterator;

static INPUT: &str = include_str!("data/q19.data");

#[derive(Clone, Debug, PartialEq)]
enum Cell {
    Empty,
    Vertical,
    Horizontal,
    Both,
    Letter(char),
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Packet {
    pos: (usize, usize),
    dir: Direction,
    seen_letters: Vec<char>,
    steps: u64,
}

impl Packet {
    pub fn new(board: BoardRef) -> Packet {
        Packet {
            pos: get_starting_pos(board).unwrap(),
            dir: Direction::Down,
            seen_letters: Vec::new(),
            steps: 0,
        }
    }

    fn go(&mut self, board: BoardRef) -> bool {
        let curr = get_cell(self.pos, board).unwrap();
        self.steps += 1;
        if let Cell::Letter(x) = curr {
            self.seen_letters.push(x);
        }
        let mut next_pos = match self.dir {
            Direction::Up => (self.pos.0 - 1, self.pos.1),
            Direction::Down => (self.pos.0 + 1, self.pos.1),
            Direction::Left => (self.pos.0, self.pos.1 - 1),
            Direction::Right => (self.pos.0, self.pos.1 + 1),
        };
        let mut next = get_cell(next_pos, board);
        // println!("Going from {:?}/{:?} to {:?}/{:?}", self.pos, curr, next_pos, next);
        if next != Some(Cell::Empty) && next.is_some() {
            self.pos = next_pos;
            return true;
        }
        // Time to see if we can go a different direction!
        if curr != Cell::Both {
            return false;
        }
        match self.dir {
            Direction::Up | Direction::Down => {
                next_pos = (self.pos.0, self.pos.1 - 1);
                next = get_cell(next_pos, board);
                // println!("  Turning Left {:?}/{:?}", next_pos, next);
                if next != Some(Cell::Empty) && next.is_some() {
                    self.pos = next_pos;
                    self.dir = Direction::Left;
                } else {
                    next_pos = (self.pos.0, self.pos.1 + 1);
                    next = get_cell(next_pos, board);
                    // println!("  Turning Right {:?}/{:?}", next_pos, next);
                    if next != Some(Cell::Empty) && next.is_some() {
                        self.pos = next_pos;
                        self.dir = Direction::Right;
                    }
                }
            }
            Direction::Left | Direction::Right => {
                next_pos = (self.pos.0 - 1, self.pos.1);
                next = get_cell(next_pos, board);
                // println!("  Turning Up {:?}/{:?}", next_pos, next);
                if next != Some(Cell::Empty) && next.is_some() {
                    self.pos = next_pos;
                    self.dir = Direction::Up;
                } else {
                    next_pos = (self.pos.0 + 1, self.pos.1);
                    next = get_cell(next_pos, board);
                    // println!("  Turning Down {:?}/{:?}", next_pos, next);
                    if next != Some(Cell::Empty) && next.is_some() {
                        self.pos = next_pos;
                        self.dir = Direction::Down;
                    }
                }
            }
        }
        true
    }
}

type Board = Vec<Vec<Cell>>;
type BoardRef<'a> = &'a [Vec<Cell>];

fn get_cell(pos: (usize, usize), board: BoardRef) -> Option<Cell> {
    if pos.0 < board.len() && pos.1 < board[pos.0].len() {
        Some(board[pos.0][pos.1].clone())
    } else {
        None
    }
}

fn get_board(data: &str) -> Board {
    let mut board: Board = Vec::new();
    // println!();
    for line in data.lines() {
        // println!("{}", line);
        let mut row = Vec::new();
        for cell in line.chars() {
            row.push(match cell {
                ' ' => Cell::Empty,
                '|' => Cell::Vertical,
                '-' => Cell::Horizontal,
                '+' => Cell::Both,
                x => Cell::Letter(x),
            })
        }
        board.push(row);
    }
    board
}

fn get_starting_pos(board: BoardRef) -> Option<(usize, usize)> {
    for cell in 0..board[0].len() {
        if board[0][cell] == Cell::Vertical {
            return Some((0, cell));
        }
    }
    None
}

fn process_data_a(data: &str) -> String {
    let board = get_board(data);
    let mut packet = Packet::new(&board);
    // println!("\n{:?} - {:?}\n", packet, board);

    while packet.go(&board) {
        // packet.go has it all covered… ;)
        // println!("  {:?}\n", packet);
    }

    String::from_iter(&packet.seen_letters)
}

fn process_data_b(data: &str) -> u64 {
    let board = get_board(data);
    let mut packet = Packet::new(&board);
    // println!("\n{:?} - {:?}\n", packet, board);

    while packet.go(&board) {
        // packet.go has it all covered… ;)
        // println!("  {:?}\n", packet);
    }

    packet.steps
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
    fn number(&self) -> String {
        String::from("19")
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
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(
            "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ ",
        ),
        "ABCDEF"
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(
            "     |          
     |  +--+    
     A  |  C    
 F---|--|-E---+ 
     |  |  |  D 
     +B-+  +--+ 
",
        ),
        38
    );
}
