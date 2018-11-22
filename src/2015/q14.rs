//-----------------------------------------------------
// Setup.

use aoc::Day;

use regex::Regex;
use std::str::FromStr;


static INPUT: &'static str = "Dancer can fly 27 km/s for 5 seconds, but then must rest for 132 seconds.
Cupid can fly 22 km/s for 2 seconds, but then must rest for 41 seconds.
Rudolph can fly 11 km/s for 5 seconds, but then must rest for 48 seconds.
Donner can fly 28 km/s for 5 seconds, but then must rest for 134 seconds.
Dasher can fly 4 km/s for 16 seconds, but then must rest for 55 seconds.
Blitzen can fly 14 km/s for 3 seconds, but then must rest for 38 seconds.
Prancer can fly 3 km/s for 21 seconds, but then must rest for 40 seconds.
Comet can fly 18 km/s for 6 seconds, but then must rest for 103 seconds.
Vixen can fly 18 km/s for 5 seconds, but then must rest for 84 seconds.";

#[derive(Debug)]
#[derive(Eq)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
struct Reindeer {
  name: String,
  speed: u32,
  duration: u32,
  rest: u32,
  points: u32,
}

impl FromStr for Reindeer {
  type Err = ();

  fn from_str(s: &str) -> Result<Reindeer, ()> {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"^([A-Za-z]+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.$").unwrap();
    }
    let captures = RE.captures(s);
    match captures {
      Some(cap) => {
        Ok(Reindeer {
          name: cap[1].to_string(),
          speed: cap[2].parse().unwrap(),
          duration: cap[3].parse().unwrap(),
          rest: cap[4].parse().unwrap(),
          points: 0,
        })
      },
      None => Err(()),
    }
  }
}

impl Reindeer {
  fn distance(&self, time: u32) -> u32 {
    let mut rv = self.speed * self.duration * (time / (self.duration + self.rest));
    let remainder = time % (self.duration + self.rest);
    rv += self.speed * remainder.min(self.duration);
    rv
  }
}

fn process_data_a(data: &str) -> u32 {
  let mut reindeers: Vec<Reindeer> = Vec::new();
  for line in data.lines() {
    reindeers.push(line.parse().unwrap());
  }
  reindeers.iter().map(|x| x.distance(2503)).max().unwrap()
}

fn process_data_b(data: &str, count: u32) -> u32 {
  let mut reindeers: Vec<Reindeer> = Vec::new();
  for line in data.lines() {
    reindeers.push(line.parse().unwrap());
  }
  for i in 1..=count {
    let lead = reindeers.iter().map(|x| x.distance(i)).max().unwrap();
    reindeers
      .iter_mut()
      .filter(|x| x.distance(i) == lead)
      .for_each(|x| x.points += 1);
  }
  reindeers.iter().map(|x| x.points).max().unwrap()
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
    let result = process_data_b(INPUT, 2503);
    println!("Result = {}", result);
  }
}

#[test]
fn a() {
  let comet: Reindeer = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds."
    .parse()
    .unwrap();
  let dancer: Reindeer = "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."
    .parse()
    .unwrap();
  assert_eq!(comet.distance(1), 14);
  assert_eq!(comet.distance(10), 140);
  assert_eq!(comet.distance(11), 140);
  assert_eq!(comet.distance(1000), 1120);
  assert_eq!(dancer.distance(1), 16);
  assert_eq!(dancer.distance(10), 160);
  assert_eq!(dancer.distance(11), 176);
  assert_eq!(dancer.distance(1000), 1056);
}

#[test]
fn b() {
  assert_eq!(
    process_data_b(
      "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
      1000,
    ),
    689
  );
}
