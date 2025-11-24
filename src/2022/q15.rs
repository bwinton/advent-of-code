//-----------------------------------------------------
// Setup.

use std::ops::RangeInclusive;

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
};
use range_set::RangeSet;

static INPUT: &str = include_str!("data/q15.data");

type Coord = (i64, i64);

fn sensor(i: &str) -> IResult<&str, Coord> {
    let (input, (_, x, _, y, _)) = (
        tag("Sensor at x="),
        complete::i64,
        tag(", y="),
        complete::i64,
        tag(": "),
    )
        .parse(i)?;
    Ok((input, (x, y)))
}

fn beacon(i: &str) -> IResult<&str, Coord> {
    let (input, (_, x, _, y)) = (
        tag("closest beacon is at x="),
        complete::i64,
        tag(", y="),
        complete::i64,
    )
        .parse(i)?;
    Ok((input, (x, y)))
}

fn line(i: &str) -> IResult<&str, (Coord, Coord)> {
    let (input, (a, b)) = (sensor, beacon).parse(i)?;
    Ok((input, (a, b)))
}

fn parser(i: &str) -> IResult<&str, Vec<(Coord, i64)>> {
    let (input, list) = separated_list1(line_ending, line).parse(i)?;
    let list = list
        .into_iter()
        .map(|(a, b)| (a, get_distance(&a, &b)))
        .collect();
    Ok((input, list))
}

fn process_data_a(data: &str) -> usize {
    let values = parser(data).unwrap().1;

    let row = if values[0].0.0 == 2 {
        10 // Test data.
    } else {
        2_000_000 // Real data.
    };

    // Use a RangeSet to keep track of the points, for speed.
    let mut covered: RangeSet<[RangeInclusive<i64>; 1]> = RangeSet::new();

    for (sensor, distance) in values.into_iter() {
        let distance_to_row = (sensor.1 - row).abs();
        if distance_to_row <= distance {
            let remaining = distance - distance_to_row;
            covered.insert_range((sensor.0 - remaining)..=(sensor.0 + remaining - 1));
        }
    }
    covered.iter().count()
}

fn get_distance(a: &Coord, b: &Coord) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn test_point(point: Coord, values: &[(Coord, i64)], max: i64) -> Option<Coord> {
    let mut found = true;
    if point.0 < 0 || point.0 > max || point.1 < 0 || point.1 > max {
        return None;
    }
    for &(test_sensor, test_distance) in values {
        if get_distance(&test_sensor, &point) <= test_distance {
            // This isn't the point.
            found = false;
            break;
        }
    }
    if found {
        return Some(point);
    }
    None
}

fn process_data_b(data: &str) -> i64 {
    let values = parser(data).unwrap().1;

    let max = if values[0].0.0 == 2 {
        20 // Test data.
    } else {
        4_000_000 // Real data.
    };

    let mut x = 0;
    let mut y = 0;

    // We know the missing beacon has to be one away from a sensor's range, so only look in those locations.
    'outer: for &(sensor, distance) in &values {
        let distance = distance + 1;
        for i in 0..=distance {
            let remaining = distance - i;
            if let Some(value) = test_point((sensor.0 - i, sensor.1 - remaining), &values, max) {
                (x, y) = value;
                break 'outer;
            }
            if let Some(value) = test_point((sensor.0 - i, sensor.1 + remaining), &values, max) {
                (x, y) = value;
                break 'outer;
            }
            if let Some(value) = test_point((sensor.0 + i, sensor.1 - remaining), &values, max) {
                (x, y) = value;
                break 'outer;
            }
            if let Some(value) = test_point((sensor.0 + i, sensor.1 + remaining), &values, max) {
                (x, y) = value;
                break 'outer;
            }
        }
    }

    x * 4_000_000 + y
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
