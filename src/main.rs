mod q01;
mod q02;
mod q03;
use std::env;

#[macro_use] extern crate itertools;

fn main() {
  let mut args: Vec<_> = env::args().skip(1).collect();
  if args.len() == 0 {
    args = vec![String::from("*")];
  }
  for argument in args {
    q01::select(&argument);
    q02::select(&argument);
    q03::select(&argument);
    println!("");
  }
}
