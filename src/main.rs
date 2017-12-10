#[macro_use] mod macros;
mod day;

mod q01; mod q02; mod q03; mod q04; mod q05; mod q06; mod q07; mod q08; mod q09;
mod q10;// mod q11; mod q12; mod q13; mod q14; mod q15; mod q16; mod q17; mod q18; mod q19;
// mod q20; mod q21; mod q22; mod q23; mod q24; mod q25;

use std::ops::Deref;
use clap::Arg;

#[macro_use] extern crate clap;
#[macro_use] extern crate lazy_static;

#[cfg(test)]
#[macro_use] extern crate maplit;

extern crate crypto;
extern crate itertools;
extern crate regex;

pub fn select(day: &day::Day, arg: &str) {
  let day_num = day.number();
  match arg.to_lowercase() {
    ref q if *q == format!("{}{}", day_num, "a") => day.a(),
    ref q if *q == format!("{}{}", day_num, "b") => day.b(),
    ref q if *q == day.number() => {day.a(); day.b()},
    ref q if *q == "*" => {day.a(); day.b()},
    _ => ()
  }
}

fn main() {
  let matches = app_from_crate!("\n")
    .arg(Arg::with_name("day")
      .help("Which day(s) to run")
      .long_help("Specify a day, or days, or parts of a day or days to run.
 Putting a number and an 'a' or 'b' will run that part for that day.
 Putting a number will run both parts for that day.
 Putting '*' (the default) will run all parts for all days.
")
      .index(1)
      .multiple(true)
      .default_value("*"))
    .get_matches();

  let args: Vec<&str> = matches.values_of("day").unwrap().collect();

  let days = q_vec!(
    q01, q02, q03, q04, q05, q06, q07, q08, q09,
    q10//, q11, q12, q13, q14, q15, q16, q17, q18, q19,
    // q20, q21, q22, q23, q24, q25
    );

  for argument in args {
    for day in &days {
      select(day.deref(), argument);
    }
    println!();
  }
}
