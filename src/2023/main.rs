// modules
mod q01;
mod q02;
mod q03;
mod q04;
mod q05;

#[macro_use]
extern crate aoc;

#[allow(unused_imports)]
#[macro_use]
extern crate enumset;
#[allow(unused_imports)]
#[macro_use]
extern crate indoc;
#[allow(unused_imports)]
#[macro_use]
extern crate itertools;

fn main() {
    let days = q_vec!(q01, q02, q03, q04, q05,);

    aoc::main(&days)
}