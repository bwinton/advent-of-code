//-----------------------------------------------------
// Setup.

use aoc::util::Point2;
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{alpha1, i64, newline},
    multi::separated_list0,
    sequence::tuple,
};
use num_rational::Ratio;

#[derive(Debug, Clone)]
struct Machine {
    a: Point2,
    b: Point2,
    prize: Point2,
}

static INPUT: &str = include_str!("data/q13.data");

fn button(i: &str) -> IResult<&str, Point2> {
    // Button A: X+44, Y+17\n
    let (input, (_, _, _, x, _, y, _)) = tuple((
        tag("Button "),
        alpha1,
        tag(": X+"),
        i64,
        tag(", Y+"),
        i64,
        newline,
    ))(i)?;
    Ok((input, (x, y)))
}

fn prize(i: &str) -> IResult<&str, Point2> {
    // Prize: X=11320, Y=11922\n
    let (input, (_, x, _, y, _)) = tuple((tag("Prize: X="), i64, tag(", Y="), i64, newline))(i)?;
    Ok((input, (x, y)))
}

fn machine(i: &str) -> IResult<&str, Machine> {
    let (input, (a, b, prize)) = tuple((button, button, prize))(i)?;
    Ok((input, Machine { a, b, prize }))
}

fn parser(i: &str) -> IResult<&str, Vec<Machine>> {
    let (input, machines) = separated_list0(newline, machine)(i)?;
    Ok((input, machines))
}

fn process_data_a(data: &str) -> i64 {
    let mut rv = 0;
    let machines = parser(data).unwrap().1;
    for machine in machines {
        let slope = Ratio::new(machine.a.1, machine.a.0);
        let b = (slope * machine.prize.0 - machine.prize.1) / (slope * machine.b.0 - machine.b.1);
        let a = (-b * machine.b.0 + machine.prize.0) / machine.a.0;
        if a.is_integer() && b.is_integer() {
            rv += a.to_integer() * 3 + b.to_integer();
        }
    }
    rv
}

fn process_data_b(data: &str) -> i64 {
    let mut rv = 0;
    let mut machines = parser(data).unwrap().1;
    for machine in machines.iter_mut() {
        machine.prize.0 += 10000000000000;
        machine.prize.1 += 10000000000000;
        let slope = Ratio::new(machine.a.1, machine.a.0);
        let b = (slope * machine.prize.0 - machine.prize.1) / (slope * machine.b.0 - machine.b.1);
        let a = (-b * machine.b.0 + machine.prize.0) / machine.a.0;
        if a.is_integer() && b.is_integer() {
            rv += a.to_integer() * 3 + b.to_integer();
        }
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("13");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "
    Button A: X+94, Y+34
    Button B: X+22, Y+67
    Prize: X=8400, Y=5400

    Button A: X+26, Y+66
    Button B: X+67, Y+21
    Prize: X=12748, Y=12176

    Button A: X+17, Y+86
    Button B: X+84, Y+37
    Prize: X=7870, Y=6450

    Button A: X+69, Y+23
    Button B: X+27, Y+71
    Prize: X=18641, Y=10279
"
        )),
        480
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b(indoc!("")), 0);
}
