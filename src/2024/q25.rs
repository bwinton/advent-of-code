//-----------------------------------------------------
// Setup.

use itertools::Itertools;
use nom::{
    IResult,
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    sequence::terminated,
};

static INPUT: &str = include_str!("data/q25.data");

type Items = Vec<Vec<usize>>;

fn line(i: &str) -> IResult<&str, Vec<char>> {
    let (input, line) = terminated(many1(one_of("#.")), newline)(i)?;
    // Massage the result.
    Ok((input, line))
}

fn item(i: &str) -> IResult<&str, (bool, Vec<usize>)> {
    let (input, item) = many1(line)(i)?;
    // Massage the result.
    let is_key = item[0][0] == '.';
    let height = item.len();
    let mut key = vec![usize::MAX; item[0].len()];
    for (y, line) in item.iter().enumerate() {
        for (x, &cell) in line.iter().enumerate() {
            if is_key {
                if cell == '#' && key[x] == usize::MAX {
                    key[x] = height - y - 1;
                }
            } else if cell == '.' && key[x] == usize::MAX {
                key[x] = y - 1;
            }
        }
    }
    Ok((input, (is_key, key)))
}

fn parser(i: &str) -> IResult<&str, (Items, Items)> {
    let (input, stuff) = separated_list1(newline, item)(i)?;
    let mut locks = vec![];
    let mut keys = vec![];
    for (key, thing) in stuff {
        if key {
            keys.push(thing);
        } else {
            locks.push(thing);
        }
    }
    Ok((input, (locks, keys)))
}

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    let (locks, keys) = parser(data).unwrap().1;
    let height = *keys
        .iter()
        .chain(locks.iter())
        .map(|v| v.iter().max().unwrap())
        .max()
        .unwrap();
    'outer: for (lock, key) in locks.iter().cartesian_product(keys.iter()) {
        for (l, k) in lock.iter().zip(key.iter()) {
            if k + l > height {
                continue 'outer;
            }
        }
        rv += 1;
    }
    rv

    // 59,914 is too high.
}

fn process_data_b(data: &str) -> usize {
    let rv = 0;
    for _line in data.lines() {
        // Do something
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("25");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "
            #####
            .####
            .####
            .####
            .#.#.
            .#...
            .....

            #####
            ##.##
            .#.##
            ...##
            ...#.
            ...#.
            .....

            .....
            #....
            #....
            #...#
            #.#.#
            #.###
            #####

            .....
            .....
            #.#..
            ###..
            ###.#
            ###.#
            #####

            .....
            .....
            .....
            #....
            #.#..
            #.#.#
            #####
            "
        )),
        3
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b(indoc!("")), 0);
}
