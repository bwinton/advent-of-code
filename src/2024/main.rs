#![feature(exact_size_is_empty)]

// modules
mod q01;
mod q02;
mod q03;

#[macro_use]
extern crate aoc;
extern crate enumset;
#[allow(unused_imports)]
#[macro_use]
extern crate indoc;
extern crate itertools;

fn main() {
    let days = q_vec!(q01, q02, q03,);

    aoc::main(&days)
}
