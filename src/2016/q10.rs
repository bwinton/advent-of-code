//-----------------------------------------------------
// Setup.

use aoc::Day;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

static INPUT: &str = include_str!("data/q10.data");
// static INPUT : &'static str = "value 5 goes to bot 2
// bot 2 gives low to bot 1 and high to bot 0
// value 3 goes to bot 1
// bot 1 gives low to output 1 and high to bot 0
// bot 0 gives low to output 2 and high to output 0
// value 2 goes to bot 2";

#[derive(Debug)]
struct Value {
    number: i32,
    bot: i32,
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Value, ()> {
        let re: Regex = Regex::new(r"^value (\d+) goes to bot (\d+)$").unwrap();
        let mut rv = Value {
            number: -1,
            bot: -1,
        };
        if let Some(x) = re.captures(s) {
            rv.number = x[1].parse().unwrap();
            rv.bot = x[2].parse().unwrap();
            Ok(rv)
        } else {
            Err(())
        }
    }
}

#[derive(Clone, Debug)]
enum Destination {
    Unknown,
    Bot(i32),
    Output(i32),
}

impl FromStr for Destination {
    type Err = ();

    fn from_str(s: &str) -> Result<Destination, ()> {
        let re: Regex = Regex::new(r"^(bot|output) (\d+)$").unwrap();
        let cap = re.captures(s);
        let mut rv = Destination::Unknown;
        if s.starts_with("bot") {
            rv = Destination::Bot(cap.unwrap()[2].parse().unwrap());
        } else if s.starts_with("output") {
            rv = Destination::Output(cap.unwrap()[2].parse().unwrap());
        }
        Ok(rv)
    }
}

#[derive(Debug)]
struct Bot {
    number: i32,
    first: Option<i32>,
    second: Option<i32>,
    low_dest: Destination,
    high_dest: Destination,
}

impl Bot {
    fn push_value(&mut self, value: i32) -> bool {
        // println!("Pushing {} to {:?}", value, self);
        match self.first {
            None => {
                self.first = Some(value);
                false
            }
            Some(value_one) => {
                if value > value_one {
                    self.second = Some(value);
                } else {
                    self.second = self.first;
                    self.first = Some(value);
                }
                true
            }
        }
    }

    fn propagate(&self, bots: &mut HashMap<i32, Bot>) -> HashMap<i32, i32> {
        // println!("Propagating {:?}", self);
        let mut rv: HashMap<i32, i32> = HashMap::new();
        match self.low_dest {
            Destination::Unknown => {
                println!(
                    "ERROR!!!  Bot {} has no low_dest!!!  {:?}",
                    self.number, self
                );
            }
            Destination::Bot(number) => {
                let mut bot = bots.get_mut(&number).unwrap().clone();
                if bot.push_value(self.first.unwrap()) {
                    rv.extend(bot.propagate(bots).iter());
                };
                bots.insert(bot.number, bot);
            }
            Destination::Output(number) => {
                // println!("Bot {} output {} to {}", self.number, self.first.unwrap(), number);
                rv.insert(number, self.first.unwrap());
            }
        };

        match self.high_dest {
            Destination::Unknown => {
                println!(
                    "ERROR!!!  Bot {} has no low_dest!!!  {:?}",
                    self.number, self
                );
            }
            Destination::Bot(number) => {
                let mut bot = bots.get_mut(&number).unwrap().clone();
                if bot.push_value(self.second.unwrap()) {
                    rv.extend(bot.propagate(bots).iter());
                };
                bots.insert(bot.number, bot);
            }
            Destination::Output(number) => {
                // println!("Bot {} output {} to {}", self.number, self.second.unwrap(), number);
                rv.insert(number, self.second.unwrap());
            }
        }
        rv
    }
}

impl Clone for Bot {
    fn clone(&self) -> Self {
        Bot {
            number: self.number,
            first: self.first,
            second: self.second,
            low_dest: self.low_dest.clone(),
            high_dest: self.high_dest.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.number = source.number;
        self.first = source.first;
        self.second = source.second;
        self.low_dest = source.low_dest.clone();
        self.high_dest = source.high_dest.clone();
    }
}

impl FromStr for Bot {
    type Err = ();

    fn from_str(s: &str) -> Result<Bot, ()> {
        let re: Regex = Regex::new(
            r"^bot (\d+) gives low to ((bot|output) \d+) and high to ((bot|output) \d+)$",
        )
        .unwrap();
        let mut rv = Bot {
            number: -1,
            first: Option::None,
            second: Option::None,
            low_dest: Destination::Unknown,
            high_dest: Destination::Unknown,
        };
        let cap = re.captures(s);
        match cap {
            None => return Err(()),
            Some(x) => {
                rv.number = x[1].parse().unwrap();
                rv.low_dest = x[2].parse().unwrap();
                rv.high_dest = x[4].parse().unwrap();
            }
        }

        if rv.number == -1 {
            Err(())
        } else {
            Ok(rv)
        }
    }
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
        let mut values: Vec<Value> = Vec::new();
        let mut bots: HashMap<i32, Bot> = HashMap::new();
        for line in INPUT.lines() {
            let value_opt: Result<Value, ()> = line.parse();
            match value_opt {
                Err(()) => {}
                Ok(value) => {
                    values.push(value);
                }
            }
            let bot_opt: Result<Bot, ()> = line.parse();
            match bot_opt {
                Err(()) => {}
                Ok(bot) => {
                    bots.insert(bot.number, bot);
                }
            }
        }
        values.sort_by(|a, b| a.number.cmp(&b.number));
        // bots.sort_by(|a, b| a.number.cmp(&b.number));

        for value in &values {
            let mut bot = bots.get_mut(&value.bot).unwrap().clone();
            if bot.push_value(value.number) {
                bot.propagate(&mut bots);
            }
            bots.insert(bot.number, bot);
        }
        // println!("\n  V:{:?}\n  B:{:?}", values, bots);
        let result = bots.values().find(|bot|
      // bot.first == Some(2) && bot.second == Some(5)
      bot.first == Some(17) && bot.second == Some(61));
        println!("Result = Bot {:?}", result.unwrap().number);
    }

    fn b(&self) {
        print!("{}B: ", self.number());
        let mut values: Vec<Value> = Vec::new();
        let mut bots: HashMap<i32, Bot> = HashMap::new();
        for line in INPUT.lines() {
            let value_opt: Result<Value, ()> = line.parse();
            match value_opt {
                Err(()) => {}
                Ok(value) => {
                    values.push(value);
                }
            }
            let bot_opt: Result<Bot, ()> = line.parse();
            match bot_opt {
                Err(()) => {}
                Ok(bot) => {
                    bots.insert(bot.number, bot);
                }
            }
        }
        values.sort_by(|a, b| a.number.cmp(&b.number));
        // bots.sort_by(|a, b| a.number.cmp(&b.number));
        let mut outputs: HashMap<i32, i32> = HashMap::new();

        for value in &values {
            let mut bot = bots.get_mut(&value.bot).unwrap().clone();
            if bot.push_value(value.number) {
                outputs.extend(bot.propagate(&mut bots).iter());
            }
            bots.insert(bot.number, bot);
        }
        // println!("\n  O:{:?}", outputs);
        let result = outputs[&0] * outputs[&1] * outputs[&2];
        println!(
            "Result = {:?}*{:?}*{:?} = {:?}",
            outputs[&0], outputs[&1], outputs[&2], result
        );
    }
}
