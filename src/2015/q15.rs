//-----------------------------------------------------
// Setup.

use aoc::Day;

use std;
use std::str::FromStr;
use regex::Regex;


static INPUT : &'static str = "Sprinkles: capacity 5, durability -1, flavor 0, texture 0, calories 5
PeanutButter: capacity -1, durability 3, flavor 0, texture 0, calories 1
Frosting: capacity 0, durability -1, flavor 4, texture 0, calories 6
Sugar: capacity -1, durability 0, flavor 0, texture 2, calories 8";

#[derive(Debug)]
struct Ingredient {
  name: String,
  capacity: i32,
  durability: i32,
  flavor: i32,
  texture: i32,
  calories: i32
}

impl FromStr for Ingredient {
  type Err = ();

  fn from_str(s: &str) -> Result<Ingredient, ()> {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"^([A-Za-z]+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)$").unwrap();
    }
    let captures = RE.captures(s);
    match captures {
      Some(cap) => {
        Ok(Ingredient{
          name: cap.at(1).unwrap().to_string(),
          capacity: cap.at(2).unwrap().parse().unwrap(),
          durability: cap.at(3).unwrap().parse().unwrap(),
          flavor: cap.at(4).unwrap().parse().unwrap(),
          texture: cap.at(5).unwrap().parse().unwrap(),
          calories: cap.at(6).unwrap().parse().unwrap(),
        })
      },
      None => Err(())
    }
  }
}

impl Ingredient {
  fn add(&mut self, other: &Ingredient, amount: i32) {
    self.capacity += other.capacity * amount;
    self.durability += other.durability * amount;
    self.flavor += other.flavor * amount;
    self.texture += other.texture * amount;
    self.calories += other.calories * amount;
  }

  fn floor(&mut self) {
    self.capacity = self.capacity.max(0);
    self.durability = self.durability.max(0);
    self.flavor = self.flavor.max(0);
    self.texture = self.texture.max(0);
    self.calories = self.calories.max(0);
  }
}

define_iterator!(HundredIter (
    &curr: Vec<i32> = vec![],
    &max: i32 = 100,
    &len: usize = 0
  ) -> Option<Vec<i32>> {

  let curr_len = *len - 1;
  if curr.is_empty() {
    for _ in 0..curr_len {
      curr.push(0)
    }
  } else {
    curr[curr_len - 1] += 1;
  }

  if curr[0] == *max {
    return None;
  }

  let mut rest: i32 = *max - curr.iter().sum::<i32>();

  let mut found = false;
  while rest < 0 {
    for i in 1..curr_len {
      if curr[curr_len - i] != 0 {
        found = true;
        curr[curr_len - i] = 0;
        curr[curr_len - i - 1] += 1;
        break;
      }
    }
    if !found {
      break;
    }
    rest = *max - curr.iter().sum::<i32>();
  }
  if !found && rest < 0 {
    None
  } else {
    let mut rv = curr.clone();
    rv.push(rest);

    Some(rv)
  }
});

fn get_score(amounts: &[i32], ingredients: &[Ingredient]) -> (i32, i32) {
  let mut sum = Ingredient{
    name: "Sum".to_string(),
    capacity: 0,
    durability: 0,
    flavor: 0,
    texture: 0,
    calories: 0,
  };
  for (i, amount) in amounts.iter().enumerate() {
    sum.add(&ingredients[i], *amount);
  };
  sum.floor();
  (sum.capacity * sum.durability * sum.flavor * sum.texture, sum.calories)
}

fn process_data_a(data: &str) -> i32 {
  let mut ingredients: Vec<Ingredient> = Vec::new();
  for line in data.lines() {
    ingredients.push(line.parse().unwrap());
  }

  let mut max = 0;
  let iter = HundredIter{ len: ingredients.len() as usize, ..Default::default() };
  for x in iter {
    let score = get_score(&x, &ingredients).0;
    if score > max {
      max = score
    }
  }

  max
}

fn process_data_b(data: &str) -> i32 {
  let mut ingredients: Vec<Ingredient> = Vec::new();
  for line in data.lines() {
    ingredients.push(line.parse().unwrap());
  }

  let mut max = 0;
  let iter = HundredIter{ len: ingredients.len() as usize, ..Default::default() };
  for x in iter {
    let (score, calories) = get_score(&x, &ingredients);
    if score > max && calories == 500 {
      max = score
    }
  }

  max
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
  fn number(&self) -> String {
    String::from("15")
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
  assert_eq!(process_data_a("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"), 62842880);
}

#[test]
fn b() {
  assert_eq!(process_data_b("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"), 57600000);
}
