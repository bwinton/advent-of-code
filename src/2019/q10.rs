//-----------------------------------------------------
// Setup.

use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    f64::consts::PI,
};

use num_rational::Rational32;

static INPUT: &str = include_str!("data/q10.data");

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Angle {
    Left(Rational32),
    Right(Rational32),
    NegativeInfinity,
    Zero,
    Infinity,
}

fn get_angle(source: (i32, i32), dest: (i32, i32)) -> Angle {
    match source.0 - dest.0 {
        0 => match source.1 - dest.1 {
            x if x < 0 => Angle::NegativeInfinity,
            0 => Angle::Zero,
            _ => Angle::Infinity,
        },
        x if x < 0 => Angle::Left(Rational32::new(source.1 - dest.1, source.0 - dest.0)),
        _ => Angle::Right(Rational32::new(source.1 - dest.1, source.0 - dest.0)),
    }
}

fn get_atan(first: (i32, i32), second: (i32, i32)) -> i64 {
    let result = f64::from(second.0 - first.0).atan2(f64::from(first.1 - second.1));
    let result = (result + (2.0 * PI)) % (2.0 * PI);
    (result * 100_000f64) as i64
}

fn process_data_a(data: &str) -> usize {
    let mut map = vec![];
    let mut asteroids = vec![];
    for (y, line) in data.lines().enumerate() {
        let mut row = vec![];
        for (x, cell) in line.chars().enumerate() {
            row.push(cell == '#');
            if cell == '#' {
                asteroids.push((x as i32, y as i32));
            }
        }
        map.push(row);
    }
    let mut angles: HashMap<(i32, i32), HashSet<Angle>> = HashMap::new();
    for combo in asteroids.iter().combinations(2) {
        let source = combo[0];
        let dest = combo[1];
        angles
            .entry(*source)
            .or_insert_with(HashSet::new)
            .insert(get_angle(*source, *dest));
        angles
            .entry(*dest)
            .or_insert_with(HashSet::new)
            .insert(get_angle(*dest, *source));
    }
    angles.values().map(|x| x.len()).max().unwrap()
}

fn process_data_b(data: &str) -> i32 {
    let mut map = vec![];
    let mut asteroids = vec![];
    for (y, line) in data.lines().enumerate() {
        let mut row = vec![];
        for (x, cell) in line.chars().enumerate() {
            row.push(cell == '#');
            if cell == '#' {
                asteroids.push((x as i32, y as i32));
            }
        }
        map.push(row);
    }

    let mut angles: HashMap<(i32, i32), HashSet<Angle>> = HashMap::new();
    for combo in asteroids.iter().combinations(2) {
        let source = combo[0];
        let dest = combo[1];
        angles
            .entry(*source)
            .or_insert_with(HashSet::new)
            .insert(get_angle(*source, *dest));
        angles
            .entry(*dest)
            .or_insert_with(HashSet::new)
            .insert(get_angle(*dest, *source));
    }
    let mut max = angles.iter().next().unwrap();
    for angle in &angles {
        if angle.1.len() > max.1.len() {
            max = angle;
        }
    }
    let source = max.0;

    let mut angles: HashMap<i64, Vec<(i32, i32)>> = HashMap::new();
    for dest in asteroids.iter() {
        if dest == source {
            continue;
        }
        angles
            .entry(get_atan(*source, *dest))
            .or_insert_with(Vec::new)
            .push(*dest);
    }
    let mut keys: Vec<_> = angles.keys().collect();
    keys.sort();
    for (i, key) in keys.iter().enumerate() {
        if i == 199 {
            return angles[key][0].0 * 100 + angles[key][0].1;
        }
    }
    0
}

//-----------------------------------------------------
// Questions.

q_impl!("10");

#[test]
fn a() {
    assert_eq!(
        process_data_a(
            ".#..#
.....
#####
....#
...##"
        ),
        8
    );
    assert_eq!(
        process_data_a(
            "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"
        ),
        33
    );
    assert_eq!(
        process_data_a(
            "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###."
        ),
        35
    );
    assert_eq!(
        process_data_a(
            ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."
        ),
        41
    );
    assert_eq!(
        process_data_a(
            ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"
        ),
        210
    );
}

#[test]
fn b() {
    // To the left
    assert_eq!(get_atan((29, 28), (29, 3)), 0);
    assert_eq!(get_atan((29, 28), (29, 27)), 0);
    assert_eq!(get_atan((29, 28), (29, 11)), 0);
    assert_eq!(get_atan((29, 28), (29, 14)), 0);
    assert_eq!(get_atan((29, 28), (29, 26)), 0);
    assert_eq!(get_atan((29, 28), (29, 7)), 0);
    assert_eq!(get_atan((29, 28), (29, 23)), 0);
    assert_eq!(get_atan((29, 28), (29, 4)), 0);
    assert_eq!(get_atan((29, 28), (29, 2)), 0);

    // To the right
    assert_eq!(get_atan((29, 28), (29, 32)), 314_159);

    // To the bottom
    assert_eq!(get_atan((29, 28), (32, 28)), 157_079);

    // To the top
    assert_eq!(get_atan((29, 28), (2, 28)), 471_238);
    assert_eq!(get_atan((29, 28), (23, 28)), 471_238);
    assert_eq!(get_atan((29, 28), (20, 28)), 471_238);
    assert_eq!(get_atan((29, 28), (0, 28)), 471_238);
    assert_eq!(get_atan((29, 28), (3, 28)), 471_238);
    assert_eq!(get_atan((29, 28), (7, 28)), 471_238);

    assert_eq!(
        process_data_b(
            ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"
        ),
        802
    );
}
