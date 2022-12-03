// modules
mod q01;
mod q02;
mod q03;
mod q04;

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
    let days = q_vec!(q01, q02, q03, q04,);

    aoc::main(&days)
}
