#![feature(exact_size_is_empty)]

// modules
mod q01;

#[macro_use]
extern crate aoc;
extern crate enumset;
#[allow(unused_imports)]
#[macro_use]
extern crate indoc;
extern crate itertools;

fn main() {
    let days = q_vec!(q01);

    aoc::main(&days)
}
