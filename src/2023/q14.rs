//-----------------------------------------------------
// Setup.

use std::collections::HashMap;

use aoc::util::Point2;

static INPUT: &str = include_str!("data/q14.data");

#[derive(Copy, Clone, Debug, PartialOrd, Ord, Hash, Eq, PartialEq)]
enum Rock {
    Rounded,
    Cube,
}

#[allow(dead_code)]
fn print_map(map: &HashMap<Point2, Rock>, max: Point2) -> String {
    let mut rv = String::new();
    for y in 0..=max.1 {
        for x in 0..=max.0 {
            rv.push(match map.get(&(x, y)) {
                None => '.',
                Some(Rock::Rounded) => 'O',
                Some(Rock::Cube) => '#',
            });
        }
        rv.push('\n');
    }
    rv
}

fn parse(data: &str) -> (HashMap<Point2, Rock>, Point2) {
    let mut map = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in data.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            max_x = max_x.max(x as i64);
            match cell {
                '#' => {
                    map.insert((x as i64, y as i64), Rock::Cube);
                }
                'O' => {
                    map.insert((x as i64, y as i64), Rock::Rounded);
                }
                '.' => {}
                _ => {
                    panic!("Unknown character! {}", cell)
                }
            }
        }
        max_y = max_y.max(y as i64);
    }

    (map, (max_x, max_y))
}

fn move_rock(map: &mut HashMap<(i64, i64), Rock>, curr: Point2, max: Point2, direction: Point2) {
    if let Some(Rock::Rounded) = map.get(&curr) {
        let mut new = (curr.0 as i64, curr.1 as i64);
        // Move this as far as as possible.
        loop {
            new.0 += direction.0;
            new.1 += direction.1;
            if new.0 < 0 || new.1 < 0 || new.0 > max.0 as i64 || new.1 > max.1 as i64 {
                map.remove(&curr);
                new.0 -= direction.0;
                new.1 -= direction.1;
                map.insert((new.0, new.1), Rock::Rounded);
                break;
            }

            if map.get(&(new.0, new.1)).is_some() {
                // hit a rock;
                map.remove(&curr);
                new.0 -= direction.0;
                new.1 -= direction.1;
                map.insert((new.0, new.1), Rock::Rounded);
                break;
            }
        }
    }
}

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    let (mut map, max) = parse(data);
    for y in 0..=max.1 {
        for x in 0..=max.0 {
            move_rock(&mut map, (x, y), max, (0, -1));
        }
    }
    for ((_, y), rock) in map {
        if rock == Rock::Rounded {
            rv += max.1 - y + 1;
        }
    }

    rv as usize
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    let (mut map, max) = parse(data);
    let mut seen = HashMap::new();
    const CYCLE_COUNT: usize = 1000000000;
    for cycle in 0..=CYCLE_COUNT {
        let key = map.keys().cloned().collect::<Vec<_>>();
        if let Some((i, cycle_sum)) = seen.get(&key) {
            if (CYCLE_COUNT - cycle) % (cycle - i) == 0 {
                rv = *cycle_sum;
                break;
            }
        } else {
            let mut cycle_sum = 0;
            for ((_, y), rock) in &map {
                if rock == &Rock::Rounded {
                    cycle_sum += max.1 - y + 1;
                }
            }
            seen.insert(key, (cycle, cycle_sum));
        }
        for y in 0..=max.1 {
            for x in 0..=max.0 {
                move_rock(&mut map, (x, y), max, (0, -1));
            }
        }
        for x in 0..=max.0 {
            for y in 0..=max.1 {
                move_rock(&mut map, (x, y), max, (-1, 0));
            }
        }
        for y in 0..=max.1 {
            for x in 0..=max.0 {
                move_rock(&mut map, (x, max.1 - y), max, (0, 1));
            }
        }
        for x in 0..=max.0 {
            for y in 0..=max.1 {
                move_rock(&mut map, (max.0 - x, y), max, (1, 0));
            }
        }
    }
    rv as usize
}

//-----------------------------------------------------
// Questions.

q_impl!("14");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "
    O....#....
    O.OO#....#
    .....##...
    OO.#O....O
    .O.....O#.
    O.#..O.#.#
    ..O..#O..O
    .......O..
    #....###..
    #OO..#....
    "
        )),
        136
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "
    O....#....
    O.OO#....#
    .....##...
    OO.#O....O
    .O.....O#.
    O.#..O.#.#
    ..O..#O..O
    .......O..
    #....###..
    #OO..#....
    "
        )),
        64
    );
}
