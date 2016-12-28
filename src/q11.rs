//-----------------------------------------------------
// Setup.

use day;
use std::str::FromStr;
use regex::Regex;

static INPUT : &'static str = "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";

#[derive(Debug)]
struct Generator {
  element: String
}

impl FromStr for Generator {
  type Err = ();

  fn from_str(s: &str) -> Result<Generator, ()> {
    let re: Regex = Regex::new(r"^a ([a-z]+) generator$").unwrap();
    let blank = String::from("");
    let mut rv = Generator{element: blank.clone()};
    let captures = re.captures(s);
    match captures {
      None => return Err(()),
      Some(cap) => {
        rv.element = String::from(cap.at(1).unwrap_or(""));
      }
    }
    return Ok(rv);
  }
}

#[derive(Debug)]
struct Microchip {
  element: String
}

impl FromStr for Microchip {
  type Err = ();

  fn from_str(s: &str) -> Result<Microchip, ()> {
    let re: Regex = Regex::new(r"^a ([a-z]+)-compatible microchip$").unwrap();
    let blank = String::from("");
    let mut rv = Microchip{element: blank.clone()};
    let captures = re.captures(s);
    match captures {
      None => return Err(()),
      Some(cap) => {
        rv.element = String::from(cap.at(1).unwrap_or(""));
      }
    }
    return Ok(rv);
  }
}

#[derive(Debug)]
enum Item {
  Generator(Generator),
  Microchip(Microchip)
}

// impl FromStr for Item {
//   type Err = ();

//   fn from_str(s: &str) -> Result<Item, ()> {
//     let re: Regex = Regex::new(r"^a ([a-z]+)-compatible microchip$").unwrap();
//     let captures = re.captures(s);
//     match captures {
//       None => return Err(()),
//       Some(cap) => {
//         return Ok(Item::Microchip(String::from(cap.at(1).unwrap_or(""))));
//       }
//     }
//   }
// }

#[derive(Debug)]
struct Floor {
  number: i32,
  items: Vec<Item>
}

impl FromStr for Floor {
  type Err = ();

  fn from_str(s: &str) -> Result<Floor, ()> {
    let re: Regex = Regex::new(r"^The ([a-z]*) floor contains (.*)\.$").unwrap();
    let mut rv = Floor{number: -1, items: Vec::new()};
    let captures = re.captures(s);
    match captures {
      None => return Err(()),
      Some(cap) => {
        println!("match: {:?}", cap);
        match cap.at(1).unwrap_or("").as_ref() {
          "first" => {rv.number = 1},
          "second" => {rv.number = 2},
          "third" => {rv.number = 3},
          "fourth" => {rv.number = 4},
          _ => {return Err(())}
        }
        let items = cap.at(2).unwrap_or("");
        println!("items1: {:?}", items);
        let item_re: Regex = Regex::new(r"a [a-z]*(:?-compatible microchip| generator)").unwrap();
        for item_captures in item_re.captures_iter(items) {
          println!("items2: {:?}", item_captures);
          let generator_opt :Result<Generator, ()> = item_captures.at(0).unwrap().parse();
          match generator_opt {
            Err(()) => {},
            Ok(value) => {
              rv.items.push(Item::Generator(value));
            }
          }
          let microchip_opt :Result<Microchip, ()> = item_captures.at(0).unwrap().parse();
          match microchip_opt {
            Err(()) => {},
            Ok(value) => {
              rv.items.push(Item::Microchip(value));
            }
          }
        }
      }
    }
    return Ok(rv);
  }
}


//-----------------------------------------------------
// Questions.

pub struct Q;

impl day::Day for Q {
  fn number(&self) -> String {
    return String::from("11");
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    let result = 0;
    for line in INPUT.lines() {
      let floor : Floor = line.parse().unwrap();
      println!("{:?}", floor);
    }

    println!("Result = {}", result);
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    let result = 0;
    println!("Result = {}", result);
  }
}
