//-----------------------------------------------------
// Setup.

use std::collections::HashSet;

use itertools::Itertools;

static INPUT: &str = include_str!("data/q18.data");

fn adjacent(a: &[usize], b: &[usize]) -> bool {
    let mut diffs = 0;
    for (&x, &y) in a.iter().zip(b) {
        if x != y {
            if x == y + 1 || y == x + 1 {
                diffs += 1;
            } else {
                // They differ by more than one.
                return false;
            }
        }
    }
    diffs == 1
}

fn process_data_a(data: &str) -> usize {
    let cubes: Vec<Vec<usize>> = data
        .lines()
        .map(|line| line.split(',').map(|i| i.parse().unwrap()).collect())
        .collect();
    // Each cube has 6 faces.
    let mut rv = cubes.len() * 6;
    // Find adjacent cubes.
    for (a, b) in cubes.iter().tuple_combinations() {
        if adjacent(a, b) {
            rv -= 2;
        }
    }
    rv
}

fn get_adjacent(curr: &[i32]) -> Vec<Vec<i32>> {
    vec![
        // X
        vec![curr[0] - 1, curr[1], curr[2]],
        vec![curr[0] + 1, curr[1], curr[2]],
        // Y
        vec![curr[0], curr[1] - 1, curr[2]],
        vec![curr[0], curr[1] + 1, curr[2]],
        // Z
        vec![curr[0], curr[1], curr[2] - 1],
        vec![curr[0], curr[1], curr[2] + 1],
    ]
}

fn process_data_b(data: &str) -> usize {
    let cubes: Vec<Vec<i32>> = data
        .lines()
        .map(|line| line.split(',').map(|i| i.parse().unwrap()).collect())
        .collect();
    let mut rv = 0;
    let mut min = vec![20, 20, 20];
    let mut max = vec![0, 0, 0];
    for cube in &cubes {
        for (i, &coord) in cube.iter().enumerate() {
            if coord <= min[i] {
                // Add an extra row on the front.
                min[i] = coord - 1;
            }
            if coord >= max[i] {
                // Add an extra row on the end.
                max[i] = coord + 1;
            }
        }
    }
    let mut stack = vec![vec![0, 0, 0]];
    let mut seen = HashSet::new();
    while !stack.is_empty() {
        let curr = stack.pop().unwrap();
        if seen.contains(&curr) {
            continue;
        }
        seen.insert(curr.clone());

        for next in get_adjacent(&curr) {
            if !(min[0]..=max[0]).contains(&next[0])
                || !(min[1]..=max[1]).contains(&next[1])
                || !(min[2]..=max[2]).contains(&next[2])
            {
                // If the move is invalid, skip it.
                continue;
            }

            if cubes.contains(&next) {
                rv += 1;
            } else if !seen.contains(&next) {
                stack.push(next);
            }
        }
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("18");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    // assert_eq!(process_data_a(indoc!("1,1,1
    // 2,1,1
    // ")), 10);
    assert_eq!(
        process_data_a(indoc!(
            "2,2,2
    1,2,2
    3,2,2
    2,1,2
    2,3,2
    2,2,1
    2,2,3
    2,2,4
    2,2,6
    1,2,5
    3,2,5
    2,1,5
    2,3,5
    "
        )),
        64
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "2,2,2
    1,2,2
    3,2,2
    2,1,2
    2,3,2
    2,2,1
    2,2,3
    2,2,4
    2,2,6
    1,2,5
    3,2,5
    2,1,5
    2,3,5
    "
        )),
        58
    );
}
