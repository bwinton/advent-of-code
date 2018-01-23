//-----------------------------------------------------
// Setup.

use aoc::Day;

use itertools::Itertools;

static INPUT: &'static str = "197,97,204,108,1,29,5,71,0,50,2,255,248,78,254,63";

fn process_data(len: usize, data: &[usize], rounds: usize) -> Vec<usize> {
  let mut numbers: Vec<usize> = (0..len).collect();
  let mut start = 0;
  let mut skip = 0;
  for _round in 0..rounds {
    for twist in data {
      let mut end = start + twist;
      // println!("{:?}, {}, {}", numbers, start, end);
      if end <= len {
        numbers[start..end].reverse();
      } else {
        let mut temp = numbers.clone();
        temp.append(&mut numbers.clone());
        temp[start..end].reverse();
        numbers.splice(start.., temp[start..len].iter().cloned());
        numbers.splice(..end % len, temp[len..end].iter().cloned());
      }
      start = (end + skip) % len;
      skip = (skip + 1) % len;
    }
  }
  numbers
}

fn get_input(input: &str) -> Vec<usize> {
  let mut rv: Vec<usize> = input.chars().map(|x| x as usize).collect();
  rv.extend(&mut [17, 31, 73, 47, 23].iter());
  rv
}

fn densify(sparse: &[usize]) -> String {
  let mut temp = Vec::new();
  for chunk in &sparse.iter().chunks(16) {
    temp.push(chunk.fold(0, |acc, &x| acc ^ x));
  }
  temp.iter().map(|x| format!("{:02x}", x)).join("")
}


//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
  fn number(&self) -> String {
    String::from("10")
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    let input: Vec<usize> = INPUT
      .split(',')
      .map(|x| x.parse::<usize>().unwrap())
      .collect();
    let result = process_data(256, &input, 1);
    println!("Result = {}", result[0] * result[1]);
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    println!("Result = {}", densify(&process_data(256, &get_input(INPUT), 64)));
  }
}

#[test]
fn a() {
  assert_eq!(process_data(5, &vec![3, 4, 1, 5], 1), vec![3, 4, 2, 1, 0]);
}

#[test]
fn b() {
  assert_eq!(65 ^ 27 ^ 9 ^ 1 ^ 4 ^ 3 ^ 40 ^ 50 ^ 91 ^ 7 ^ 6 ^ 0 ^ 2 ^ 5 ^ 68 ^ 22, 64);
  assert_eq!(
    vec![64, 7, 255].iter().map(|x| format!("{:02x}", x)).join(
      "",
    ),
    "4007ff"
  );
  assert_eq!(get_input("1,2,3"), vec![49, 44, 50, 44, 51, 17, 31, 73, 47, 23]);
  assert_eq!(
    densify(&process_data(256, &get_input(""), 64)),
    "a2582a3a0e66e6e86e3812dcb672a272"
  );
  assert_eq!(
    densify(&process_data(256, &get_input("AoC 2017"), 64)),
    "33efeb34ea91902bb2f59c9920caa6cd"
  );
  assert_eq!(
    densify(&process_data(256, &get_input("1,2,3"), 64)),
    "3efbe78a8d82f29979031a4aa0b16a9d"
  );
  assert_eq!(
    densify(&process_data(256, &get_input("1,2,4"), 64)),
    "63960835bcdc130f0b66d7ff4f6a5a8e"
  );
}
