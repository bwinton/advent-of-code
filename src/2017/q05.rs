//-----------------------------------------------------
// Setup.

use aoc::Day;

static INPUT: &'static str = include_str!("data/q05.data");

fn parse(data: &str) -> Vec<i32> {
  data.lines().map(|i| i.parse::<i32>().unwrap()).collect()
}

fn process_data_a(data: &str) -> i32 {
  let mut jumps = parse(data);
  let mut pc: i32 = 0;
  let mut count = 0;
  while pc < jumps.len() as i32 && pc >= 0 {
    let prev = jumps[pc as usize];
    jumps[pc as usize] += 1;
    pc += prev;
    count += 1;
  }
  count
}

fn process_data_b(data: &str) -> i32 {
  let mut jumps = parse(data);
  let mut pc: i32 = 0;
  let mut count = 0;
  while pc < jumps.len() as i32 && pc >= 0 {
    let prev = jumps[pc as usize];
    // println!("{}: {:?}", pc, jumps);
    jumps[pc as usize] += if prev >= 3 { -1 } else { 1 };
    pc += prev;
    count += 1;
  }
  count
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
  fn number(&self) -> String {
    String::from("5")
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
  assert_eq!(
    process_data_a(
      "0
3
0
1
-3",
    ),
    5
  );
}

#[test]
fn b() {
  assert_eq!(
    process_data_b(
      "0
3
0
1
-3",
    ),
    10
  );
}
