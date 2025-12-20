//-----------------------------------------------------
// Setup.

use std::collections::{HashMap, HashSet};

use aoc::util::{Direction, Point2};

static INPUT: &str = include_str!("data/q21.data");

fn parse(data: &str) -> (HashSet<Point2>, (Point2, Point2), Point2) {
    let mut board = HashSet::new();
    let mut start = (0, 0);
    let mut max = (0, data.lines().count() as i64);
    for (y, line) in data.lines().enumerate() {
        if max.0 == 0 {
            max.0 = line.chars().count() as i64;
        }
        for (x, cell) in line.chars().enumerate() {
            if cell == '#' {
                board.insert((x as i64, y as i64));
            } else if cell == 'S' {
                start = (x as i64, y as i64);
            }
        }
    }
    (board, ((0, 0), max), start)
}

fn calculate_moves(
    board: &HashSet<Point2>,
    bounds: (Point2, Point2),
    start: Point2,
    steps: i64,
) -> HashMap<Point2, i64> {
    let mut seen = HashMap::new();
    let mut curr = HashSet::new();

    curr.insert(start);
    seen.insert(start, 0);
    for step in 0..steps {
        let mut next = HashSet::new();
        for position in curr {
            for direction in Direction::all() {
                if let Some(test) = direction.move_pos(position, 1, None, None)
                    && test.0 >= bounds.0.0
                    && test.0 < bounds.1.0
                    && test.1 >= bounds.0.1
                    && test.1 < bounds.1.1
                    && !board.contains(&test)
                    && !seen.contains_key(&test)
                {
                    next.insert(test);
                    seen.insert(test, step + 1);
                }
            }
        }
        curr = next;
    }
    for position in curr {
        if seen.contains_key(&position) {
            // Someone else added this already!
            continue;
        }
        seen.insert(position, steps);
    }
    seen
}

fn calculate_moves_b_2(
    board: &HashSet<Point2>,
    bounds: (Point2, Point2),
    start: Point2,
    steps: i64,
) -> usize {
    // With many, many thanks to https://www.youtube.com/watch?v=9UOMZSL0JTg!
    let board_count = steps / bounds.1.0 - 1;

    let odd_grids = board_count.pow(2) as usize;
    let even_grids = (board_count + 1).pow(2) as usize;

    let points = calculate_moves(board, bounds, start, bounds.1.0);
    let mut even_points = points.len();
    let odd_points = points.values().filter(|&&v| v % 2 == 1).count();
    even_points -= odd_points;

    let corner_steps = bounds.1.0 - 1;
    let top = calculate_moves(board, bounds, (start.0, bounds.1.1 - 1), corner_steps);
    let top = top
        .iter()
        .filter(|&(_k, &v)| v % 2 == corner_steps % 2)
        .count();
    let left = calculate_moves(board, bounds, (0, start.1), corner_steps);
    let left = left
        .iter()
        .filter(|&(_k, &v)| v % 2 == corner_steps % 2)
        .count();
    let bottom = calculate_moves(board, bounds, (start.0, 0), corner_steps);
    let bottom = bottom
        .iter()
        .filter(|&(_k, &v)| v % 2 == corner_steps % 2)
        .count();
    let right = calculate_moves(board, bounds, (bounds.1.0 - 1, start.1), corner_steps);
    let right = right
        .iter()
        .filter(|&(_k, &v)| v % 2 == corner_steps % 2)
        .count();

    let small_edge_steps = bounds.1.0 / 2 - 1;
    let small_top_right = calculate_moves(board, bounds, (0, bounds.1.1 - 1), small_edge_steps);
    let small_top_right = small_top_right
        .iter()
        .filter(|&(_k, &v)| v % 2 == small_edge_steps % 2)
        .count();
    let small_bottom_right = calculate_moves(board, bounds, (0, 0), small_edge_steps);
    let small_bottom_right = small_bottom_right
        .iter()
        .filter(|&(_k, &v)| v % 2 == small_edge_steps % 2)
        .count();
    let small_bottom_left = calculate_moves(board, bounds, (bounds.1.0 - 1, 0), small_edge_steps);
    let small_bottom_left = small_bottom_left
        .iter()
        .filter(|&(_k, &v)| v % 2 == small_edge_steps % 2)
        .count();
    let small_top_left = calculate_moves(
        board,
        bounds,
        (bounds.1.0 - 1, bounds.1.1 - 1),
        small_edge_steps,
    );
    let small_top_left = small_top_left
        .iter()
        .filter(|&(_k, &v)| v % 2 == small_edge_steps % 2)
        .count();

    let big_edge_steps = 3 * bounds.1.0 / 2 - 1;
    let big_top_right = calculate_moves(board, bounds, (0, bounds.1.1 - 1), big_edge_steps);
    let big_top_right = big_top_right
        .iter()
        .filter(|&(_k, &v)| v % 2 == big_edge_steps % 2)
        .count();
    let big_bottom_right = calculate_moves(board, bounds, (0, 0), big_edge_steps);
    let big_bottom_right = big_bottom_right
        .iter()
        .filter(|&(_k, &v)| v % 2 == big_edge_steps % 2)
        .count();
    let big_bottom_left = calculate_moves(board, bounds, (bounds.1.0 - 1, 0), big_edge_steps);
    let big_bottom_left = big_bottom_left
        .iter()
        .filter(|&(_k, &v)| v % 2 == big_edge_steps % 2)
        .count();
    let big_top_left = calculate_moves(
        board,
        bounds,
        (bounds.1.0 - 1, bounds.1.1 - 1),
        big_edge_steps,
    );
    let big_top_left = big_top_left
        .iter()
        .filter(|&(_k, &v)| v % 2 == big_edge_steps % 2)
        .count();

    odd_grids * odd_points
        + even_grids * even_points
        + top
        + left
        + bottom
        + right
        + (board_count as usize + 1)
            * (small_top_right + small_bottom_right + small_bottom_left + small_top_left)
        + (board_count as usize)
            * (big_top_right + big_bottom_right + big_bottom_left + big_top_left)
}

fn process_data_a(data: &str) -> usize {
    let (board, bounds, start) = parse(data);
    let seen = calculate_moves(&board, bounds, start, 64);
    seen.into_values().filter(|&v| v % 2 == 64 % 2).count()
}

fn process_data_b(data: &str) -> usize {
    let (board, bounds, start) = parse(data);
    calculate_moves_b_2(&board, bounds, start, 26501365)
}

//-----------------------------------------------------
// Questions.

q_impl!("21");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    let (board, bounds, start) = parse(indoc!(
        "
    ...........
    .....###.#.
    .###.##..#.
    ..#.#...#..
    ....#.#....
    .##..S####.
    .##..#...#.
    .......##..
    .##.#.####.
    .##..##.##.
    ...........
    "
    ));

    let seen = calculate_moves(&board, bounds, start, 6);
    assert_eq!(seen.into_values().filter(|&v| v % 2 == 0).count(), 16);
    let seen = calculate_moves(
        &board,
        ((i64::MIN, i64::MIN), (i64::MAX, i64::MAX)),
        start,
        64,
    );
    assert_eq!(seen.into_values().filter(|&v| v % 2 == 0).count(), 4056);
}

#[test]
fn b() {
    // Can't use the same algorithm for the sample as for the real input, so skip this.
    // Not gonna lie, I kinda hate these type of problems.
}
