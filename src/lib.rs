pub mod computer;

#[macro_use]
extern crate clap;
#[macro_use]
extern crate derive_more;

extern crate combine;

use std::fmt::Display;
use std::ops::Deref;

use clap::Arg;

pub trait Day {
    fn number(&self) -> String;
    fn a(&self);
    fn b(&self);
}

#[macro_export]
macro_rules! q_vec {
  ( $( $x:ident ),* ) => {
    {
      let temp_vec:Vec<Box<aoc::Day>> = vec!(
      $(
        Box::new($x::Q),
      )*
      );
      temp_vec
    }
  };
}

// From https://play.rust-lang.org/?gist=0cbc09e0fc41016f5f5c240d088a4410&version=stable
#[macro_export]
macro_rules! define_iterator {
  ($itname:ident ($(&$name:ident : $ty:ty = $e:expr),*) -> Option<$rty:ty> { $($body:tt)* }) => {
    struct $itname {
      $($name : $ty),*
    }

    impl std::default::Default for $itname {
      fn default() -> Self {
        $itname {
          $($name : $e),*
        }
      }
    }

    impl Iterator for $itname {
      type Item = $rty;
      fn next(&mut self) -> Option<Self::Item> {
        $(let $name : &mut $ty = &mut self.$name;)*
        $($body)*
      }
    }
  }
}

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

    use test::Bencher;

    #[ignore]
    #[bench]
    fn bench_a(b: &mut Bencher) {
      let day = Q{};
      b.iter(|| day.a());
    }

    #[ignore]
    #[bench]
    fn bench_b(b: &mut Bencher) {
      let day = Q{};
      b.iter(|| day.b());
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

fn select(day: &Day, arg: &str) {
    let day_num = day.number();
    match arg.to_lowercase() {
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

pub fn main(days: &[Box<Day>]) {
    let matches = app_from_crate!("\n")
        .arg(
            Arg::with_name("day")
                .help("Which day(s) to run")
                .long_help(
                    "Specify a day, or days, or parts of a day or days to run.
 Putting a number and an 'a' or 'b' will run that part for that day.
 Putting a number will run both parts for that day.
 Putting '*' (the default) will run all parts for all days.
",
                )
                .index(1)
                .multiple(true)
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
