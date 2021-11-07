//-----------------------------------------------------
// Setup.

use std::{collections::HashMap, str::FromStr};

static INPUT: &str = include_str!("data/q25.data");

#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
    group: Option<usize>,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let values: Vec<_> = s.split(',').collect();
        if values.len() != 4 {
            return Err(());
        }
        Ok(Point {
            x: values[0].parse().unwrap(),
            y: values[1].parse().unwrap(),
            z: values[2].parse().unwrap(),
            w: values[3].parse().unwrap(),
            group: None,
        })
    }
}

impl Point {
    fn distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs()
            + (self.y - other.y).abs()
            + (self.z - other.z).abs()
            + (self.w - other.w).abs()
    }

    fn add_to_group(&mut self, groups: &mut HashMap<usize, Vec<Point>>, i: usize) {
        self.group = Some(i);
        groups
            .entry(i)
            .or_insert_with(|| -> Vec<Point> { vec![] })
            .push(self.clone());
    }
}

fn process_data_a(data: &str) -> usize {
    let mut points = vec![];
    for line in data.lines() {
        let point: Point = line.parse().unwrap();
        points.push(point);
    }

    let mut groups: HashMap<usize, Vec<Point>> = HashMap::new();
    let mut available = points;

    let mut id = 0;

    for curr in available.iter_mut() {
        let mut close_groups = vec![];
        for (i, group) in groups.clone() {
            for point in &group {
                if curr.distance(point) <= 3 {
                    close_groups.push(i);
                    break;
                }
            }
        }
        if close_groups.is_empty() {
            // Couldn't find a group, so make a new one.
            curr.add_to_group(&mut groups, id);
            id += 1;
        } else if close_groups.len() == 1 {
            curr.add_to_group(&mut groups, close_groups[0]);
        } else {
            // merge groups.
            close_groups.sort_unstable();
            let base = close_groups.pop().unwrap();
            curr.add_to_group(&mut groups, base);
            for group in &close_groups {
                for mut point in groups[group].clone() {
                    point.add_to_group(&mut groups, base);
                }
                groups.remove(group);
            }
        }
    }

    // Not 34.
    groups.len()
}

fn process_data_b(_data: &str) -> i32 {
    0
}

//-----------------------------------------------------
// Questions.

q_impl!("25");

#[test]
fn a() {
    assert_eq!(
        process_data_a(
            "0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0"
        ),
        2
    );
    assert_eq!(
        process_data_a(
            "-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0"
        ),
        4
    );
    assert_eq!(
        process_data_a(
            "1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2"
        ),
        3
    );
    assert_eq!(
        process_data_a(
            "1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2"
        ),
        8
    );
}

#[test]
fn b() {
    assert_eq!(process_data_b(""), 0);
}
