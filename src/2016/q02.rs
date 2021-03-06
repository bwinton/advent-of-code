//-----------------------------------------------------
// Setup.

use aoc::Day;

static INPUT: &str = include_str!("data/q02.data");
// static INPUT : &'static str = "ULL
// RRDDD
// LURDL
// UUUUD";

type Key = [usize; 2];
type Keypad = Vec<Vec<char>>;
type KeypadRef<'a> = &'a [Vec<char>];

#[derive(Debug)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

fn get(keypad: KeypadRef, key: Key) -> char {
    keypad[key[0]][key[1]]
}

impl Direction {
    fn shift(&self, key: Key, keypad: KeypadRef) -> Key {
        match *self {
            Direction::Up => {
                if get(keypad, [key[0] - 1, key[1]]) == ' ' {
                    key
                } else {
                    [key[0] - 1, key[1]]
                }
            }
            Direction::Left => {
                if get(keypad, [key[0], key[1] - 1]) == ' ' {
                    key
                } else {
                    [key[0], key[1] - 1]
                }
            }
            Direction::Down => {
                if get(keypad, [key[0] + 1, key[1]]) == ' ' {
                    key
                } else {
                    [key[0] + 1, key[1]]
                }
            }
            Direction::Right => {
                if get(keypad, [key[0], key[1] + 1]) == ' ' {
                    key
                } else {
                    [key[0], key[1] + 1]
                }
            }
        }
    }
}

use std::str::FromStr;
impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Direction, ()> {
        match s {
            "U" => Ok(Direction::Up),
            "L" => Ok(Direction::Left),
            "D" => Ok(Direction::Down),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

fn handle_direction(key: Key, keypad: KeypadRef, next: char) -> Key {
    let direction: Direction = next.to_string().parse().unwrap();
    // println!("{:?}, {:?}", direction, direction.shift(key));
    direction.shift(key, keypad)
}

fn parse_line(key: &mut Key, keypad: KeypadRef, line: &str) {
    for direction in line.chars() {
        *key = handle_direction(*key, keypad, direction);
    }
    print!("{}", get(keypad, *key));
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
    fn number(&self) -> String {
        String::from("2")
    }

    fn a(&self) {
        print!("{}A: ", self.number());
        let keypad: Keypad = vec![
            vec![' ', ' ', ' ', ' ', ' '],
            vec![' ', '1', '2', '3', ' '],
            vec![' ', '4', '5', '6', ' '],
            vec![' ', '7', '8', '9', ' '],
            vec![' ', ' ', ' ', ' ', ' '],
        ];

        let mut key: Key = [2, 2];

        print!("Result = ");
        for line in INPUT.lines() {
            parse_line(&mut key, &keypad, line);
        }
        println!();
    }

    fn b(&self) {
        print!("{}B: ", self.number());
        let keypad: Keypad = vec![
            vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
            vec![' ', ' ', ' ', '1', ' ', ' ', ' '],
            vec![' ', ' ', '2', '3', '4', ' ', ' '],
            vec![' ', '5', '6', '7', '8', '9', ' '],
            vec![' ', ' ', 'A', 'B', 'C', ' ', ' '],
            vec![' ', ' ', ' ', 'D', ' ', ' ', ' '],
            vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ];

        let mut key: Key = [3, 1];

        print!("Result = ");
        for line in INPUT.lines() {
            parse_line(&mut key, &keypad, line);
        }
        println!();
    }
}
