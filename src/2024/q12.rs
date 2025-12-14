//-----------------------------------------------------
// Setup.

use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use aoc::util::{Direction, Point2};
use itertools::Itertools;

static INPUT: &str = include_str!("data/q12.data");

#[derive(Clone, Debug)]
struct Region {
    plant: char,
    plots: HashSet<Point2>,
}

impl Region {
    fn area(&self) -> usize {
        self.plots.len()
    }

    fn perimeter(&self) -> usize {
        let mut rv = 0;
        for cell in &self.plots {
            let mut add = 4;
            for direction in Direction::all() {
                if let Some(next) = direction.move_pos(*cell, 1, None, None)
                    && self.plots.contains(&next) {
                        add -= 1;
                    }
            }
            rv += add;
        }
        rv
    }

    fn sides(&self) -> usize {
        Direction::all()
            .into_iter()
            .map(|direction| self.find_sides(direction))
            .sum()
    }

    fn find_sides(&self, direction: Direction) -> usize {
        let mut rv = 0;
        let mut sides: HashMap<i64, Vec<i64>> = HashMap::new();
        for cell in &self.plots {
            let (parallel, cross) = match direction {
                Direction::North | Direction::South => (cell.1, cell.0),
                Direction::East | Direction::West => (cell.0, cell.1),
            };

            if let Some(next) = direction.move_pos(*cell, 1, None, None) {
                if !self.plots.contains(&next) {
                    sides.entry(parallel).or_default().push(cross);
                }
            } else {
                sides.entry(parallel).or_default().push(cross);
            }
        }
        for side in sides.values_mut() {
            side.sort();
            rv += 1;
            for (a, b) in side.iter().tuple_windows() {
                if *a + 1 != *b {
                    rv += 1;
                }
            }
        }
        rv
    }
}

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let perimeter_price = self.area() * self.perimeter();
        let side_price = self.area() * self.sides();
        f.write_fmt(format_args!("A region of {} plants with area {}, perimeter {}, and sides {} for a price of {} or {}.", self.plant, self.area(), self.perimeter(), self.sides(), perimeter_price, side_price))
    }
}

fn add_region(map: &[Vec<(char, bool)>], curr: Point2) -> Region {
    let bounds = (map[0].len() as i64, map.len() as i64);
    let plant = map[curr.1 as usize][curr.0 as usize].0;
    let mut plots = HashSet::new();
    // populate plots.
    let mut stack = vec![curr];
    while let Some(cell) = stack.pop() {
        plots.insert(cell);
        for direction in Direction::all() {
            if let Some(next) = direction.move_pos(cell, 1, Some((0, 0)), Some(bounds)) {
                let next_cell = map[next.1 as usize][next.0 as usize];
                if !next_cell.1 && next_cell.0 == plant && !plots.contains(&next) {
                    stack.push(next);
                }
            }
        }
    }
    Region { plant, plots }
}

fn parse(data: &str) -> Vec<Region> {
    let mut map = vec![];
    for line in data.lines() {
        let mut row = vec![];
        for cell in line.chars() {
            row.push((cell, false));
        }
        map.push(row);
    }
    let mut regions = vec![];
    for y in 0..map.len() {
        let row = &mut map[y];
        for x in 0..row.len() {
            let cell = &mut map[y][x];
            if !cell.1 {
                let region = add_region(&map, (x as i64, y as i64));
                for (x, y) in region.clone().plots {
                    map[y as usize][x as usize].1 = true;
                }
                regions.push(region);
            }
        }
    }
    regions
}

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    let regions = parse(data);
    for region in regions {
        rv += region.area() * region.perimeter();
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    let regions = parse(data);
    for region in regions {
        // println!("Region {}", region);
        rv += region.area() * region.sides();
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("12");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "
    AAAA
    BBCD
    BBCC
    EEEC
    "
        )),
        140
    );

    assert_eq!(
        process_data_a(indoc!(
            "
    OOOOO
    OXOXO
    OOOOO
    OXOXO
    OOOOO
    "
        )),
        772
    );

    assert_eq!(
        process_data_a(indoc!(
            "
    RRRRIICCFF
    RRRRIICCCF
    VVRRRCCFFF
    VVRCCCJFFF
    VVVVCJJCFE
    VVIVCCJJEE
    VVIIICJJEE
    MIIIIIJJEE
    MIIISIJEEE
    MMMISSJEEE
    "
        )),
        1930
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "
    AAAA
    BBCD
    BBCC
    EEEC
    "
        )),
        80
    );

    assert_eq!(
        process_data_b(indoc!(
            "
    OOOOO
    OXOXO
    OOOOO
    OXOXO
    OOOOO
    "
        )),
        436
    );

    assert_eq!(
        process_data_b(indoc!(
            "
    EEEEE
    EXXXX
    EEEEE
    EXXXX
    EEEEE
    "
        )),
        236
    );

    assert_eq!(
        process_data_b(indoc!(
            "
    AAAAAA
    AAABBA
    AAABBA
    ABBAAA
    ABBAAA
    AAAAAA
    "
        )),
        368
    );

    assert_eq!(
        process_data_b(indoc!(
            "
    RRRRIICCFF
    RRRRIICCCF
    VVRRRCCFFF
    VVRCCCJFFF
    VVVVCJJCFE
    VVIVCCJJEE
    VVIIICJJEE
    MIIIIIJJEE
    MIIISIJEEE
    MMMISSJEEE
    "
        )),
        1206
    );
}
