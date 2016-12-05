mod q01;
mod q02;
mod q03;
mod q04;
mod q05;
use std::env;

#[macro_use] extern crate itertools;

extern crate crypto;
extern crate regex;

fn main() {
  let mut args: Vec<_> = env::args().skip(1).collect();
  if args.len() == 0 {
    args = vec![String::from("*")];
  }
  for argument in args {
    q01::select(&argument);
    q02::select(&argument);
    q03::select(&argument);
    q04::select(&argument);
    q05::select(&argument);
    println!("");
  }
}
