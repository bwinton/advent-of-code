//-----------------------------------------------------
// Setup.

use regex::Regex;
use std::str::FromStr;

static INPUT : &'static str = include_str!("data/q06.data");

#[derive(Clone, Debug)]
struct Point {
    id: i32,
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Point, ()> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+), (\d+)").unwrap();
        }

        if let Some(cap) = RE.captures(s) {
            return Ok(Point {
                id: -1,
                x: cap[1].parse().unwrap(),
                y: cap[2].parse().unwrap(),
            })
        }

        Err(())
    }
}

fn process_data_a(data: &str) -> i32 {
    let mut points = vec![];
    for (id, line) in data.lines().enumerate() {
        let mut point: Point = line.parse().unwrap();
        point.id = id as i32;
        points.push(point);
    }
    let mut min_x = i32::max_value();
    let mut min_y = i32::max_value();
    let mut max_x = 0;
    let mut max_y = 0;

    for point in &points {
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

    let mut points_of_interest: Vec<_> = points.iter().filter(|point| point.x != min_x && point.x != max_x && point.y != min_y && point.y != max_y).collect();
    println!("{:?}\n{},{} - {},{}", &points_of_interest, min_x, min_y, max_x, max_y);

    let mut changed = true;
    while changed {
        changed = false;
        // loop through the points, claim the neighbours, add them to the size.
        // if none of the points of interest have changed, bail out.
        // can keep an index to tell the distance we've gone.
    }
    0
}

fn process_data_b(_data: &str) -> i32 {
    0
}

//-----------------------------------------------------
// Questions.

q_impl!("6");

#[test]
fn a() {
    assert_eq!(process_data_a("1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"), 17);
}

#[test]
fn b() {
    assert_eq!(process_data_b(""), 0);
}
