use itertools::Itertools;
use std::collections::HashSet;

use aoc::letters::recognize_letters;

//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q13.data");

fn fold(coord: (&str, usize), grid: HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut rv = HashSet::new();
    let (axis, value) = coord;
    for (mut x, mut y) in grid {
        if axis == "x" {
            if x > value {
                x = value - (x - value);
            }
        } else if y > value {
            y = value - (y - value);
        }
        rv.insert((x, y));
    }
    rv
}

fn process_data_a(data: &str) -> usize {
    let mut grid = HashSet::new();
    let mut folds = vec![];
    let mut getting_points = true;
    for line in data.lines() {
        // Do something
        if line.is_empty() {
            getting_points = false;
            continue;
        }
        if getting_points {
            let (x, y) = line.split_once(',').unwrap();
            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();
            grid.insert((x, y));
        } else {
            let (coord, value) = line[11..].split_once('=').unwrap();
            let value: usize = value.parse().unwrap();
            folds.push((coord, value));
        }
    }
    grid = fold(folds[0], grid);
    grid.len()
}

fn process_data_b(data: &str) -> String {
    let mut grid = HashSet::new();
    let mut folds = vec![];
    let mut getting_points = true;
    for line in data.lines() {
        // Do something
        if line.is_empty() {
            getting_points = false;
            continue;
        }
        if getting_points {
            let (x, y) = line.split_once(',').unwrap();
            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();
            grid.insert((x, y));
        } else {
            let (coord, value) = line[11..].split_once('=').unwrap();
            let value: usize = value.parse().unwrap();
            folds.push((coord, value));
        }
    }
    for curr in folds {
        grid = fold(curr, grid);
    }
    let max_x = grid.iter().sorted().next_back().unwrap().0;
    let max_y = grid.iter().sorted_by_key(|x| x.1).next_back().unwrap().1;
    let mut image = Vec::with_capacity(max_x * max_y);
    for y in 0..=max_y {
        for x in 0..=max_x {
            image.push(grid.contains(&(x, y)));
        }
    }
    recognize_letters(&image)
}

//-----------------------------------------------------
// Questions.

q_impl!("13");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0
    
    fold along y=7
    fold along x=5
    "
        )),
        17
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0

    fold along y=7
    fold along x=5
    "
        )),
        indoc!(
            "████ 
        ██   
         ██  
          ██ 
           █ 
        ████ 
        █"
        )
    );
}
