//-----------------------------------------------------
// Setup.

use std::collections::{HashMap, VecDeque};

static INPUT: &str = include_str!("data/q17.data");

use once_cell::sync::Lazy;

static ROCKS: Lazy<[Vec<(i32, i32)>; 5]> = Lazy::new(|| {
    [
        vec![(2, 3), (3, 3), (4, 3), (5, 3)],         // Line
        vec![(3, 1), (2, 2), (3, 2), (4, 2), (3, 3)], // Plus
        vec![(4, 1), (4, 2), (2, 3), (3, 3), (4, 3)], // L
        vec![(2, 0), (2, 1), (2, 2), (2, 3)],         // I
        vec![(2, 2), (3, 2), (2, 3), (3, 3)],         // Square
    ]
});

fn move_rock(
    rock: &mut [(i32, i32)],
    board: &VecDeque<[bool; 7]>,
    moves: &[char],
    move_index: &mut usize,
) -> bool {
    let curr_move = moves[*move_index % moves.len()];
    *move_index += 1;

    // Move left or right.
    let mut test = rock.to_owned();
    match curr_move {
        '<' => {
            // Move left.
            for pos in test.iter_mut() {
                pos.0 -= 1;
            }
        }
        '>' => {
            // Move right.
            for pos in test.iter_mut() {
                pos.0 += 1;
            }
        }
        _ => {
            panic!("Invalid move: {}", curr_move)
        }
    }
    let mut valid = true;
    for &(x, y) in &test {
        if x < 0 || x as usize >= board[0].len() || board[y as usize][x as usize] {
            valid = false;
            break;
        }
    }
    if valid {
        rock.copy_from_slice(&test);
    }

    // Move down.
    let mut test = rock.to_owned();
    for pos in test.iter_mut() {
        pos.1 += 1;
    }
    let mut valid = true;
    for &(x, y) in &test {
        if y as usize >= board.len() || board[y as usize][x as usize] {
            valid = false;
            break;
        }
    }
    if valid {
        rock.copy_from_slice(&test);
    }
    valid
}

fn process_data_a(data: &str) -> usize {
    let moves: Vec<_> = data.trim().chars().collect();
    let mut board: VecDeque<[bool; 7]> = VecDeque::new();
    let mut move_index = 0;
    for i in 0..2022 {
        let mut rock = ROCKS[i % ROCKS.len()].clone();
        board.extend(&[[false; 7]; 7]);
        board.rotate_right(7);

        // Let the rock move and fall.
        while move_rock(&mut rock, &board, &moves, &mut move_index) {}

        // It's come to rest, so add it to the board.
        for (x, y) in rock {
            board[y as usize][x as usize] = true;
        }

        // Clean up the top of the board.
        while board[0] == [false, false, false, false, false, false, false] {
            board.pop_front();
        }
    }
    board.len()
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    let moves: Vec<_> = data.trim().chars().collect();
    let mut board: VecDeque<[bool; 7]> = VecDeque::new();
    let mut move_index = 0;
    let mut seen = HashMap::new();
    for i in 0..1_000_000_000_000 {
        let mut rock = ROCKS[i % ROCKS.len()].clone();
        board.extend(&[[false; 7]; 7]);
        board.rotate_right(7);

        // Let the rock move and fall.
        while move_rock(&mut rock, &board, &moves, &mut move_index) {}

        // It's come to rest, so add it to the board.
        for (x, y) in rock {
            board[y as usize][x as usize] = true;
        }

        // Clean up the top of the board.
        while board[0] == [false, false, false, false, false, false, false] {
            board.pop_front();
        }
        let key = (i % ROCKS.len(), move_index % moves.len());
        if seen.contains_key(&key) {
            let (prev_i, prev_len) = seen[&key];
            let period = i - prev_i;
            let delta = board.len() - prev_len;
            let multiple = (999_999_999_999 - i) / period;
            let remainder = (999_999_999_999 - i) % period;
            if remainder == 0 {
                rv = board.len() + delta * multiple;
                break;
            }
        }
        seen.insert(key, (i, board.len()));
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("17");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
    "
        )),
        3068
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
"
        )),
        1_514_285_714_288
    );
}
