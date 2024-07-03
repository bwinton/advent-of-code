//-----------------------------------------------------
// Setup.

use aoc::util::Point3;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{i64, newline},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

static INPUT: &str = include_str!("data/q24.data");

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    position: Point3,
    velocity: Point3,
}

fn hailstone(i: &str) -> IResult<&str, Hailstone> {
    // 19, 13, 30 @ -2,  1, -2
    let (input, (px, _, py, _, pz, _, vx, _, vy, _, vz)) = tuple((
        i64,
        tag(", "),
        i64,
        tag(", "),
        i64,
        tag(" @ "),
        i64,
        tag(", "),
        i64,
        tag(", "),
        i64,
    ))(i)?;
    let hailstone = Hailstone {
        position: (px, py, pz),
        velocity: (vx, vy, vz),
    };
    Ok((input, hailstone))
}

fn parser(i: &str) -> IResult<&str, Vec<Hailstone>> {
    let (input, gates) = separated_list1(newline, hailstone)(i)?;
    Ok((input, gates))
}

fn get_answer_a(hailstones: &[Hailstone], lower: i64, upper: i64) -> usize {
    let lower = lower as f64;
    let upper = upper as f64;
    let mut rv = 0;
    // for each pair of lines
    for (a, b) in hailstones.iter().tuple_combinations() {
        let m1 = a.velocity.1 as f64 / a.velocity.0 as f64;
        let m2 = b.velocity.1 as f64 / b.velocity.0 as f64;
        let x_denom = m1 - m2;
        if x_denom == 1.0 {
            // They're parallel, so don't meet.
            continue;
        }
        let p1 = a.position.1 as f64 - m1 * a.position.0 as f64;
        let p2 = b.position.1 as f64 - m2 * b.position.0 as f64;
        let x = (p2 - p1) / x_denom;
        let y = m1 * x + p1;
        if lower <= x
            && x <= upper
            && lower <= y
            && y <= upper
            && (x - a.position.0 as f64).signum() == (a.velocity.0 as f64).signum()
            && (x - b.position.0 as f64).signum() == (b.velocity.0 as f64).signum()
        {
            rv += 1;
        }
    }
    rv
}

fn get_answer_b(hailstones: &[Hailstone], _lower: i64, _upper: i64) -> usize {
    let mut parallels = vec![];
    for (a, b) in hailstones.iter().tuple_combinations() {
        let m1 = a.velocity.1 as f64 / a.velocity.0 as f64;
        let m2 = b.velocity.1 as f64 / b.velocity.0 as f64;
        let x_denom = m1 - m2;
        if x_denom == 0.0 {
            // They're parallel, so don't meet.
            parallels.push((a, b));
        }
    }
    // println!("parallels: {:?}", parallels);
    parallels.len()
}

fn process_data_a(data: &str) -> usize {
    let hailstones = parser(data).unwrap().1;
    get_answer_a(&hailstones, 200000000000000, 400000000000000)
}

fn process_data_b(data: &str) -> usize {
    let hailstones = parser(data).unwrap().1;
    get_answer_b(&hailstones, 200000000000000, 400000000000000)
}

//-----------------------------------------------------
// Questions.

q_impl!("24");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        get_answer_a(
            &parser(indoc!(
                "
    19, 13, 30 @ -2, 1, -2
    18, 19, 22 @ -1, -1, -2
    20, 25, 34 @ -2, -2, -4
    12, 31, 28 @ -1, -2, -1
    20, 19, 15 @ 1, -5, -3
    "
            ))
            .unwrap()
            .1,
            7,
            27
        ),
        2
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        get_answer_b(
            &parser(indoc!(
                "
    19, 13, 30 @ -2, 1, -2
    18, 19, 22 @ -1, -1, -2
    20, 25, 34 @ -2, -2, -4
    12, 31, 28 @ -1, -2, -1
    20, 19, 15 @ 1, -5, -3
    "
            ))
            .unwrap()
            .1,
            7,
            27
        ),
        47
    );
}
