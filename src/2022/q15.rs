//-----------------------------------------------------
// Setup.

use std::{collections::HashSet, ops::RangeInclusive};

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

use range_set::RangeSet;

static INPUT: &str = include_str!("data/q15.data");

type Coord = (i64, i64);

// Sensor at x=3844106, y=3888618: closest beacon is at x=3225436, y=4052707

fn sensor(i: &str) -> IResult<&str, Coord> {
    let (input, (_, x, _, y, _)) = tuple((
        tag("Sensor at x="),
        complete::i64,
        tag(", y="),
        complete::i64,
        tag(": "),
    ))(i)?;
    Ok((input, (x, y)))
}

fn beacon(i: &str) -> IResult<&str, Coord> {
    let (input, (_, x, _, y)) = tuple((
        tag("closest beacon is at x="),
        complete::i64,
        tag(", y="),
        complete::i64,
    ))(i)?;
    Ok((input, (x, y)))
}

fn line(i: &str) -> IResult<&str, (Coord, Coord)> {
    let (input, (a, b)) = tuple((sensor, beacon))(i)?;
    Ok((input, (a, b)))
}

fn parser(i: &str) -> IResult<&str, Vec<(Coord, Coord)>> {
    let (input, list) = separated_list1(line_ending, line)(i)?;
    Ok((input, list))
}

fn process_data_a(data: &str) -> usize {
    let values = parser(data).unwrap().1;

    let row = if values[0].0 .0 == 2 {
        10 // Test data.
    } else {
        2_000_000 // Real data.
    };

    let mut covered = HashSet::new();

    for (sensor, beacon) in values.iter() {
        let distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
        let distance_to_row = (sensor.1 - row).abs();
        if distance_to_row <= distance {
            let remaining = distance - distance_to_row;
            for x in (sensor.0 - remaining)..(sensor.0 + remaining) {
                covered.insert(x);
            }
        }
    }
    covered.len()
}

fn process_data_b(data: &str) -> i64 {
    let values = parser(data).unwrap().1;

    let max: i64 = if values[0].0 .0 == 2 {
        20 // Test data.
    } else {
        4_000_000 // Real data.
    };

    let mut available =
        vec![RangeSet::<[RangeInclusive<i64>; 1]>::from(0..=max); (max + 1) as usize];

    for (sensor, beacon) in values.iter() {
        let distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
        for row in sensor.1 - distance..=sensor.1 + distance {
            if row < 0 || row > max {
                continue;
            }
            let distance_to_row = (sensor.1 - row).abs();
            let remaining = distance - distance_to_row;

            available[row as usize].remove_range((sensor.0 - remaining)..=(sensor.0 + remaining));
        }
    }

    let y = available
        .iter()
        .position(|range| !range.is_empty())
        .unwrap();
    let x = available[y].iter().next().unwrap();

    x * 4_000_000 + y as i64
}

//-----------------------------------------------------
// Questions.

q_impl!("15");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    Sensor at x=9, y=16: closest beacon is at x=10, y=16
    Sensor at x=13, y=2: closest beacon is at x=15, y=3
    Sensor at x=12, y=14: closest beacon is at x=10, y=16
    Sensor at x=10, y=20: closest beacon is at x=10, y=16
    Sensor at x=14, y=17: closest beacon is at x=10, y=16
    Sensor at x=8, y=7: closest beacon is at x=2, y=10
    Sensor at x=2, y=0: closest beacon is at x=2, y=10
    Sensor at x=0, y=11: closest beacon is at x=2, y=10
    Sensor at x=20, y=14: closest beacon is at x=25, y=17
    Sensor at x=17, y=20: closest beacon is at x=21, y=22
    Sensor at x=16, y=7: closest beacon is at x=15, y=3
    Sensor at x=14, y=3: closest beacon is at x=15, y=3
    Sensor at x=20, y=1: closest beacon is at x=15, y=3
    "
        )),
        26
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    Sensor at x=9, y=16: closest beacon is at x=10, y=16
    Sensor at x=13, y=2: closest beacon is at x=15, y=3
    Sensor at x=12, y=14: closest beacon is at x=10, y=16
    Sensor at x=10, y=20: closest beacon is at x=10, y=16
    Sensor at x=14, y=17: closest beacon is at x=10, y=16
    Sensor at x=8, y=7: closest beacon is at x=2, y=10
    Sensor at x=2, y=0: closest beacon is at x=2, y=10
    Sensor at x=0, y=11: closest beacon is at x=2, y=10
    Sensor at x=20, y=14: closest beacon is at x=25, y=17
    Sensor at x=17, y=20: closest beacon is at x=21, y=22
    Sensor at x=16, y=7: closest beacon is at x=15, y=3
    Sensor at x=14, y=3: closest beacon is at x=15, y=3
    Sensor at x=20, y=1: closest beacon is at x=15, y=3
    "
        )),
        56000011
    );
}
