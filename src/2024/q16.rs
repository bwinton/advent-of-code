//-----------------------------------------------------
// Setup.

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use aoc::util::{Direction, Point2};

static INPUT: &str = include_str!("data/q16.data");

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Cell {
    Wall,
    Empty,
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    score: usize,
    direction: Direction,
    curr: Point2,
    path: HashSet<Point2>,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // compare scores in reverse!
        other.score.cmp(&self.score)
    }
}

fn parse(
    data: &str,
) -> (
    Vec<Vec<Cell>>,
    Point2,
    Point2,
    Option<Point2>,
    Option<Point2>,
) {
    let mut map = vec![];
    let mut start: Point2 = (-1, -1);
    let mut end: Point2 = (-1, -1);
    for (y, line) in data.lines().enumerate() {
        let mut row = vec![];
        for (x, cell) in line.chars().enumerate() {
            row.push(match cell {
                '#' => Cell::Wall,
                _ => Cell::Empty,
            });
            if cell == 'S' {
                start = (x as i64, y as i64);
            } else if cell == 'E' {
                end = (x as i64, y as i64)
            }
        }
        map.push(row);
    }
    let min = Some((0, 0));
    let max = Some((map[0].len() as i64, map.len() as i64));
    (map, start, end, min, max)
}

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    let (map, start, end, min, max) = parse(data);

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), Direction::East, start));
    let mut seen = HashSet::new();
    while let Some((score, direction, curr)) = heap.pop() {
        // println!("{} for {}@{:?}", score.0, direction, curr);
        if seen.contains(&(curr, direction)) {
            continue;
        }
        seen.insert((curr, direction));
        if curr == end {
            rv = score.0;
            break;
        }
        // Otherwise, we can step or turn.
        if let Some(next) = direction.move_pos(curr, 1, min, max)
            && !seen.contains(&(next, direction))
                && map[next.1 as usize][next.0 as usize] == Cell::Empty
            {
                heap.push((Reverse(score.0 + 1), direction, next));
            }
        heap.push((Reverse(score.0 + 1000), direction.turn_left(), curr));
        heap.push((Reverse(score.0 + 1000), direction.turn_right(), curr));
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let (map, start, end, min, max) = parse(data);

    let mut min_score = None;
    let mut path = HashSet::new();
    path.insert(start);
    let mut heap = BinaryHeap::new();
    let curr = State {
        score: 0,
        direction: Direction::East,
        curr: start,
        path,
    };
    heap.push(curr);
    let mut seen: HashMap<(Point2, Direction), (usize, HashSet<Point2>)> = HashMap::new();
    while let Some(State {
        score,
        direction,
        curr,
        path,
    }) = heap.pop()
    {
        // println!("{} for {}@{:?}", score, direction, curr);
        if min_score.is_some() && Some(score) > min_score {
            continue;
        }
        if curr == end {
            min_score = Some(score);
            seen.entry((curr, direction)).or_default().1.extend(path);
            // println!("Adding {} {:?}", score, path);
            continue;
        }
        if let std::collections::hash_map::Entry::Vacant(e) = seen.entry((curr, direction)) {
            e.insert((score, path.clone()));
        } else {
            let (prev_score, prev_path) = seen.get_mut(&(curr, direction)).unwrap();
            if score == *prev_score {
                prev_path.extend(path);
            }
            continue;
        }
        // Otherwise, we can step or turn.
        heap.push(State {
            score: score + 1000,
            direction: direction.turn_left(),
            curr,
            path: path.clone(),
        });
        heap.push(State {
            score: score + 1000,
            direction: direction.turn_right(),
            curr,
            path: path.clone(),
        });
        if let Some(next) = direction.move_pos(curr, 1, min, max) {
            if seen.contains_key(&(next, direction)) {
                let (prev_score, prev_path) = seen.get_mut(&(curr, direction)).unwrap();
                if score + 1 == *prev_score {
                    prev_path.extend(path);
                }
                continue;
            }
            if map[next.1 as usize][next.0 as usize] == Cell::Empty {
                let mut path = path.clone();
                path.insert(next);
                heap.push(State {
                    score: score + 1,
                    direction,
                    curr: next,
                    path,
                });
            }
        }
    }
    // println!("Final: {:?}", best_path);
    let mut rv = 0;
    for direction in Direction::all() {
        // println!(
        //     "({:?}, {}): {:?}",
        //     end,
        //     direction,
        //     seen.get(&(end, direction))
        // );
        if let Some(path) = seen.get(&(end, direction)) {
            // println!("  {}", path.1.len());
            rv += path.1.len();
        }
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("16");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "
    ###############
    #.......#....E#
    #.#.###.#.###.#
    #.....#.#...#.#
    #.###.#####.#.#
    #.#.#.......#.#
    #.#.#####.###.#
    #...........#.#
    ###.#.#####.#.#
    #...#.....#.#.#
    #.#.#.###.#.#.#
    #.....#...#.#.#
    #.###.#.#.#.#.#
    #S..#.....#...#
    ###############
    "
        )),
        7036
    );

    assert_eq!(
        process_data_a(indoc!(
            "
    #################
    #...#...#...#..E#
    #.#.#.#.#.#.#.#.#
    #.#.#.#...#...#.#
    #.#.#.#.###.#.#.#
    #...#.#.#.....#.#
    #.#.#.#.#.#####.#
    #.#...#.#.#.....#
    #.#.#####.#.###.#
    #.#.#.......#...#
    #.#.###.#####.###
    #.#.#...#.....#.#
    #.#.#.#####.###.#
    #.#.#.........#.#
    #.#.#.#########.#
    #S#.............#
    #################
    "
        )),
        11048
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "
    ###############
    #.......#....E#
    #.#.###.#.###.#
    #.....#.#...#.#
    #.###.#####.#.#
    #.#.#.......#.#
    #.#.#####.###.#
    #...........#.#
    ###.#.#####.#.#
    #...#.....#.#.#
    #.#.#.###.#.#.#
    #.....#...#.#.#
    #.###.#.#.#.#.#
    #S..#.....#...#
    ###############
    "
        )),
        45
    );

    assert_eq!(
        process_data_b(indoc!(
            "
    #################
    #...#...#...#..E#
    #.#.#.#.#.#.#.#.#
    #.#.#.#...#...#.#
    #.#.#.#.###.#.#.#
    #...#.#.#.....#.#
    #.#.#.#.#.#####.#
    #.#...#.#.#.....#
    #.#.#####.#.###.#
    #.#.#.......#...#
    #.#.###.#####.###
    #.#.#...#.....#.#
    #.#.#.#####.###.#
    #.#.#.........#.#
    #.#.#.#########.#
    #S#.............#
    #################
    "
        )),
        64
    );
}
