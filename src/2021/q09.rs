//-----------------------------------------------------
// Setup.

use itertools::Itertools;

static INPUT: &str = include_str!("data/q09.data");

fn lowest_neighbour(i: usize, j: usize, map: &[Vec<u8>]) -> u8 {
    let mut rv = u8::MAX;
    if i > 0 && map[i - 1][j] < rv {
        rv = map[i - 1][j];
    }
    if j > 0 && map[i][j - 1] < rv {
        rv = map[i][j - 1];
    }
    if i < map.len() - 1 && map[i + 1][j] < rv {
        rv = map[i + 1][j];
    }
    if j < map[i].len() - 1 && map[i][j + 1] < rv {
        rv = map[i][j + 1];
    }
    rv
}

fn get_basin(i: usize, j: usize, map: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut to_check = vec![(i, j)];
    let mut seen = vec![];

    while !to_check.is_empty() {
        let (i, j) = to_check.pop().unwrap();
        seen.push((i, j));
        if i > 0 {
            let i = i - 1;
            if !seen.contains(&(i, j)) && !to_check.contains(&(i, j)) && map[i][j] != 9 {
                to_check.push((i, j));
            }
        }
        if j > 0 {
            let j = j - 1;
            if !seen.contains(&(i, j)) && !to_check.contains(&(i, j)) && map[i][j] != 9 {
                to_check.push((i, j));
            }
        }
        if i < map.len() - 1 {
            let i = i + 1;
            if !seen.contains(&(i, j)) && !to_check.contains(&(i, j)) && map[i][j] != 9 {
                to_check.push((i, j));
            }
        }
        if j < map[i].len() - 1 {
            let j = j + 1;
            if !seen.contains(&(i, j)) && !to_check.contains(&(i, j)) && map[i][j] != 9 {
                to_check.push((i, j));
            }
        }
    }

    seen
}

fn process_data_a(data: &str) -> u64 {
    let mut map: Vec<Vec<u8>> = vec![];
    for line in data.lines() {
        map.push(
            line.chars()
                .map(|x| format!("{}", x).parse().unwrap())
                .collect(),
        );
    }
    let mut lows = vec![];
    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell < lowest_neighbour(i, j, &map) {
                lows.push(cell as u64);
            }
        }
    }
    lows.iter().map(|x| x + 1).sum()
}

fn process_data_b(data: &str) -> usize {
    let mut map: Vec<Vec<u8>> = vec![];
    for line in data.lines() {
        map.push(
            line.chars()
                .map(|x| format!("{}", x).parse().unwrap())
                .collect(),
        );
    }
    let mut lows = vec![];
    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell < lowest_neighbour(i, j, &map) {
                lows.push((i, j));
            }
        }
    }
    lows.iter()
        .map(|x| get_basin(x.0, x.1, &map).len())
        .sorted()
        .rev()
        .take(3)
        .product()
}

//-----------------------------------------------------
// Questions.

q_impl!("9");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "2199943210
    3987894921
    9856789892
    8767896789
    9899965678
    "
        )),
        15
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "2199943210
    3987894921
    9856789892
    8767896789
    9899965678
    "
        )),
        1134
    );
}
