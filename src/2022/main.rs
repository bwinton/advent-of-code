// modules
mod q01;

#[macro_use]
extern crate aoc;
// #[macro_use]
extern crate enumset;
#[allow(unused_imports)]
#[macro_use]
extern crate indoc;
// #[macro_use]
extern crate itertools;

fn main() {
    let days = q_vec!(q01,);

    aoc::main(&days)
}
