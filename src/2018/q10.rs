//-----------------------------------------------------
// Setup.

use regex::Regex;
use std::{
    cmp::{max, min},
    str::FromStr,
};

static INPUT: &str = include_str!("data/q10.data");

#[derive(Debug)]
struct Point {
    p_x: i32,
    p_y: i32,
    v_x: i32,
    v_y: i32,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Point, ()> {
        let re: &Regex = regex!(r"position=< *(-?\d+), *(-?\d+)> velocity=< *(-?\d+), *(-?\d+)>");

        if let Some(cap) = re.captures(s) {
            return Ok(Point {
                p_x: cap[1].parse().unwrap(),
                p_y: cap[2].parse().unwrap(),
                v_x: cap[3].parse().unwrap(),
                v_y: cap[4].parse().unwrap(),
            });
        }

        Err(())
    }
}

impl Point {
    fn step(&mut self) {
        self.p_x += self.v_x;
        self.p_y += self.v_y;
    }

    fn step_back(&mut self) {
        self.p_x -= self.v_x;
        self.p_y -= self.v_y;
    }
}

fn process_data_a(data: &str) -> String {
    let mut points = vec![];
    for line in data.lines() {
        let point: Point = line.parse().unwrap();
        points.push(point);
    }
    let mut smallest = i64::MAX;
    loop {
        for point in &mut points {
            point.step();
        }
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = 0;
        let mut max_y = 0;
        // figure out if we're close enough…
        for point in &points {
            min_x = min(min_x, point.p_x);
            min_y = min(min_y, point.p_y);
            max_x = max(max_x, point.p_x);
            max_y = max(max_y, point.p_y);
        }
        let area = i64::from(max_x - min_x) * i64::from(max_y - min_y);
        if smallest < area {
            // Step back one, and print the board!
            let mut min_x = i32::MAX;
            let mut min_y = i32::MAX;
            let mut max_x = 0;
            let mut max_y = 0;
            for point in &mut points {
                point.step_back();
                min_x = min(min_x, point.p_x);
                min_y = min(min_y, point.p_y);
                max_x = max(max_x, point.p_x);
                max_y = max(max_y, point.p_y);
            }

            let mut rv = String::with_capacity(area as usize * 2 + max_y as usize + 1);
            rv.push('\n');
            for y in min_y - 1..=max_y + 1 {
                'x: for x in min_x - 1..=max_x + 1 {
                    for point in &points {
                        if point.p_x == x && point.p_y == y {
                            rv.push('█');
                            continue 'x;
                        }
                    }
                    rv.push(' ');
                }
                rv.push('\n');
            }
            break rv;
        }
        smallest = area;
    }
}

fn process_data_b(data: &str) -> i32 {
    let mut points = vec![];
    for line in data.lines() {
        let point: Point = line.parse().unwrap();
        points.push(point);
    }
    let mut smallest = i64::MAX;
    let mut index = 0;
    loop {
        index += 1;
        for point in &mut points {
            point.step();
        }
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = 0;
        let mut max_y = 0;
        // figure out if we're close enough…
        for point in &points {
            min_x = min(min_x, point.p_x);
            min_y = min(min_y, point.p_y);
            max_x = max(max_x, point.p_x);
            max_y = max(max_y, point.p_y);
        }
        let area = i64::from(max_x - min_x) * i64::from(max_y - min_y);
        if smallest < area {
            break;
        }
        smallest = area;
    }
    index - 1
}

//-----------------------------------------------------
// Questions.

q_impl!("10");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(
            "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>"
        ),
        "
            
 █   █  ███ 
 █   █   █  
 █   █   █  
 █████   █  
 █   █   █  
 █   █   █  
 █   █   █  
 █   █  ███ 
            
"
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(
            "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>"
        ),
        3
    );
}
