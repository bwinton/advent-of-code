//-----------------------------------------------------
// Setup.

use std::collections::HashSet;

use aoc::util::Direction;

static INPUT: &str = include_str!("data/q06.data");

fn get_data(data: &str) -> (HashSet<(i64, i64)>, Option<(i64, i64)>, Option<(i64, i64)>, (i64, i64), Direction) {
    let mut obstructions = HashSet::new();
    let mut curr = (-1,-1);
    let direction = Direction::North;
    let origin = Some((0,0));
    let bounds = Some((
        data.lines().next().unwrap().chars().count() as i64,
        data.lines().count() as i64));

    for (y, line) in data.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            match cell {
                '#' => {
                    obstructions.insert((x as i64, y as i64));
                },
                '^' => {
                    curr = (x as i64,y as i64);
                },
                '.' => {}
                _ => {panic!("Unknown character {} at ({},{})", cell, x, y)}
            }
        }
    }
    (obstructions, origin, bounds, curr, direction)
}

fn walk_path(mut curr: (i64, i64), mut direction: Direction, origin: Option<(i64, i64)>, bounds: Option<(i64, i64)>, obstructions: HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    let mut guard_path = HashSet::new();
    guard_path.insert(curr);
    
    while let Some(next) = direction.move_pos(curr, 1, origin, bounds) {
        // If there is something directly in front of you, turn right 90 degrees.
        if obstructions.contains(&next) {
            direction = direction.turn_right();
        } else {
            curr = next;
            guard_path.insert(curr);    
        }
    }
    guard_path
}

fn find_loop(mut direction: Direction, mut curr: (i64, i64), origin: Option<(i64, i64)>, bounds: Option<(i64, i64)>, obstructions: &HashSet<(i64, i64)>) -> bool {
    let mut guard_path = HashSet::new();
    guard_path.insert((curr, direction));

    while let Some(next) = direction.move_pos(curr, 1, origin, bounds) {
        // If we've seen this before, we're in a loop!
        if guard_path.contains(&(next, direction)) {
            return true;
        }
        // If there is something directly in front of you, turn right 90 degrees.
        if obstructions.contains(&next) {
            direction = direction.turn_right();
        } else {
            curr = next;
            guard_path.insert((curr, direction));    
        }
    }
    
    false
}

fn process_data_a(data: &str) -> usize {
    let (obstructions, origin, bounds, mut curr, mut direction) = get_data(data);
    let guard_path = walk_path(curr, direction, origin, bounds, obstructions);
    guard_path.len()
}

fn process_data_b(data: &str) -> usize {
    let (obstructions, origin, bounds, curr, direction) = get_data(data);

    let mut rv = 0;
    for x in origin.unwrap().0..bounds.unwrap().0 {
        for y in origin.unwrap().1..bounds.unwrap().1 {
            // println!("Checking ({},{})…", x, y);

            if !obstructions.contains(&(x,y)) && curr != (x, y) {
                let mut obstructions = obstructions.clone();
                obstructions.insert((x, y));
                // println!("  Adding obstruction…");
                if find_loop(direction, curr, origin, bounds, &obstructions) {
                    // println!("  Looped!");
                    rv += 1;
                }    
            }
        }
    }

    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("6");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_a(indoc!("
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
    ")), 41);
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b(indoc!("
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
    ")), 6);
}
