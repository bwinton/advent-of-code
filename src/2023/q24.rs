//-----------------------------------------------------
// Setup.

use std::collections::HashSet;

use aoc::util::Point3;

use itertools::Itertools;
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{i64, newline},
    multi::separated_list1,
    sequence::tuple,
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

fn get_answer_b(hailstones: &[Hailstone]) -> i64 {
    let mut potential_xs: Option<HashSet<i64>> = None;
    let mut potential_ys = None;
    let mut potential_zs = None;

    for (a, b) in hailstones.iter().tuple_combinations() {
        if a.velocity.0 == b.velocity.0 && a.velocity.0.abs() > 100 {
            let mut new_xs = HashSet::new();
            let difference = b.position.0 - a.position.0;
            for v in -1000..1000 {
                if v == a.velocity.0 {
                    continue;
                }
                if difference % (v - a.velocity.0) == 0 {
                    new_xs.insert(v);
                }
            }
            if let Some(potentials) = potential_xs {
                potential_xs = Some(&potentials & &new_xs);
            } else {
                potential_xs = Some(new_xs);
            }
        }

        if a.velocity.1 == b.velocity.1 && a.velocity.1.abs() > 100 {
            let mut new_ys = HashSet::new();
            let difference = b.position.1 - a.position.1;
            for v in -1000..1000 {
                if v == a.velocity.1 {
                    new_ys.insert(v);
                    continue;
                }
                if difference % (v - a.velocity.1) == 0 {
                    new_ys.insert(v);
                }
            }
            if let Some(potentials) = potential_ys {
                potential_ys = Some(&potentials & &new_ys);
            } else {
                potential_ys = Some(new_ys);
            }
        }

        if a.velocity.2 == b.velocity.2 && a.velocity.2.abs() > 100 {
            let mut new_zs = HashSet::new();
            let difference = b.position.2 - a.position.2;
            for v in -1000..1000 {
                if v == a.velocity.2 {
                    continue;
                }
                if difference % (v - a.velocity.2) == 0 {
                    new_zs.insert(v);
                }
            }
            if let Some(potentials) = potential_zs {
                potential_zs = Some(&potentials & &new_zs);
            } else {
                potential_zs = Some(new_zs);
            }
        }
    }
    println!("{:?}, {:?}, {:?}", potential_xs, potential_ys, potential_zs);

    let velocity: Point3 = (
        *potential_xs.unwrap().iter().next().unwrap(),
        *potential_ys.unwrap().iter().next().unwrap(),
        *potential_zs.unwrap().iter().next().unwrap(),
    );
    let a = hailstones[0];
    let b = hailstones[1];

    let a_slope = (a.velocity.1 - velocity.1) as f64 / (a.velocity.0 - velocity.0) as f64;
    let b_slope = (b.velocity.1 - velocity.1) as f64 / (b.velocity.0 - velocity.0) as f64;
    let ca = a.position.1 as f64 - (a_slope * a.position.0 as f64);
    let cb = b.position.1 as f64 - (b_slope * b.position.0 as f64);
    let x_pos = ((cb - ca) / (a_slope - b_slope)) as i64;
    let y_pos = (a_slope * x_pos as f64 + ca) as i64;
    let time = (x_pos - a.position.0) / (a.velocity.0 - velocity.0);
    let z_pos = a.position.2 + (a.velocity.2 - velocity.2) * time;
    x_pos + y_pos + z_pos
}

fn process_data_a(data: &str) -> usize {
    let hailstones = parser(data).unwrap().1;
    get_answer_a(&hailstones, 200000000000000, 400000000000000)
}

fn process_data_b(data: &str) -> i64 {
    let hailstones = parser(data).unwrap().1;
    get_answer_b(&hailstones)
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
    // use pretty_assertions::assert_eq;
    // assert_eq!(
    //     get_answer_b(
    //         &parser(indoc!(
    //             "
    // 19, 13, 30 @ -2, 1, -2
    // 18, 19, 22 @ -1, -1, -2
    // 20, 25, 34 @ -2, -2, -4
    // 12, 31, 28 @ -1, -2, -1
    // 20, 19, 15 @ 1, -5, -3
    // "
    //         ))
    //         .unwrap()
    //         .1
    //     ),
    //     47
    // );
}
