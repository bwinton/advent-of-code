pub mod computer;
pub mod letters;

#[macro_use]
extern crate clap;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate lazy_static;

extern crate combine;

use std::{fmt::Display, ops::Deref};

use clap::Arg;

pub trait Day {
    fn number(&self) -> String;
    fn a(&self);
    fn b(&self);
}

#[macro_export]
macro_rules! q_vec {
  ( $( $x:ident, )* ) => {
    {
      let temp_vec:Vec<Box<dyn aoc::Day>> = vec!(
      $(
        Box::new($x::Q),
      )*
      );
      temp_vec
    }
  };
  ( $( $x:ident ),* ) => {
    q_vec!{$( $x, )*}
  };
}

// Had a cool macro here from https://play.rust-lang.org/?gist=0cbc09e0fc41016f5f5c240d088a4410&version=stable
// but replaced it with `std::iter::from_fn`! 😃

#[macro_export]
macro_rules! q_impl {
    ($e:expr) => {
        use aoc::Day;

        pub struct Q;

        impl Day for Q {
            fn number(&self) -> String {
                String::from($e)
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
    };
}

pub fn print_vec(v: &[impl Display]) -> String {
    format!(
        "[{}]",
        v.iter().fold(String::new(), |acc, ref num| {
            let len = acc.is_empty();
            acc + if len { ", " } else { "" } + &num.to_string()
        })
    )
}

fn select(day: &dyn Day, arg: &str) {
    let day_num = day.number();
    match arg.to_lowercase().replace("::", "") {
        ref q if *q == format!("{}{}", day_num, "a") => day.a(),
        ref q if *q == format!("{}{}", day_num, "b") => day.b(),
        ref q if *q == day.number() => {
            day.a();
            day.b()
        }
        ref q if *q == "*" => {
            day.a();
            day.b()
        }
        _ => (),
    }
}

pub fn main(days: &[Box<dyn Day>]) {
    color_backtrace::install();
    let matches = app_from_crate!("\n")
        .arg(
            Arg::new("day")
                .about("Which day(s) to run")
                .long_about(
                    "Specify a day, or days, or parts of a day or days to run.
 Putting a number and an 'a' or 'b' will run that part for that day.
 Putting a number will run both parts for that day.
 Putting '*' (the default) will run all parts for all days.
",
                )
                .index(1)
                .multiple_values(true)
                .default_value("*"),
        )
        .get_matches();

    let args: Vec<&str> = matches.values_of("day").unwrap().collect();

    for argument in args {
        for day in days {
            select(day.deref(), argument);
        }
        println!();
    }
}
