//-----------------------------------------------------
// Setup.

use std::collections::HashSet;

use aoc::util::{Direction, Point2};

static INPUT: &str = include_str!("data/q06.data");

type State = (
    HashSet<Point2>,
    Option<Point2>,
    Option<Point2>,
    Point2,
    Direction,
);

fn get_data(data: &str) -> State {
    let mut obstructions = HashSet::new();
    let mut curr = (-1, -1);
    let direction = Direction::North;
    let origin = Some((0, 0));
    let bounds = Some((
        data.lines().next().unwrap().chars().count() as i64,
        data.lines().count() as i64,
    ));

    for (y, line) in data.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            match cell {
                '#' => {
                    obstructions.insert((x as i64, y as i64));
                }
                '^' => {
                    curr = (x as i64, y as i64);
                }
                '.' => {}
                _ => {
                    panic!("Unknown character {} at ({},{})", cell, x, y)
                }
            }
        }
    }
    (obstructions, origin, bounds, curr, direction)
}

fn walk_path(
    mut curr: Point2,
    mut direction: Direction,
    origin: Option<Point2>,
    bounds: Option<Point2>,
    obstructions: &HashSet<Point2>,
) -> Vec<(Point2, Direction)> {
    let mut guard_path = Vec::new();
    guard_path.push((curr, direction));

    while let Some(next) = direction.move_pos(curr, 1, origin, bounds) {
        // If there is something directly in front of you, turn right 90 degrees.
        if obstructions.contains(&next) {
            direction = direction.turn_right();
        } else {
            curr = next;
            guard_path.push((curr, direction));
        }
    }
    guard_path
}

fn find_loop(
    mut direction: Direction,
    mut curr: Point2,
    origin: Option<Point2>,
    bounds: Option<Point2>,
    obstructions: &HashSet<Point2>,
    mut guard_path: Vec<(Point2, Direction)>,
) -> bool {
    let mut seen = vec![vec![[false; 4]; bounds.unwrap().0 as usize]; bounds.unwrap().1 as usize];
    for &((x, y), dir) in &guard_path {
        seen[y as usize][x as usize][dir as usize] = true;
    }
    while let Some(next) = direction.move_pos(curr, 1, origin, bounds) {
        // If we've seen this before, we're in a loop!
        if seen[next.1 as usize][next.0 as usize][direction as usize] {
            return true;
        }
        // If there is something directly in front of you, turn right 90 degrees.
        if obstructions.contains(&next) {
            direction = direction.turn_right();
        } else {
            curr = next;
            guard_path.push((curr, direction));
            seen[next.1 as usize][next.0 as usize][direction as usize] = true;
        }
    }

    false
}

fn process_data_a(data: &str) -> usize {
    let (obstructions, origin, bounds, curr, direction) = get_data(data);
    let guard_path = walk_path(curr, direction, origin, bounds, &obstructions);
    let unique: HashSet<Point2> = HashSet::from_iter(guard_path.iter().map(|(p, _)| *p));
    unique.len()
}

fn process_data_b(data: &str) -> usize {
    let (obstructions, origin, bounds, curr, direction) = get_data(data);

    let potentials = walk_path(curr, direction, origin, bounds, &obstructions);
    let mut valid = HashSet::new();
    let mut seen = HashSet::new();
    seen.insert(curr);
    for (i, &potential) in potentials.iter().enumerate() {
        if seen.contains(&potential.0) || potential.0 == curr {
            continue;
        }
        let guard_path = Vec::from_iter(potentials.clone().into_iter().take(i));
        seen.insert(potential.0);
        let (curr, direction) = guard_path[guard_path.len() - 1];

        let mut obstructions = obstructions.clone();
        obstructions.insert(potential.0);
        if find_loop(direction, curr, origin, bounds, &obstructions, guard_path) {
            valid.insert(potential);
        }
    }
    valid.len()
}

//-----------------------------------------------------
// Questions.

q_impl!("6");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "
    ....#.....
    .........#
    ..........
    ..#.......
    .......#..
    ..........
    .#..^.....
    ........#.
    #.........
    ......#...
    "
        )),
        41
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "
    ....#.....
    .........#
    ..........
    ..#.......
    .......#..
    ..........
    .#..^.....
    ........#.
    #.........
    ......#...
    "
        )),
        6
    );
}
