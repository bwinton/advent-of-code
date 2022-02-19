//-----------------------------------------------------
// Setup.

use regex::Regex;
use std::{
    cmp::{max, min},
    collections::HashMap,
    str::FromStr,
};

static INPUT: &str = include_str!("data/q06.data");

#[derive(Clone, Debug)]
struct Point {
    id: i32,
    x: i32,
    y: i32,
    size: i32,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Point, ()> {
        let re: &Regex = regex!(r"(\d+), (\d+)");

        if let Some(cap) = re.captures(s) {
            return Ok(Point {
                id: -1,
                x: cap[1].parse().unwrap(),
                y: cap[2].parse().unwrap(),
                size: 0,
            });
        }

        Err(())
    }
}

fn process_data_a(data: &str) -> i32 {
    let mut points = HashMap::new();
    for (id, line) in data.lines().enumerate() {
        let mut point: Point = line.parse().unwrap();
        point.id = id as i32;
        points.insert(point.id, point);
    }
    let mut min_x = i32::max_value();
    let mut min_y = i32::max_value();
    let mut max_x = 0;
    let mut max_y = 0;

    for point in points.values() {
        min_x = min(min_x, point.x);
        min_y = min(min_y, point.y);
        max_x = max(max_x, point.x);
        max_y = max(max_y, point.y);
    }

    let mut points_of_interest = points.clone();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let mut distances = vec![];
            for point in points.values() {
                let d2 = (x - point.x).abs() + (y - point.y).abs();
                distances.push((d2, point.id));
            }
            distances.sort_unstable();
            let min;
            {
                min = *distances.iter().min().unwrap();
            }
            distances.retain(|p| p.0 == min.0);

            if distances.len() > 1 {
                continue;
            }
            // Filter out points that hit the edge, cause they'll go on forever.
            if x == min_x || x == max_x || y == min_y || y == max_y {
                points_of_interest.remove(&min.1);
            } else if let Some(point) = points_of_interest.get_mut(&min.1) {
                point.size += 1;
            }
        }
    }

    points_of_interest.values().map(|p| p.size).max().unwrap()
}

fn find_safe_areas(data: &str, max_distance: i32) -> i32 {
    let mut points = HashMap::new();
    for (id, line) in data.lines().enumerate() {
        let mut point: Point = line.parse().unwrap();
        point.id = id as i32;
        points.insert(point.id, point);
    }
    let mut min_x = i32::max_value();
    let mut min_y = i32::max_value();
    let mut max_x = 0;
    let mut max_y = 0;

    for point in points.values() {
        if point.x < min_x {
            min_x = point.x;
        }
        if point.y < min_y {
            min_y = point.y;
        }
        if point.x > max_x {
            max_x = point.x;
        }
        if point.y > max_y {
            max_y = point.y;
        }
    }

    let mut safe_count = 0;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let mut distances = 0;
            for point in points.values() {
                distances += (x - point.x).abs() + (y - point.y).abs();
            }

            if distances < max_distance {
                safe_count += 1;
            }
        }
    }

    safe_count
}

fn process_data_b(data: &str) -> i32 {
    find_safe_areas(data, 10000)
}

//-----------------------------------------------------
// Questions.

q_impl!("6");

#[test]
fn a() {
    assert_eq!(
        process_data_a(
            "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"
        ),
        17
    );
}

#[test]
fn b() {
    assert_eq!(
        find_safe_areas(
            "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9",
            32
        ),
        16
    );
}
