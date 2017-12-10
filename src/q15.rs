//-----------------------------------------------------
// Setup.

use day;

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
//
// define_iterator!(HundredIter (
//     &curr: Vec<u32> = vec![],
//     &max: usize = 100,
//     &len: usize = 0
//   ) -> Option<[i32;2]> {
//   let rv = *curr;
//
//   *remaining -= 1;
//
//   match *dir {
//     Direction::Left => {
//       curr[0] += 1;
//       if *remaining == 0 {
//         *dir = Direction::Up;
//         *remaining = *len;
//       }
//     },
//     Direction::Up => {
//       curr[1] -= 1;
//       if *remaining == 0 {
//         *dir = Direction::Right;
//         *len += 1;
//         *remaining = *len;
//       }
//     },
//     Direction::Right => {
//       curr[0] -= 1;
//       if *remaining == 0 {
//         *dir = Direction::Down;
//         *remaining = *len;
//       }
//     },
//     Direction::Down => {
//       curr[1] += 1;
//       if *remaining == 0 {
//         *dir = Direction::Left;
//         *len += 1;
//         *remaining = *len;
//       }
//     }
//   }
//
//   Some(rv)
// });

fn get_score(amounts: Vec<u32>, ingredients: &Vec<Ingredient>) -> u32 {
  for (i, amount) in amounts.iter().enumerate() {
    println!("{}, {}, {:?}", i, amount, ingredients[i]);
  }
  0
}

fn process_data_a(data: &str) -> u32 {
  let mut ingredients: Vec<Ingredient> = Vec::new();
  for line in data.lines() {
    ingredients.push(line.parse().unwrap());
  }
  println!("{:?}", ingredients);

  let mut max = 0;

  let mut iter = &(0..100);
  for i in 1..ingredients.len() {
    iter = iter.cartesian_product(&(0..(100 - i)))
  }
  for x in iter {
    println!("{:?}", x);
    // println!("{} {} {} {}", x, y, z, 100-x-y-z);
    // let score = get_score(vec![x, y, z, 100-x-y-z], &ingredients);
    // if score > max {
    //   max = score
    // }
  }

  max
}

fn process_data_b(_data: &str) -> i32 {
  0
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl day::Day for Q {
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
  assert_eq!(process_data_b(""), 0);
}
