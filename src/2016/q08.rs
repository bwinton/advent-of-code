//-----------------------------------------------------
// Setup.

use aoc::Day;
use regex::Regex;
use std::fmt;

static INPUT: &'static str = include_str!("data/q08.data");
// static INPUT : &'static str = "rect 3x2
// rotate column x=1 by 1
// rotate row y=0 by 4
// rotate column x=1 by 1";

const COLS: usize = 50;
const ROWS: usize = 6;
// const COLS : usize = 7;
// const ROWS : usize = 3;

struct Display {
  // 50 wide by 6 high.
  on: i32,
  cells: [[bool; COLS]; ROWS],
}

impl Display {
  fn new() -> Display {
    Display {
      on: 0,
      cells: [[false; COLS]; ROWS],
    }
  }

  fn on(&self) -> i32 {
    self.on
  }

  fn run(&mut self, turn: &Turn) {
    // println!("{:?}", turn);
    match turn.kind.as_ref() {
      "rect" => {
        for i in 0..turn.arg2 {
          for j in 0..turn.arg1 {
            if !self.cells[i][j] {
              self.cells[i][j] = true;
              self.on += 1;
            }
          }
        }
      },
      "rotate column" => {
        let mut new = [false; ROWS];
        for (i, cell) in new.iter_mut().enumerate().take(ROWS) {
          // println!("cell[{}][{}] from [{}][{}]", i, turn.arg1, (ROWS + i - turn.arg2) % ROWS, turn.arg1);
          *cell = self.cells[(ROWS + i - turn.arg2) % ROWS][turn.arg1];
        }
        for (i, &row) in new.iter().enumerate().take(ROWS) {
          self.cells[i][turn.arg1] = row;
        }
      },
      "rotate row" => {
        let mut new = [false; COLS];
        for (i, cell) in new.iter_mut().enumerate().take(COLS) {
          // println!("cell[{}][{}] from [{}][{}]", turn.arg1, i, turn.arg1, (COLS + i - turn.arg2) % COLS);
          *cell = self.cells[turn.arg1][(COLS + i - turn.arg2) % COLS];
        }
        self.cells[turn.arg1][..COLS].clone_from_slice(&new[..COLS]);
      },
      _ => println!("Error!"),
    }
  }
}

impl fmt::Debug for Display {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    for i in 0..ROWS {
      for j in 0..COLS {
        if self.cells[i][j] {
          write!(formatter, "#").unwrap();
        } else {
          write!(formatter, ".").unwrap();
        }
      }
      writeln!(formatter).unwrap();
    }
    Ok(())
  }
}


#[derive(Debug)]
struct Turn {
  kind: String,
  arg1: usize,
  arg2: usize,
}

impl Turn {}

use std::str::FromStr;
impl FromStr for Turn {
  type Err = ();

  fn from_str(s: &str) -> Result<Turn, ()> {
    let turn_re = Regex::new(r"^(rect|rotate column|rotate row) (.+)$").unwrap();
    let rect_re = Regex::new(r"^(\d+)x(\d+)$").unwrap();
    let rotate_re = Regex::new(r"^(x|y)=(\d+) by (\d+)$").unwrap();
    let blank = String::from("");
    let mut rv = Turn {
      kind: blank.clone(),
      arg1: 0,
      arg2: 0,
    };
    for cap in turn_re.captures_iter(s) {
      rv.kind = cap[1].to_string();
      let rest = &cap[2].to_string();
      match rv.kind.as_ref() {
        "rect" => {
          let extra = rect_re.captures(rest).unwrap();
          rv.arg1 = extra[1].parse().unwrap();
          rv.arg2 = extra[2].parse().unwrap();
        },
        "rotate column" | "rotate row" => {
          let extra = rotate_re.captures(rest).unwrap();
          rv.arg1 = extra[2].parse().unwrap();
          rv.arg2 = extra[3].parse().unwrap();
        },
        _ => println!("Error!"),
      }
    }
    Ok(rv)
  }
}


//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
  fn number(&self) -> String {
    String::from("8")
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    let mut display = Display::new();
    // println!("{:?}", display);
    for line in INPUT.lines() {
      let turn: Turn = line.parse().unwrap();
      display.run(&turn);
      // println!("{:?}", display);
    }
    println!("Result = {}", display.on());
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    let mut display = Display::new();
    for line in INPUT.lines() {
      let turn: Turn = line.parse().unwrap();
      display.run(&turn);
    }
    println!("Result =");
    println!("{:?}", display);
  }
}
