//-----------------------------------------------------
// Setup.

use std::collections::HashMap;

use aoc::util::Point2;

static INPUT: &str = include_str!("data/q13.data");

fn parse(data: &str) -> Vec<HashMap<Point2, ()>> {
    let mut rv = vec![];

    let mut curr = HashMap::new();
    let mut y = 0;
    for line in data.lines() {
        if line.is_empty() {
            rv.push(curr);
            curr = HashMap::new();
            y = 0;
            continue;
        }
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                curr.insert((x, y), ());
            }
        }
        y += 1;
    }
    if !curr.is_empty() {
        rv.push(curr);
    }

    rv
}

fn check_horizontal(
    pattern: &HashMap<Point2, ()>,
    bounds: (usize, usize, usize, usize),
    errors: usize,
    candidate: usize,
) -> bool {
    let (min_x, min_y, max_x, max_y) = bounds;
    let delta = (candidate - min_y).min(max_y - (candidate + 1));

    let mut misses = 0;

    for y in 0..=delta {
        for x in min_x..=max_x {
            if pattern.get(&(x, candidate - y)) != pattern.get(&(x, candidate + 1 + y)) {
                misses += 1;
                if misses > errors {
                    return false;
                }
            }
        }
    }

    misses == errors
}

fn check_vertical(
    pattern: &HashMap<Point2, ()>,
    bounds: (usize, usize, usize, usize),
    errors: usize,
    candidate: usize,
) -> bool {
    let (min_x, min_y, max_x, max_y) = bounds;
    let delta = (candidate - min_x).min(max_x - (candidate + 1));

    let mut misses = 0;

    for x in 0..=delta {
        for y in min_y..=max_y {
            if pattern.get(&(candidate - x, y)) != pattern.get(&(candidate + 1 + x, y)) {
                misses += 1;
                if misses > errors {
                    return false;
                }
            }
        }
    }

    misses == errors
}

fn find_reflection(
    pattern: HashMap<Point2, ()>,
    bounds: (usize, usize, usize, usize),
    errors: usize,
) -> (Option<usize>, Option<usize>) {
    let mut h = None;
    let mut v = None;
    let (min_x, min_y, max_x, max_y) = bounds;

    for y in min_y..max_y {
        if check_horizontal(&pattern, bounds, errors, y) {
            h = Some(y + 1);
            break;
        }
    }

    for x in min_x..max_x {
        if check_vertical(&pattern, bounds, errors, x) {
            v = Some(x + 1);
            break;
        }
    }
    (h, v)
}

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    let patterns = parse(data);
    for pattern in patterns {
        let min_x = pattern.keys().map(|&(x, _y)| x).min().unwrap();
        let max_x = pattern.keys().map(|&(x, _y)| x).max().unwrap();
        let min_y = pattern.keys().map(|&(_x, y)| y).min().unwrap();
        let max_y = pattern.keys().map(|&(_x, y)| y).max().unwrap();

        let (h, v) = find_reflection(pattern, (min_x, min_y, max_x, max_y), 0);
        if let Some(h) = h {
            rv += 100 * h;
        } else if let Some(v) = v {
            rv += v;
        }
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    let patterns = parse(data);
    for pattern in patterns {
        let min_x = pattern.keys().map(|&(x, _y)| x).min().unwrap();
        let max_x = pattern.keys().map(|&(x, _y)| x).max().unwrap();
        let min_y = pattern.keys().map(|&(_x, y)| y).min().unwrap();
        let max_y = pattern.keys().map(|&(_x, y)| y).max().unwrap();

        let (h, v) = find_reflection(pattern, (min_x, min_y, max_x, max_y), 1);
        if let Some(h) = h {
            rv += 100 * h;
        } else if let Some(v) = v {
            rv += v;
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
    #.##..##.
    ..#.##.#.
    ##......#
    ##......#
    ..#.##.#.
    ..##..##.
    #.#.##.#.
    "
        )),
        5
    );
    assert_eq!(
        process_data_a(indoc!(
            "
    #...##..#
    #....#..#
    ..##..###
    #####.##.
    #####.##.
    ..##..###
    #....#..#
    "
        )),
        400
    );
    assert_eq!(
        process_data_a(indoc!(
            "#.##..##.
    ..#.##.#.
    ##......#
    ##......#
    ..#.##.#.
    ..##..##.
    #.#.##.#.
    
    #...##..#
    #....#..#
    ..##..###
    #####.##.
    #####.##.
    ..##..###
    #....#..#
    "
        )),
        405
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "
    #.##..##.
    ..#.##.#.
    ##......#
    ##......#
    ..#.##.#.
    ..##..##.
    #.#.##.#.
    "
        )),
        300
    );
    assert_eq!(
        process_data_b(indoc!(
            "
    #...##..#
    #....#..#
    ..##..###
    #####.##.
    #####.##.
    ..##..###
    #....#..#
    "
        )),
        100
    );

    assert_eq!(
        process_data_b(indoc!(
            "#.##..##.
    ..#.##.#.
    ##......#
    ##......#
    ..#.##.#.
    ..##..##.
    #.#.##.#.
    
    #...##..#
    #....#..#
    ..##..###
    #####.##.
    #####.##.
    ..##..###
    #....#..#
    "
        )),
        400
    );
}
