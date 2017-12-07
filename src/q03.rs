//-----------------------------------------------------
// Setup.

use day;

use std;
use std::collections::HashMap;

static INPUT : i32 = 265_149;

enum Direction {
  Left,
  Up,
  Right,
  Down
}

define_iterator!(SpiralIter (
    &curr: [i32; 2] = [0, 0],
    &dir: Direction = Direction::Left,
    &len: usize = 1,
    &remaining: usize = 1
  ) -> Option<[i32;2]> {
  let rv = *curr;

  *remaining -= 1;

  match *dir {
    Direction::Left => {
      curr[0] += 1;
      if *remaining == 0 {
        *dir = Direction::Up;
        *remaining = *len;
      }
    },
    Direction::Up => {
      curr[1] -= 1;
      if *remaining == 0 {
        *dir = Direction::Right;
        *len += 1;
        *remaining = *len;
      }
    },
    Direction::Right => {
      curr[0] -= 1;
      if *remaining == 0 {
        *dir = Direction::Down;
        *remaining = *len;
      }
    },
    Direction::Down => {
      curr[1] += 1;
      if *remaining == 0 {
        *dir = Direction::Left;
        *len += 1;
        *remaining = *len;
      }
    }
  }

  Some(rv)
});

fn process_data_a(data: i32) -> i32 {
  if data == 1 { return 0; }
  let numbers = 0..;
  let mut rv = 0;
  for i in numbers {
    let block = 2 * i + 1;
    if block*block >= data {
      let remainder = data - (block-2)*(block-2);
      let low = i;
      let high = 2*i;
      let mut seesaw = (low..high).chain(high-1..low+1).cycle().skip(remainder as usize);
      rv = seesaw.next().unwrap();
      break;
    }
  }
  rv
}


define_iterator!(MultIter(
    &spiral: SpiralIter = SpiralIter::default(),
    &seen: HashMap<[i32;2], usize> = HashMap::new()
  ) -> Option<usize> {
  let mut rv = 0;
  let curr = spiral.next().unwrap();
  if curr == [0,0] {
    rv = 1;
  }
  for x in -1..2 {
    for y in -1..2 {
      if let Some(cell) = seen.get(&[curr[0] + x, curr[1] + y]) {
        rv += cell;
      }
    }
  }
  seen.insert(curr, rv);
  Some(rv)
});


fn process_data_b(data: i32) -> usize {
  for number in MultIter::default() {
    if number > data as usize {
      return number;
    }
  }
  unreachable!();
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl day::Day for Q {
  fn number(&self) -> String {
    String::from("3")
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
  assert_eq!(process_data_a(1), 0);
  assert_eq!(process_data_a(12), 3);
  assert_eq!(process_data_a(23), 2);
  assert_eq!(process_data_a(1024), 31);
}

#[test]
fn b() {
  let spiral_values: Vec<[i32;2]> = SpiralIter::default().take(25).collect();
  let spiral_expected: Vec<[i32;2]> = vec![
      [0,0],   [1,0],  [1,-1], [0,-1], [-1,-1],
     [-1,0],  [-1,1],   [0,1],  [1,1],   [2,1],
      [2,0],  [2,-1],  [2,-2], [1,-2],  [0,-2],
    [-1,-2], [-2,-2], [-2,-1], [-2,0],  [-2,1],
     [-2,2],  [-1,2],   [0,2],  [1,2],   [2,2]
  ];
  assert_eq!(spiral_values, spiral_expected);

  let mult_values: Vec<usize> = MultIter::default().take(23).collect();
  let mult_expected: Vec<usize> = vec![
      1,   1,   2,   4,   5,
     10,  11,  23,  25,  26,
     54,  57,  59, 122, 133,
    142, 147, 304, 330, 351,
    362, 747, 806];
  assert_eq!(mult_values, mult_expected);
}
