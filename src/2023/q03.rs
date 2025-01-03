//-----------------------------------------------------
// Setup.

use std::collections::{HashMap, HashSet};

use aoc::util::{Point2, ring};

static INPUT: &str = include_str!("data/q03.data");

#[derive(Debug, Clone)]

enum Cell {
    Symbol(char),
    Number(i64, i64, i64),
}

fn parse_data(data: &str) -> (i64, i64, HashMap<Point2, Cell>) {
    let rows = data.lines().count() as i64;
    let cols = data.lines().next().unwrap().chars().count() as i64;
    let mut grid = HashMap::new();
    for (y, line) in data.lines().enumerate() {
        let mut curr = None;
        for (x, cell) in line.chars().enumerate() {
            if cell.is_ascii_digit() {
                let mut temp = curr.unwrap_or((x as i64, 0));
                temp.1 *= 10;
                temp.1 += cell.to_digit(10).unwrap() as i64;
                curr = Some(temp);
            } else if let Some(temp) = curr {
                // We've hit the end of a number, so store it in all the positions!
                for i in temp.0..x as i64 {
                    grid.insert((i, y as i64), Cell::Number(temp.0, y as i64, temp.1));
                }
                curr = None;
            }
            if !cell.is_ascii_digit() && cell != '.' {
                grid.insert((x as i64, y as i64), Cell::Symbol(cell));
            }
        }
        if let Some(temp) = curr {
            // We've hit the end of the line with a number, so store it in all the positions!
            for i in temp.0..cols {
                grid.insert((i, y as i64), Cell::Number(temp.0, y as i64, temp.1));
            }
        }
    }
    (rows, cols, grid)
}

fn process_data_a(data: &str) -> usize {
    let (rows, cols, grid) = parse_data(data);

    let mut numbers = HashSet::new();
    for y in 0..rows {
        for x in 0..cols {
            if let Some(Cell::Symbol(_cell)) = grid.get(&(x, y)) {
                for (i, j) in ring((x, y), 1, (0, 0), (cols, rows)) {
                    if let Some(Cell::Number(x, y, cell)) = grid.get(&(i, j)) {
                        numbers.insert((x, y, cell));
                    }
                }
            }
        }
    }
    let rv: i64 = numbers.iter().map(|(_, _, value)| *value).sum();

    // 498559
    rv as usize
}

fn process_data_b(data: &str) -> usize {
    let (rows, cols, grid) = parse_data(data);

    let mut numbers: Vec<i64> = vec![];
    for y in 0..rows {
        for x in 0..cols {
            if let Some(Cell::Symbol('*')) = grid.get(&(x, y)) {
                let mut parts = HashSet::new();
                for (i, j) in ring((x, y), 1, (0, 0), (cols, rows)) {
                    if let Some(Cell::Number(x, y, cell)) = grid.get(&(i, j)) {
                        parts.insert((x, y, cell));
                    }
                }
                if parts.len() == 2 {
                    numbers.push(parts.iter().map(|(_, _, value)| *value).product());
                }
            }
        }
    }
    let rv: i64 = numbers.iter().sum();

    // 72246648
    rv as usize
}

//-----------------------------------------------------
// Questions.

q_impl!("3");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    "
        )),
        4361
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    "
        )),
        467835
    );
}
