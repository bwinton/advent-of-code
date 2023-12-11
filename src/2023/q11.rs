//-----------------------------------------------------
// Setup.

use std::collections::HashSet;

use itertools::Itertools;

static INPUT: &str = include_str!("data/q11.data");

fn calculate_answer(data: &str, offset: usize) -> usize {
    // Doubling should only add 1.
    let offset = offset - 1;
    let mut rv = 0;
    let mut y_offset = 0;
    let mut map = HashSet::new();
    let mut min_x = usize::MAX;
    let mut max_x = 0;
    for (y, line) in data.lines().enumerate() {
        let mut found = false;
        for (x, cell) in line.chars().enumerate() {
            if cell == '#' {
                map.insert((x, y + y_offset));

                found = true;
                if x < min_x {
                    min_x = x;
                }
                if x > max_x {
                    max_x = x;
                }
            }
        }
        if !found {
            y_offset += offset;
        }
    }

    let mut empty_map = HashSet::new();
    let mut x_offset = 0;
    for x in min_x..=max_x {
        let mut found = false;
        for cell in map.iter().filter(|&cell| cell.0 == x) {
            empty_map.insert((cell.0 + x_offset, cell.1));
            found = true;
        }
        if !found {
            x_offset += offset;
        }
    }

    let map = empty_map;
    for (&a, &b) in map.iter().tuple_combinations() {
        rv += a.0.abs_diff(b.0) + a.1.abs_diff(b.1);
    }
    rv
}

fn process_data_a(data: &str) -> usize {
    calculate_answer(data, 2)
}

fn process_data_b(data: &str) -> usize {
    calculate_answer(data, 1000000)
}

//-----------------------------------------------------
// Questions.

q_impl!("11");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....
    "
        )),
        374
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        calculate_answer(
            indoc!(
                "...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....
    "
            ),
            10
        ),
        1030
    );

    assert_eq!(
        calculate_answer(
            indoc!(
                "...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....
    "
            ),
            100
        ),
        8410
    );
}
