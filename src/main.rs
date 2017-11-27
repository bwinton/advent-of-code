mod day;

// mod q01; mod q02; mod q03; mod q04; mod q05; mod q06; mod q07; mod q08; mod q09; mod q10;
// mod q11; mod q12; mod q13; mod q14; mod q15; mod q16; mod q17; mod q18; mod q19; mod q20;
// mod q21; mod q22; mod q23; mod q24; mod q25;

use std::env;
use std::ops::Deref;

#[macro_use] extern crate itertools;
#[macro_use] extern crate lazy_static;

extern crate crypto;
extern crate regex;

macro_rules! q_vec {
  ( $( $x:ident ),* ) => {
    {
      let temp_vec:Vec<Box<day::Day>> = vec!(
      $(
        Box::new($x::Q),
      )*
      );
      temp_vec
    }
  };
}

pub fn select(day: &day::Day, arg: &String) {
  let day_num = day.number();
  match arg.as_ref() {
    q if q == format!("{}{}", day_num, "a") => day.a(),
    q if q == format!("{}{}", day_num, "b") => day.b(),
    q if q == day.number() => {day.a(); day.b()}
    "*" => {day.a(); day.b()},
    _ => ()
  }
}

fn main() {
  let mut args: Vec<_> = env::args().skip(1).collect();
  if args.len() == 0 {
    args = vec![String::from("*")];
  }

  let days = q_vec!(
    // q01, q02, q03, q04, q05, q06, q07, q08, q09, q10,
    // q11, q12, q13, q14, q15, q16, q17, q18, q19, q20,
    // q21, q22, q23, q24, q25
    );

  for argument in args {
    for day in &days {
      select(day.deref(), &argument);
    }
    println!("");
  }
}
