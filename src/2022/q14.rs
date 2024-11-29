//-----------------------------------------------------
// Setup.

use itertools::Itertools;

static INPUT: &str = include_str!("data/q14.data");

type Coord = (usize, usize);

enum Drop {
    Continue,
    Stopped,
    Fell,
}

fn get_paths(data: &str) -> (Vec<Vec<Coord>>, Coord, Coord) {
    let mut min = (500, 1);
    let mut max = (0, 0);
    let paths: Vec<Vec<Coord>> = data
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|c| {
                    let values: Vec<usize> = c.split(',').map(|i| i.parse().unwrap()).collect();
                    if values[0] < min.0 {
                        min.0 = values[0];
                    }
                    if values[0] > max.0 {
                        max.0 = values[0];
                    }
                    if values[1] < min.1 {
                        min.1 = values[1];
                    }
                    if values[1] > max.1 {
                        max.1 = values[1];
                    }
                    (values[0], values[1])
                })
                .collect()
        })
        .collect();
    let offset_x = min.0 - 1;
    let offset_y = min.1 - 1;
    (paths, (offset_x, offset_y), max)
}

fn get_grid(paths: Vec<Vec<Coord>>, offset: Coord, max: Coord) -> Vec<Vec<bool>> {
    let (offset_x, offset_y) = offset;
    let mut grid = Vec::new();
    for _y in offset_y..=(max.1 + 1) {
        let mut row = vec![];
        for _x in offset_x..=(max.0 + 1) {
            row.push(false);
        }
        grid.push(row);
    }
    for path in paths {
        for (start, end) in path.iter().tuple_windows() {
            let &(mut start_x, mut start_y) = start;
            let &(mut end_x, mut end_y) = end;
            if start_x > end_x {
                (start_x, end_x) = (end_x, start_x);
            }
            if start_y > end_y {
                (start_y, end_y) = (end_y, start_y);
            }

            for row in grid
                .iter_mut()
                .take(end_y - offset_y + 1)
                .skip(start_y - offset_y)
            {
                for cell in row
                    .iter_mut()
                    .take(end_x - offset_x + 1)
                    .skip(start_x - offset_x)
                {
                    *cell = true;
                }
            }
        }
    }
    grid
}

fn drop(curr: &mut Coord, grid: &[Vec<bool>], floor: bool) -> Drop {
    if curr.1 + 1 == grid.len() {
        return if floor { Drop::Stopped } else { Drop::Fell };
    }

    if !grid[curr.1 + 1][curr.0] {
        curr.1 += 1;
        return Drop::Continue;
    }
    if !grid[curr.1 + 1][curr.0 - 1] {
        curr.1 += 1;
        curr.0 -= 1;
        return Drop::Continue;
    }
    if !grid[curr.1 + 1][curr.0 + 1] {
        curr.1 += 1;
        curr.0 += 1;
        return Drop::Continue;
    }

    Drop::Stopped
}

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    let (paths, offset, max) = get_paths(data);
    let mut grid = get_grid(paths, offset, max);
    let sand_start = (500 - offset.0, 0 - offset.1);

    'outer: loop {
        let mut curr = sand_start;
        rv += 1;
        loop {
            match drop(&mut curr, &grid, false) {
                Drop::Continue => {}
                Drop::Stopped => {
                    grid[curr.1][curr.0] = true;
                    break;
                }
                Drop::Fell => {
                    rv -= 1;
                    break 'outer;
                }
            }
        }
    }

    rv
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    let (paths, offset, max) = get_paths(data);
    let offset = (offset.0 - 200, offset.1);
    let max = (max.0 + 200, max.1);
    let mut grid = get_grid(paths, offset, max);
    let sand_start = (500 - offset.0, 0 - offset.1);

    'outer: loop {
        let mut curr = sand_start;
        rv += 1;
        loop {
            // dbg!(curr);
            match drop(&mut curr, &grid, true) {
                Drop::Continue => {}
                Drop::Stopped => {
                    grid[curr.1][curr.0] = true;
                    if curr == sand_start {
                        break 'outer;
                    }
                    break;
                }
                Drop::Fell => {
                    panic!("Nothing should fall!");
                }
            }
        }
    }

    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("14");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "498,4 -> 498,6 -> 496,6
    503,4 -> 502,4 -> 502,9 -> 494,9
    "
        )),
        24
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "498,4 -> 498,6 -> 496,6
    503,4 -> 502,4 -> 502,9 -> 494,9
    "
        )),
        93
    );
}
