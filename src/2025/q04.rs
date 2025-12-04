//-----------------------------------------------------
// Setup.

use std::collections::HashSet;

static INPUT: &str = include_str!("data/q04.data");

fn valid(roll: &(i32, i32), map: &HashSet<(i32, i32)>) -> bool {
    let mut count = 0;
    for x in -1..=1 {
        for y in -1..=1 {
            if map.contains(&(roll.0 - x, roll.1 - y)) {
                count += 1;
            }
        }
    }
    count -= 1; // for the roll at (0,0)
    count < 4
}

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    let mut map = HashSet::new();
    for (y, line) in data.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell == '@' {
                map.insert((x as i32, y as i32));
            }
        }
    }
    for roll in map.iter() {
        if valid(roll, &map) {
            rv += 1;
        }
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    let mut map = HashSet::new();
    for (y, line) in data.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell == '@' {
                map.insert((x as i32, y as i32));
            }
        }
    }

    let mut changed = true;
    while changed {
        changed = false;
        let mut rolls = vec![];
        for roll in map.iter() {
            if valid(roll, &map) {
                rolls.push(*roll);
            }
        }
        if !rolls.is_empty() {
            changed = true;
            rv += rolls.len();
            for roll in rolls {
                map.remove(&roll);
            }
        }
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("4");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"
        )),
        13
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"
        )),
        43
    );
}
