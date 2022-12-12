//-----------------------------------------------------
// Setup.

use aoc::Day;

use std::collections::HashMap;

static INPUT: i32 = 265_149;

enum Direction {
    Left,
    Up,
    Right,
    Down,
}

fn spiral_iter() -> impl Iterator<Item = [i32; 2]> {
    let mut curr = [0, 0];
    let mut dir = Direction::Left;
    let mut len = 1;
    let mut remaining = 1;

    std::iter::from_fn(move || {
        let rv = curr;

        remaining -= 1;

        match dir {
            Direction::Left => {
                curr[0] += 1;
                if remaining == 0 {
                    dir = Direction::Up;
                    remaining = len;
                }
            }
            Direction::Up => {
                curr[1] -= 1;
                if remaining == 0 {
                    dir = Direction::Right;
                    len += 1;
                    remaining = len;
                }
            }
            Direction::Right => {
                curr[0] -= 1;
                if remaining == 0 {
                    dir = Direction::Down;
                    remaining = len;
                }
            }
            Direction::Down => {
                curr[1] += 1;
                if remaining == 0 {
                    dir = Direction::Left;
                    len += 1;
                    remaining = len;
                }
            }
        }

        Some(rv)
    })
}

fn process_data_a(data: i32) -> i32 {
    if data == 1 {
        return 0;
    }
    let numbers = 0..;
    let mut rv = 0;
    for i in numbers {
        let block = 2 * i + 1;
        if block * block >= data {
            let remainder = data - (block - 2) * (block - 2);
            let low = i;
            let high = 2 * i;
            let mut seesaw = (low..high)
                .chain(high - 1..=low)
                .cycle()
                .skip(remainder as usize);
            rv = seesaw.next().unwrap();
            break;
        }
    }
    rv
}

fn mult_iter() -> impl Iterator<Item = usize> {
    let mut spiral = spiral_iter();
    let mut seen = HashMap::new();
    std::iter::from_fn(move || {
        let mut rv = 0;
        let curr = spiral.next().unwrap();
        if curr == [0, 0] {
            rv = 1;
        }
        for x in -1..2 {
            for y in -1..2 {
                if let Some(cell) = seen.get(&[curr[0] + x, curr[1] + y]) {
                    rv += cell;
                }
            }
        }
        seen.insert(curr, rv);
        Some(rv)
    })
}

fn process_data_b(data: i32) -> usize {
    for number in mult_iter() {
        if number > data as usize {
            return number;
        }
    }
    unreachable!();
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
    fn number(&self) -> String {
        String::from("3")
    }

    fn a(&self) {
        print!("{}A: ", self.number());
        let result = process_data_a(INPUT);
        println!("Result = {}", result);
    }

    fn b(&self) {
        print!("{}B: ", self.number());
        let result = process_data_b(INPUT);
        println!("Result = {}", result);
    }
}

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_a(1), 0);
    assert_eq!(process_data_a(12), 3);
    assert_eq!(process_data_a(23), 2);
    assert_eq!(process_data_a(1024), 31);
}

#[test]
fn b() {
    let spiral_values: Vec<[i32; 2]> = spiral_iter().take(25).collect();
    let spiral_expected: Vec<[i32; 2]> = vec![
        [0, 0],
        [1, 0],
        [1, -1],
        [0, -1],
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, 1],
        [1, 1],
        [2, 1],
        [2, 0],
        [2, -1],
        [2, -2],
        [1, -2],
        [0, -2],
        [-1, -2],
        [-2, -2],
        [-2, -1],
        [-2, 0],
        [-2, 1],
        [-2, 2],
        [-1, 2],
        [0, 2],
        [1, 2],
        [2, 2],
    ];
    assert_eq!(spiral_values, spiral_expected);

    let mult_values: Vec<usize> = mult_iter().take(23).collect();
    let mult_expected: Vec<usize> = vec![
        1, 1, 2, 4, 5, 10, 11, 23, 25, 26, 54, 57, 59, 122, 133, 142, 147, 304, 330, 351, 362, 747,
        806,
    ];
    assert_eq!(mult_values, mult_expected);
}
