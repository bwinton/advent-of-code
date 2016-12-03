mod q01;
mod q02;
use std::env;

fn main() {
  let mut args: Vec<_> = env::args().skip(1).collect();
  if args.len() == 0 {
    args = vec![String::from("*")];
  }
  for argument in args {
    q01::select(&argument);
    q02::select(&argument);
    println!("");
  }
}
