//-----------------------------------------------------
// Setup.

use std::{
    cmp::Ordering,
    collections::BTreeSet,
    fmt::{Display, Formatter, Result as FmtResult},
    hash::{Hash, Hasher},
};

static INPUT: &str = include_str!("data/q13.data");

#[derive(Clone, Debug)]
enum Direction {
    Left,
    Straight,
    Right,
}

#[derive(Clone, Debug)]
struct Cart {
    direction: char,
    x: usize,
    y: usize,
    turn: Direction,
}

impl Display for Cart {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{} at ({},{})", self.direction, self.x, self.y)
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Equal => self.x.cmp(&other.x),
            rv => rv,
        }
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cart {
    fn eq(&self, other: &Cart) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Cart {}

impl Hash for Cart {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Cart {
    fn drive(&mut self, cells: &[Vec<char>]) {
        match self.direction {
            '^' => {
                self.y -= 1;
                match (cells[self.y][self.x], &self.turn) {
                    ('/', _) => self.direction = '>',
                    ('\\', _) => self.direction = '<',
                    ('+', Direction::Left) => self.direction = '<',
                    ('+', Direction::Right) => self.direction = '>',
                    _ => {}
                }
            }
            'v' => {
                self.y += 1;
                match (cells[self.y][self.x], &self.turn) {
                    ('/', _) => self.direction = '<',
                    ('\\', _) => self.direction = '>',
                    ('+', Direction::Left) => self.direction = '>',
                    ('+', Direction::Right) => self.direction = '<',
                    _ => {}
                }
            }
            '<' => {
                self.x -= 1;
                match (cells[self.y][self.x], &self.turn) {
                    ('/', _) => self.direction = 'v',
                    ('\\', _) => self.direction = '^',
                    ('+', Direction::Left) => self.direction = 'v',
                    ('+', Direction::Right) => self.direction = '^',
                    _ => {}
                }
            }
            '>' => {
                self.x += 1;
                match (cells[self.y][self.x], &self.turn) {
                    ('/', _) => self.direction = '^',
                    ('\\', _) => self.direction = 'v',
                    ('+', Direction::Left) => self.direction = '^',
                    ('+', Direction::Right) => self.direction = 'v',
                    _ => {}
                }
            }
            _ => {}
        }
        // If we're at an intersection, we turned, so update our turn.
        if cells[self.y][self.x] == '+' {
            match self.turn {
                Direction::Left => self.turn = Direction::Straight,
                Direction::Straight => self.turn = Direction::Right,
                Direction::Right => self.turn = Direction::Left,
            }
        }
    }
}

#[derive(Debug)]
struct Board {
    cells: Vec<Vec<char>>,
    carts: BTreeSet<Cart>,
}

impl Board {
    fn from_data(data: &str) -> Self {
        let mut cells = Vec::new();
        let mut carts = BTreeSet::new();
        for (y, line) in data.lines().enumerate() {
            let mut row = Vec::new();
            for (x, cell) in line.chars().enumerate() {
                let mut cell = cell;
                if cell == '^' || cell == 'v' || cell == '<' || cell == '>' {
                    carts.insert(Cart {
                        direction: cell,
                        x,
                        y,
                        turn: Direction::Left,
                    });
                    if cell == '^' || cell == 'v' {
                        cell = '|';
                    } else {
                        cell = '-';
                    }
                }
                row.push(cell);
            }
            cells.push(row);
        }
        Board { cells, carts }
    }

    fn step(&mut self) -> Vec<Cart> {
        // Move the carts and figure out their new direction.
        let mut crashes = vec![];
        let mut to_move = self.carts.clone();
        let mut moved = BTreeSet::new();
        for cart in self.carts.iter() {
            // Don't consider us in the to-moved set, cause we're moving.
            to_move.remove(cart);
            if moved.contains(cart) || crashes.contains(cart) {
                // We hit something already, so skip this one.
                continue;
            }
            // move this cart, and check for collisions…
            let mut cart = cart.clone();
            cart.drive(&self.cells);

            // Did we hit something that hasn't moved yet?
            if to_move.contains(&cart) {
                crashes.push(to_move.take(&cart).unwrap());
                crashes.push(cart);
            } else if moved.contains(&cart) {
                // Did we hit something that has already moved?
                crashes.push(moved.take(&cart).unwrap());
                crashes.push(cart);
            } else {
                moved.insert(cart);
            }
        }
        self.carts = moved;
        crashes
    }
}

fn process_data_a(data: &str) -> String {
    let mut board = Board::from_data(data);
    let mut crashes = board.step();
    while crashes.is_empty() {
        crashes = board.step();
    }

    let crash = crashes.first().unwrap();
    format!("{},{}", crash.x, crash.y)
}

fn process_data_b(data: &str) -> String {
    let mut board = Board::from_data(data);
    while board.carts.len() > 1 {
        board.step();
    }

    let last = board.carts.iter().next().unwrap();
    format!("{},{}", last.x, last.y)
}

//-----------------------------------------------------
// Questions.

q_impl!("13");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(
            r#"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   "#
        ),
        "7,3".to_string()
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(
            r#"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/"#
        ),
        "6,4".to_string()
    );
}
