mod q01;
use std::env;

fn main() {
  for argument in env::args().skip(1) {
    q01::select(argument);
    println!("");
  }
}
