// modules
mod q01;
mod q02;
mod q03;
mod q04;
mod q05;
mod q06;
mod q07;
mod q08;
mod q09;

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
    let days = q_vec!(q01, q02, q03, q04, q05, q06, q07, q08, q09,);

    aoc::main(&days)
}
