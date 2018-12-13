#![feature(repeat_generic_slice)]
#![feature(test)]

mod q01;
mod q02;
mod q03;
mod q04;
mod q05;
mod q06;
mod q07;
mod q08;
mod q09;
mod q10;
mod q11;
mod q12;
mod q13;
// mod q14;
// mod q15;
// mod q16;
// mod q17;
// mod q18;
// mod q19;
// mod q20;
// mod q21;
// mod q22;
// mod q23;
// mod q24;
// mod q25;

#[macro_use]
extern crate aoc;
#[macro_use]
extern crate lazy_static;
// #[macro_use]
// extern crate maplit;
// #[macro_use]
// extern crate nom;

extern crate test;

fn main() {
    let days = q_vec!(
        q01, q02, q03, q04, q05, q06, q07, q08, q09, q10, q11,
        q12,
        q13,
        //q14,
        //q15,
        //q16,
        //q17,
        //q18,
        //q19,
        //q20,
        //q21,
        //q22,
        //q23,
        //q24,
        //q25
    );

    aoc::main(&days)
}
