#![feature(exact_size_is_empty)]
#![feature(destructuring_assignment)]

// modules
mod q01;
mod q02;
mod q03;
mod q04;
mod q05;

#[macro_use]
extern crate aoc;
// #[macro_use]
extern crate enumset;
#[allow(unused_imports)]
#[macro_use]
extern crate indoc;
// #[macro_use]
extern crate itertools;
// #[macro_use]
extern crate lazy_static;

fn main() {
    let days = q_vec!(q01, q02, q03, q04, q05,);

    aoc::main(&days)
}
