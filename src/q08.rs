//-----------------------------------------------------
// Setup.

use day;
use regex::Regex;
use std::fmt;

static INPUT : &'static str = "rect 1x1
rotate row y=0 by 5
rect 1x1
rotate row y=0 by 5
rect 1x1
rotate row y=0 by 5
rect 1x1
rotate row y=0 by 5
rect 1x1
rotate row y=0 by 2
rect 1x1
rotate row y=0 by 2
rect 1x1
rotate row y=0 by 3
rect 1x1
rotate row y=0 by 3
rect 2x1
rotate row y=0 by 2
rect 1x1
rotate row y=0 by 3
rect 2x1
rotate row y=0 by 2
rect 1x1
rotate row y=0 by 3
rect 2x1
rotate row y=0 by 5
rect 4x1
rotate row y=0 by 5
rotate column x=0 by 1
rect 4x1
rotate row y=0 by 10
rotate column x=5 by 2
rotate column x=0 by 1
rect 9x1
rotate row y=2 by 5
rotate row y=0 by 5
rotate column x=0 by 1
rect 4x1
rotate row y=2 by 5
rotate row y=0 by 5
rotate column x=0 by 1
rect 4x1
rotate column x=40 by 1
rotate column x=27 by 1
rotate column x=22 by 1
rotate column x=17 by 1
rotate column x=12 by 1
rotate column x=7 by 1
rotate column x=2 by 1
rotate row y=2 by 5
rotate row y=1 by 3
rotate row y=0 by 5
rect 1x3
rotate row y=2 by 10
rotate row y=1 by 7
rotate row y=0 by 2
rotate column x=3 by 2
rotate column x=2 by 1
rotate column x=0 by 1
rect 4x1
rotate row y=2 by 5
rotate row y=1 by 3
rotate row y=0 by 3
rect 1x3
rotate column x=45 by 1
rotate row y=2 by 7
rotate row y=1 by 10
rotate row y=0 by 2
rotate column x=3 by 1
rotate column x=2 by 2
rotate column x=0 by 1
rect 4x1
rotate row y=2 by 13
rotate row y=0 by 5
rotate column x=3 by 1
rotate column x=0 by 1
rect 4x1
rotate row y=3 by 10
rotate row y=2 by 10
rotate row y=0 by 5
rotate column x=3 by 1
rotate column x=2 by 1
rotate column x=0 by 1
rect 4x1
rotate row y=3 by 8
rotate row y=0 by 5
rotate column x=3 by 1
rotate column x=2 by 1
rotate column x=0 by 1
rect 4x1
rotate row y=3 by 17
rotate row y=2 by 20
rotate row y=0 by 15
rotate column x=13 by 1
rotate column x=12 by 3
rotate column x=10 by 1
rotate column x=8 by 1
rotate column x=7 by 2
rotate column x=6 by 1
rotate column x=5 by 1
rotate column x=3 by 1
rotate column x=2 by 2
rotate column x=0 by 1
rect 14x1
rotate row y=1 by 47
rotate column x=9 by 1
rotate column x=4 by 1
rotate row y=3 by 3
rotate row y=2 by 10
rotate row y=1 by 8
rotate row y=0 by 5
rotate column x=2 by 2
rotate column x=0 by 2
rect 3x2
rotate row y=3 by 12
rotate row y=2 by 10
rotate row y=0 by 10
rotate column x=8 by 1
rotate column x=7 by 3
rotate column x=5 by 1
rotate column x=3 by 1
rotate column x=2 by 1
rotate column x=1 by 1
rotate column x=0 by 1
rect 9x1
rotate row y=0 by 20
rotate column x=46 by 1
rotate row y=4 by 17
rotate row y=3 by 10
rotate row y=2 by 10
rotate row y=1 by 5
rotate column x=8 by 1
rotate column x=7 by 1
rotate column x=6 by 1
rotate column x=5 by 1
rotate column x=3 by 1
rotate column x=2 by 2
rotate column x=1 by 1
rotate column x=0 by 1
rect 9x1
rotate column x=32 by 4
rotate row y=4 by 33
rotate row y=3 by 5
rotate row y=2 by 15
rotate row y=0 by 15
rotate column x=13 by 1
rotate column x=12 by 3
rotate column x=10 by 1
rotate column x=8 by 1
rotate column x=7 by 2
rotate column x=6 by 1
rotate column x=5 by 1
rotate column x=3 by 1
rotate column x=2 by 1
rotate column x=1 by 1
rotate column x=0 by 1
rect 14x1
rotate column x=39 by 3
rotate column x=35 by 4
rotate column x=20 by 4
rotate column x=19 by 3
rotate column x=10 by 4
rotate column x=9 by 3
rotate column x=8 by 3
rotate column x=5 by 4
rotate column x=4 by 3
rotate row y=5 by 5
rotate row y=4 by 5
rotate row y=3 by 33
rotate row y=1 by 30
rotate column x=48 by 1
rotate column x=47 by 5
rotate column x=46 by 5
rotate column x=45 by 1
rotate column x=43 by 1
rotate column x=38 by 3
rotate column x=37 by 3
rotate column x=36 by 5
rotate column x=35 by 1
rotate column x=33 by 1
rotate column x=32 by 5
rotate column x=31 by 5
rotate column x=30 by 1
rotate column x=23 by 4
rotate column x=22 by 3
rotate column x=21 by 3
rotate column x=20 by 1
rotate column x=12 by 2
rotate column x=11 by 2
rotate column x=3 by 5
rotate column x=2 by 5
rotate column x=1 by 3
rotate column x=0 by 4";
// static INPUT : &'static str = "rect 3x2
// rotate column x=1 by 1
// rotate row y=0 by 4
// rotate column x=1 by 1";

const COLS : usize = 50;
const ROWS : usize = 6;
// const COLS : usize = 7;
// const ROWS : usize = 3;

struct Display {
  // 50 wide by 6 high.
  on: i32,
  cells: [ [bool; COLS]; ROWS]
}

impl Display {
  fn new() -> Display {
    Display{ on:0, cells: [[false; COLS]; ROWS]}
  }

  fn on(&self) -> i32 {
    // let mut rv = 0;
    // for i in 0..ROWS {
    //   for j in 0..COLS {
    //     if self.cells[i][j] {
    //       rv += 1;
    //     }
    //   }
    // }
    // println!("On, {} == {}", self.on, rv);
    // return rv;
    return self.on;
  }

  fn run(&mut self, turn: Turn) {
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
        for i in 0..ROWS {
          // println!("cell[{}][{}] from [{}][{}]", i, turn.arg1, (ROWS + i - turn.arg2) % ROWS, turn.arg1);
          new[i] = self.cells[(ROWS + i - turn.arg2) % ROWS][turn.arg1];
        }
        for i in 0..ROWS {
          self.cells[i][turn.arg1] = new[i];
        }
      },
      "rotate row" => {
        let mut new = [false; COLS];
        for i in 0..COLS {
          // println!("cell[{}][{}] from [{}][{}]", turn.arg1, i, turn.arg1, (COLS + i - turn.arg2) % COLS);
          new[i] = self.cells[turn.arg1][(COLS + i - turn.arg2) % COLS];
        }
        for i in 0..COLS {
          self.cells[turn.arg1][i] = new[i];
        }
      },
      _ => {println!("Error!")},
    }
  }
}

impl fmt::Debug for Display {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    for i in 0..ROWS {
      for j in 0..COLS {
        if self.cells[i][j] {
          write!(formatter, "#");
        } else {
          write!(formatter, ".");
        }
      }
      write!(formatter, "\n");
    }
    return Ok(());
  }
}


#[derive(Debug)]
struct Turn {
  kind: String,
  arg1: usize,
  arg2: usize,
}

impl Turn {
}

use std::str::FromStr;
impl FromStr for Turn {
  type Err = ();

  fn from_str(s: &str) -> Result<Turn, ()> {
    let turn_re = Regex::new(r"^(rect|rotate column|rotate row) (.+)$").unwrap();
    let rect_re = Regex::new(r"^(\d+)x(\d+)$").unwrap();
    let rotate_re = Regex::new(r"^(x|y)=(\d+) by (\d+)$").unwrap();
    let blank = String::from("");
    let mut rv = Turn{kind: blank.clone(), arg1: 0, arg2: 0};
    for cap in turn_re.captures_iter(s) {
      rv.kind = String::from(cap.at(1).unwrap_or(""));
      let rest = cap.at(2).unwrap_or("");
      match rv.kind.as_ref() {
        "rect" => {
          let extra = rect_re.captures(rest).unwrap();
          rv.arg1 = extra.at(1).unwrap_or("0").parse().unwrap();
          rv.arg2 = extra.at(2).unwrap_or("0").parse().unwrap();
        },
        "rotate column" | "rotate row" => {
          let extra = rotate_re.captures(rest).unwrap();
          rv.arg1 = extra.at(2).unwrap_or("0").parse().unwrap();
          rv.arg2 = extra.at(3).unwrap_or("0").parse().unwrap();
        },
        _ => {println!("Error!")},
      }
    }
    return Ok(rv);
    // on fail, return Err(());
  }
}


//-----------------------------------------------------
// Questions.

pub struct Q;

impl day::Day for Q {
  fn number(&self) -> String {
    return String::from("8");
  }

  fn a(&self) {
    print!("8A: ");
    let mut display = Display::new();
    // println!("{:?}", display);
    for line in INPUT.lines() {
      let turn : Turn = line.parse().unwrap();
      display.run(turn);
      // println!("{:?}", display);
    }
    println!("Result = {}", display.on());
  }

  fn b(&self) {
    print!("8B: ");
    let mut display = Display::new();
    for line in INPUT.lines() {
      let turn : Turn = line.parse().unwrap();
      display.run(turn);
    }
    println!("Result =");
    println!("{:?}", display);
  }
}