//-----------------------------------------------------
// Setup.

use std::collections::{HashMap, VecDeque};

use aoc::util::Point2;
use itertools::Itertools;

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Move {
    North,
    East,
    South,
    West,
    Activate,
}

static INPUT: &str = include_str!("data/q21.data");

fn parse(data: &str) -> Vec<(Vec<usize>, usize)> {
    let mut codes = vec![];
    for line in data.lines() {
        let mut numbers = vec![];
        let mut mult = 0;
        for c in line.chars() {
            if c.is_numeric() {
                let c = c.to_digit(10).unwrap() as usize;
                numbers.push(c);
                mult *= 10;
                mult += c;
                continue;
            }
            if c != 'A' {
                panic!("Unknown character {}!", c);
            }
        }
        codes.push((numbers, mult));
    }
    codes
}

fn get_point(a: usize) -> Point2 {
    match a {
        0 => (1, 3),
        10 => (2, 3), // Activate
        1 => (0, 2),
        2 => (1, 2),
        3 => (2, 2),
        4 => (0, 1),
        5 => (1, 1),
        6 => (2, 1),
        7 => (0, 0),
        8 => (1, 0),
        9 => (2, 0),
        _ => panic!("Unknown point {}!", a),
    }
}

fn get_meta_point(a: Move) -> Point2 {
    match a {
        Move::North => (1, 0),
        Move::Activate => (2, 0),
        Move::West => (0, 1),
        Move::South => (1, 1),
        Move::East => (2, 1),
    }
}

fn get_paths(a: Point2, b: Point2, base: bool) -> Vec<Vec<Move>> {
    let gap = if base { (0, 3) } else { (0, 0) };
    let mut rv = vec![];
    let mut queue = VecDeque::new();
    queue.push_back((a, vec![]));
    while let Some(((x, y), mut path)) = queue.pop_front() {
        if (x, y) == b {
            path.push(Move::Activate);
            rv.push(path);
            continue;
        }
        if b.1 < y && !(gap.0 == x && gap.1 < y && gap.1 >= b.1) {
            let mut new_path = path.clone();
            new_path.extend([Move::North].repeat((y - b.1) as usize));
            queue.push_back(((x, b.1), new_path));
        }
        if b.0 < x && !(gap.1 == y && gap.0 < x && gap.0 >= b.0) {
            let mut new_path = path.clone();
            new_path.extend([Move::West].repeat((x - b.0) as usize));
            queue.push_back(((b.0, y), new_path));
        }
        if b.0 > x && !(gap.1 == y && gap.0 > x && gap.0 <= b.0) {
            let mut new_path = path.clone();
            new_path.extend([Move::East].repeat((b.0 - x) as usize));
            queue.push_back(((b.0, y), new_path));
        }
        if b.1 > y && !(gap.0 == x && gap.1 > y && gap.1 <= b.1) {
            let mut new_path = path.clone();
            new_path.extend([Move::South].repeat((b.1 - y) as usize));
            queue.push_back(((x, b.1), new_path));
        }
    }
    rv
}

fn get_path_len(
    moves: Vec<usize>,
    max_depth: usize,
    cache: &mut HashMap<(usize, Vec<usize>), usize>,
    meta_cache: &mut HashMap<(usize, Vec<Move>), usize>,
) -> usize {
    if let Some(&cached) = cache.get(&(0, moves.clone())) {
        return cached;
    }
    let mut path = moves.clone();
    path.insert(0, 10); // Activate
    path.push(10); // Activate
    let rv = path
        .iter()
        .tuple_windows()
        .map(|(&a, &b)| {
            let start = get_point(a);
            let end = get_point(b);
            get_paths(start, end, true)
                .into_iter()
                .map(|path| get_meta_path_len(path, 1, max_depth, meta_cache))
                .min()
                .unwrap()
        })
        .sum();

    cache.insert((0, moves), rv);
    rv
}

fn get_meta_path_len(
    moves: Vec<Move>,
    depth: usize,
    max_depth: usize,
    cache: &mut HashMap<(usize, Vec<Move>), usize>,
) -> usize {
    if let Some(&cached) = cache.get(&(depth, moves.clone())) {
        return cached;
    }

    let mut path = moves.clone();
    path.insert(0, Move::Activate);
    let rv = path
        .iter()
        .tuple_windows()
        .map(|(&a, &b)| {
            let start = get_meta_point(a);
            let end = get_meta_point(b);
            let paths = get_paths(start, end, false);
            if depth == max_depth {
                paths.iter().map(|v| v.len()).min().unwrap()
            } else {
                paths
                    .into_iter()
                    .map(|path| get_meta_path_len(path, depth + 1, max_depth, cache))
                    .min()
                    .unwrap()
            }
        })
        .sum();

    cache.insert((depth, moves), rv);
    rv
}

fn process(codes: &[(Vec<usize>, usize)], depth: usize) -> usize {
    let mut cache = HashMap::new();
    let mut meta_cache = HashMap::new();
    let mut rv = 0;
    for (code, mult) in codes {
        let mut len = 0;
        let curr_len = get_path_len(code.clone(), depth, &mut cache, &mut meta_cache);
        len += curr_len;
        rv += len * mult;
    }
    rv
}

fn process_data_a(data: &str) -> usize {
    let codes = parse(data);
    process(&codes, 2)
}

fn process_data_b(data: &str) -> usize {
    let codes = parse(data);
    process(&codes, 25)
}

//-----------------------------------------------------
// Questions.

q_impl!("21");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_a("029A"), 68 * 29);
    assert_eq!(process_data_a("980A"), 60 * 980);
    assert_eq!(process_data_a("179A"), 68 * 179);
    assert_eq!(process_data_a("456A"), 64 * 456);
    assert_eq!(process_data_a("379A"), 64 * 379);

    assert_eq!(
        process_data_a(indoc!(
            "
            029A
            980A
            179A
            456A
            379A"
        )),
        126_384
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b("029A"), 82_050_061_710 * 29);
    assert_eq!(process_data_b("980A"), 72_242_026_390 * 980);
    assert_eq!(process_data_b("179A"), 81_251_039_228 * 179);
    assert_eq!(process_data_b("456A"), 80_786_362_258 * 456);
    assert_eq!(process_data_b("379A"), 77_985_628_636 * 379);

    assert_eq!(
        process_data_b(indoc!(
            "
            029A
            980A
            179A
            456A
            379A"
        )),
        154_115_708_116_294
    );
}
