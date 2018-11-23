//-----------------------------------------------------
// Setup.

use aoc::Day;

// static INPUT : usize = 5;
static INPUT: usize = 3_014_603;

fn get_result_a() -> usize {
    let mut n = 2;
    while n < INPUT {
        n *= 2;
    }
    let l = INPUT - (n / 2);
    2 * l + 1
}

fn get_result_b() -> usize {
    let mut n = 3;
    while n < INPUT {
        n *= 3;
    }
    let mut l = INPUT - (n / 3);
    let mut m = 0;
    if l > n / 3 {
        m = l - n / 3;
        l -= m;
    }

    l + 2 * m
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
    fn number(&self) -> String {
        String::from("19")
    }

    fn a(&self) {
        print!("{}A: ", self.number());
        println!("Result = {}", get_result_a());
    }

    fn b(&self) {
        print!("{}B: ", self.number());
        println!("Result = {}", get_result_b());
    }
}
