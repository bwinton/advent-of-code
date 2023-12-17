//-----------------------------------------------------
// Setup.

use std::collections::{BinaryHeap, HashMap};

use aoc::util::Direction;

static INPUT: &str = include_str!("data/q17.data");

fn parse(data: &str) -> Vec<Vec<i32>> {
    let mut rv = vec![];
    for line in data.lines() {
        let mut curr = vec![];
        for col in line.chars() {
            curr.push(col.to_digit(10).unwrap() as i32)
        }
        rv.push(curr);
    }
    rv
}

fn process_data_a(data: &str) -> usize {
    let board = parse(data);
    find_path(&board, 0, 3)
}

fn process_data_b(data: &str) -> usize {
    let board = parse(data);
    find_path(&board, 4, 10)
}

fn find_path(board: &[Vec<i32>], min_run: i32, max_run: i32) -> usize {
    let start = (0usize, 0usize);
    let max = (board[0].len(), board.len());
    let target = (max.0 - 1, max.1 - 1);
    let mut curr = BinaryHeap::new();
    let mut seen = HashMap::new();
    curr.push((0i32, start, Direction::East, 0, vec![(start, 0)]));
    curr.push((0i32, start, Direction::South, 0, vec![(start, 0)]));
    loop {
        let next = curr.pop();
        if next.is_none() {
            break;
        }
        let (loss, cell, direction, blocks, path) = next.unwrap();

        if seen.contains_key(&(cell, direction, blocks)) {
            let value: &mut i32 = seen.get_mut(&(cell, direction, blocks)).unwrap();
            if -loss >= -*value {
                continue;
            }
            *value = loss;
        }
        seen.insert((cell, direction, blocks), loss);

        // Figure out where we can go from here…
        for proposed in Direction::all() {
            if proposed.opposite(&direction) {
                continue;
            }
            if let Some(new) = proposed.move_pos(cell, max) {
                let new_blocks = if proposed == direction {
                    if blocks == max_run {
                        continue;
                    }
                    blocks + 1
                } else {
                    1
                };
                if proposed != direction && blocks < min_run {
                    continue;
                }
                if new == target && new_blocks >= min_run {
                    return -(loss - board[new.1][new.0]) as usize;
                }
                let mut path = path.clone();
                path.push((new, new_blocks));
                curr.push((loss - board[new.1][new.0], new, proposed, new_blocks, path));
            }
        }
    }
    0
}

//-----------------------------------------------------
// Questions.

q_impl!("17");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "
    2413432311323
    3215453535623
    3255245654254
    3446585845452
    4546657867536
    1438598798454
    4457876987766
    3637877979653
    4654967986887
    4564679986453
    1224686865563
    2546548887735
    4322674655533
    "
        )),
        102
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "
    2413432311323
    3215453535623
    3255245654254
    3446585845452
    4546657867536
    1438598798454
    4457876987766
    3637877979653
    4654967986887
    4564679986453
    1224686865563
    2546548887735
    4322674655533
    "
        )),
        94
    );

    assert_eq!(
        process_data_b(indoc!(
            "
    111111111111
    999999999991
    999999999991
    999999999991
    999999999991
    "
        )),
        71
    );
}
