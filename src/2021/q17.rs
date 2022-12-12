//-----------------------------------------------------
// Setup.

use once_cell::sync::Lazy;
use regex::Regex;

static INPUT: &str = include_str!("data/q17.data");

// target area: x=20..30, y=-10..-5
static AREA_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new("^target area: x=(-?[0-9]+)..(-?[0-9]+), y=(-?[0-9]+)..(-?[0-9]+)$").unwrap()
});

fn step(position: (i64, i64), velocity: (i64, i64)) -> ((i64, i64), (i64, i64)) {
    let (mut position, mut velocity) = (position, velocity);
    position.0 += velocity.0;
    position.1 += velocity.1;

    velocity.0 += match velocity.0 {
        x if x > 0 => -1,
        x if x > 0 => 1,
        _ => 0,
    };
    velocity.1 -= 1;

    (position, velocity)
}

fn process_data_a(data: &str) -> i64 {
    // target area: x=20..30, y=-10..-5
    let line = data.lines().next().unwrap();
    let captures = AREA_RE.captures(line).unwrap();
    let x_min: i64 = captures[1].parse().unwrap();
    let x_max: i64 = captures[2].parse().unwrap();
    let y_min: i64 = captures[3].parse().unwrap();
    let y_max: i64 = captures[4].parse().unwrap();

    // println!("x: {}..{}, y: {}..{}", x_min, x_max, y_min, y_max);

    let mut rv = 0;
    'outer: for y_velocity in y_min + 1..=y_max + 1 {
        let n = -y_velocity;
        let highest = n * (n + 1) / 2;
        let steps = n * 2 + 1;

        let smallest_x = x_max / steps;
        let largest_x = steps * (steps - 1) / 2 / x_min;
        // println!("Testing x{}..{}, y{}", smallest_x, largest_x, n);
        for x_velocity in smallest_x..=largest_x {
            let mut curr = (0, 0);
            let mut velocity = (x_velocity, n);
            for _ in 0..=steps {
                (curr, velocity) = step(curr, velocity);
            }
            // println!("  {}: p{:?}, v{:?}", x_velocity, curr, velocity);
            if x_min <= curr.0 && curr.0 <= x_max && y_min <= curr.1 && curr.1 <= y_max {
                // println!("Found!!!!!!! {:?}, {:?}", curr, highest);
                rv = highest;
                break 'outer;
            }
        }
    }

    rv
}

fn process_data_b(data: &str) -> usize {
    // target area: x=20..30, y=-10..-5
    let line = data.lines().next().unwrap();
    let captures = AREA_RE.captures(line).unwrap();
    let x_min: i64 = captures[1].parse().unwrap();
    let x_max: i64 = captures[2].parse().unwrap();
    let y_min: i64 = captures[3].parse().unwrap();
    let y_max: i64 = captures[4].parse().unwrap();

    // println!("x: {}..{}, y: {}..{}", x_min, x_max, y_min, y_max);

    let mut rv = vec![];
    for y_velocity in y_min + 1..=-y_min {
        let n = -y_velocity;

        for x_velocity in 0..=x_max {
            let mut curr = (0, 0);
            let mut velocity = (x_velocity, n);
            // println!("Testing {:?}", velocity);
            loop {
                (curr, velocity) = step(curr, velocity);
                if curr.0 > x_max || curr.1 < y_min {
                    break;
                }
                // println!("  x - {} <= {} <= {}, y - {} <= {} <= {}", x_min, curr.0, x_max, y_min, curr.1, y_max);
                if curr.0 >= x_min && curr.1 <= y_max {
                    // println!("  Found {:?}!!!!!!!", (x_velocity, n));
                    rv.push((x_velocity, n));
                    break;
                }
            }
        }
    }

    // println!("rv = {:?}", rv);

    rv.len()
}

//-----------------------------------------------------
// Questions.

q_impl!("17");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!("target area: x=20..30, y=-10..-5")),
        45
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!("target area: x=20..30, y=-10..-5")),
        112
    );
}
