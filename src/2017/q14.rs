//-----------------------------------------------------
// Setup.

use aoc::Day;

use itertools::Itertools;
use itertools::enumerate;

static INPUT: &'static str = "uugsqrei";

fn get_input(input: &str) -> Vec<usize> {
  let mut rv: Vec<usize> = input.chars().map(|x| x as usize).collect();
  rv.extend(&mut [17, 31, 73, 47, 23].iter());
  rv
}

fn densify(sparse: &[usize]) -> Vec<i64> {
  let mut temp = Vec::new();
  for chunk in &sparse.iter().chunks(16) {
    temp.push(chunk.fold(0, |acc, &x| acc ^ x));
  }
  // temp.iter().map(|x| format!("{:02x}", x)).join("")
  temp.iter().map(|x| *x as i64).collect()
}

fn process_data(input: &str) -> Vec<i64> {
  let len = 256;
  let rounds = 64;
  let data = get_input(input);
  let mut numbers: Vec<usize> = (0..len).collect();
  let mut start = 0;
  let mut skip = 0;
  for _round in 0..rounds {
    for twist in &data {
      let mut end = start + twist;
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
  densify(&numbers)
}

fn find_groups(cells: &[[i32; 128]; 128], row: usize, col: usize, group: i32) -> [[i32; 128]; 128] {
  let mut rv = *cells;
  let mut upcoming = Vec::new();
  upcoming.push((row, col));
  while let Some(pos) = upcoming.pop() {
    rv[pos.0][pos.1] = group;
    if pos.0 > 0 && rv[pos.0 - 1][pos.1] == -1 {
      upcoming.push((pos.0 - 1, pos.1));
    }
    if pos.0 < 127 && rv[pos.0 + 1][pos.1] == -1 {
      upcoming.push((pos.0 + 1, pos.1));
    }
    if pos.1 > 0 && rv[pos.0][pos.1 - 1] == -1 {
      upcoming.push((pos.0, pos.1 - 1));
    }
    if pos.1 < 127 && rv[pos.0][pos.1 + 1] == -1 {
      upcoming.push((pos.0, pos.1 + 1));
    }
  }
  rv
}

fn process_data_a(data: &str) -> u32 {
  let mut rv = 0;
  for i in 0..128 {
    let key = format!("{}-{}", data, i);
    let value = process_data(&key);
    for curr in &value {
      rv += curr.count_ones();
    }
  }
  rv
}

fn process_data_b(data: &str) -> i32 {
  let mut rv = 0;
  let mut cells = [[0; 128]; 128];
  for (i, row) in cells.iter_mut().enumerate().take(128) {
    let key = format!("{}-{}", data, i);
    let value = process_data(&key);
    let bits = value.iter().map(|x| format!("{:08b}", x)).join("");
    for (j, curr) in enumerate(bits.chars()) {
      row[j] = if curr == '1' { -1 } else { 0 };
    }
  }
  for row in 0..128 {
    for col in 0..128 {
      if cells[row][col] == -1 {
        rv += 1;
        cells = find_groups(&cells, row, col, rv);
      }
    }
  }
  // for row in 0..128 {
  //   for col in 0..128 {
  //     let value = match cells[row][col] {
  //       -1 => "#".to_string(),
  //       0 => ".".to_string(),
  //       x => format!("{}", x % 10)
  //     };
  //     print!("{}", value);
  //   }
  //   println!();
  // }
  rv
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
  fn number(&self) -> String {
    String::from("14")
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
  assert_eq!(process_data_a("flqrgnkx"), 8108);
}

#[test]
fn b() {
  assert_eq!(process_data_b("flqrgnkx"), 1242);
}
