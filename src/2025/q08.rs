//-----------------------------------------------------
// Setup.

use aoc::util::{Point3, distance_3};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("data/q08.data");

type Circuits = Vec<HashSet<Point3>>;
type Distances = HashMap<i64, Vec<(Point3, Point3)>>;

fn parse(data: &str) -> (Circuits, Distances) {
    let mut boxes = vec![];
    for line in data.lines() {
        let curr: Vec<i64> = line.splitn(3, ",").map(|n| n.parse().unwrap()).collect();
        boxes.push((curr[0], curr[1], curr[2]));
    }
    let circuits: Vec<HashSet<Point3>> = boxes.iter().map(|&b| HashSet::from_iter([b])).collect();
    let mut distances: HashMap<i64, Vec<_>> = HashMap::new();
    for curr in boxes.into_iter().combinations(2) {
        let distance = distance_3(curr[0], curr[1]);
        distances
            .entry(distance)
            .or_default()
            .push((curr[0], curr[1]));
    }
    (circuits, distances)
}

fn merge(circuits: &mut Circuits, first: &Point3, second: &Point3) {
    // find the vec that contains the second, and move all of its items into the vec that contains the first.
    let first_index = circuits
        .iter()
        .find_position(|boxes| boxes.contains(first))
        .unwrap()
        .0;
    let second_index = circuits
        .iter()
        .find_position(|boxes| boxes.contains(second))
        .unwrap()
        .0;
    if first_index != second_index {
        let additions = circuits[second_index].clone();
        circuits[first_index].extend(&additions);
        circuits.remove(second_index);
    }
}

fn process_a(data: &str, count: usize) -> usize {
    let (mut circuits, distances) = parse(data);
    let mut processed = count;
    'outer: for distance in distances.keys().sorted() {
        let pairs = distances.get(distance).unwrap();
        for (first, second) in pairs {
            merge(&mut circuits, first, second);
            processed -= 1;
            if processed == 0 {
                break 'outer;
            }
        }
    }
    circuits
        .iter()
        .map(|c| c.len())
        .sorted()
        .rev()
        .take(3)
        .product()
}

fn process_data_a(data: &str) -> usize {
    process_a(data, 1000)
}

fn process_data_b(data: &str) -> i64 {
    let mut rv = 0;
    let (mut circuits, distances) = parse(data);
    'outer: for distance in distances.keys().sorted() {
        let pairs = distances.get(distance).unwrap();
        for (first, second) in pairs {
            merge(&mut circuits, first, second);
            if circuits.len() == 1 {
                rv = first.0 * second.0;
                break 'outer;
            }
        }
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("8");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_a(
            indoc!(
                "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
            ),
            10
        ),
        40
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
        )),
        25_272
    );
}
