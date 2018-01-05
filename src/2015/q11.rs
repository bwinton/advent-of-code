//-----------------------------------------------------
// Setup.

use day;

use itertools::Itertools;

static INPUT : &'static str = "hxbxwxba";

fn next_char(c: char) -> char {
  let mut rv = (c as u8 + 1) as char;
  if !valid_char(rv) {
    rv = (rv as u8 + 1) as char;
  }
  if rv > 'z' {
    rv = 'a';
  }
  rv as char
}

fn valid_char(c: char) -> bool {
  c != 'i' && c != 'l' && c != 'o'
}

fn is_valid(password: &str) -> bool {
  let mut rule_one = false;
  for (first, second, third) in password.chars().tuple_windows() {
    let a = first as u8;
    let b = second as u8;
    let c = third as u8;
    if (a + 1 == b) && (b + 1 == c) {
      rule_one = true;
      break;
    }
  }

  let rule_two = password.chars().all(valid_char);

  let mut rule_three = false;
  let mut found = None;
  for (first, second) in password.chars().tuple_windows() {
    if first == second {
      if found != None && found != Some(first) {
        rule_three = true;
        break;
      }
      found = Some(first);
    }
  }

  rule_one & rule_two & rule_three
}

fn get_next(password: &str) -> String {
  let mut rv: Vec<char> = password.chars().collect();
  let mut i = rv.len() - 1;
  rv[i] = next_char(rv[i]);
  while rv[i] == 'a' {
    i -= 1;
    rv[i] = next_char(rv[i]);
  }
  rv.into_iter().collect()
}

fn process_data(data: &str) -> String {
  let mut rv: String = data.to_owned();
  if let Some(first_invalid) = data.chars().position(|i| !valid_char(i)) {
    let mut temp: Vec<char> = data.chars().collect();
    temp[first_invalid] = (temp[first_invalid] as u8 + 1) as char;
    for item in temp.iter_mut().skip(first_invalid + 1) {
      *item = 'a';
    }
    rv = temp.into_iter().collect();
  } else {
    rv = get_next(&rv);
  }
  while !is_valid(&rv) {
    rv = get_next(&rv);
  }
  rv
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl day::Day for Q {
  fn number(&self) -> String {
    String::from("11")
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    let result = process_data(INPUT);
    println!("Result = {}", result);
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    let result = process_data(&process_data(INPUT));
    println!("Result = {}", result);
  }
}

#[test]
fn a() {
  assert!(!is_valid("hijklmmn"));
  assert!(!is_valid("abbceffg"));
  assert!(!is_valid("abbcegjk"));
  assert!(!is_valid("abcdefgh"));
  assert!(is_valid("abcdffaa"));
  assert!(!is_valid("ghijklmn"));
  assert!(is_valid("ghjaabcc"));

  assert_eq!(process_data("abcdefgh"), "abcdffaa");
  assert_eq!(process_data("ghijklmn"), "ghjaabcc");
}

#[test]
fn b() {
}
