//-----------------------------------------------------
// Setup.

use aoc::Day;

static INPUT: &'static str = include_str!("data/q03.data");
// static INPUT : &'static str = "5 10 25";
// static INPUT : &'static str = "101 301 501
// 102 302 502
// 103 303 503
// 201 401 601
// 202 402 602
// 203 403 603";

fn parse_line(line: &str) -> Vec<u32> {
  line
    .split_whitespace()
    .map(|n| n.parse().expect("Wanted a number"))
    .collect::<Vec<u32>>()
}


//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
  fn number(&self) -> String {
    String::from("3")
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    let mut possible = 0;

    fn test_data(data: &mut Vec<u32>) -> bool {
      data.sort();
      let goal = data.pop().unwrap();
      let total = data.iter().sum::<u32>();
      goal < total
    }

    for line in INPUT.lines() {
      let mut data = parse_line(line);
      if test_data(&mut data) {
        possible += 1;
      }
    }
    println!("Result = {}", possible);
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    let mut possible: u32 = 0;

    fn test_data(data: &mut Vec<u32>) -> bool {
      data.sort();
      let goal = data.pop().unwrap();
      let total = data.iter().sum::<u32>();
      goal < total
    }

    fn handle_data(current: &mut Vec<Vec<u32>>) -> u32 {
      let mut possible = 0;
      for x in izip!(current[0].iter(), current[1].iter(), current[2].iter()) {
        let mut data: Vec<u32> = vec![*x.0, *x.1, *x.2];
        if test_data(&mut data) {
          possible += 1;
        }
      }
      // println!("{:?}, {}", current, possible);
      current.clear();
      possible
    }

    let mut current: Vec<Vec<u32>> = Vec::new();

    for line in INPUT.lines() {
      let data = parse_line(line);
      current.push(data);
      if current.len() == 3 {
        possible += handle_data(&mut current)
      }
    }
    println!("Result = {}", possible);
  }
}
