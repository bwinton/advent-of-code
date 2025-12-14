//-----------------------------------------------------
// Setup.

use std::collections::HashMap;

static INPUT: &str = include_str!("data/q11.data");

fn stone_count(stone: String, iterations: i32, cache: &mut HashMap<(String, i32), usize>) -> usize {
    if iterations == 0 {
        return 1;
    };

    if let Some(&rv) = cache.get(&(stone.clone(), iterations)) {
        return rv;
    };

    let mut rv;
    if stone == *"0" {
        rv = stone_count("1".to_owned(), iterations - 1, cache);
    } else if stone.len().is_multiple_of(2) {
        let clone = stone.clone();
        let (left, right) = clone.split_at(stone.len() / 2);
        rv = stone_count(left.to_owned(), iterations - 1, cache);
        let mut right = right.trim_start_matches('0').to_owned();
        if right.is_empty() {
            right = "0".to_owned();
        }
        rv += stone_count(right, iterations - 1, cache);
    } else {
        rv = stone_count(
            format!("{}", stone.parse::<i64>().unwrap() * 2024),
            iterations - 1,
            cache,
        );
    }
    cache.insert((stone, iterations), rv);
    rv
}

fn process_data_a(data: &str) -> usize {
    let stones: Vec<String> = data.split_whitespace().map(|s| s.to_owned()).collect();
    let mut rv = 0;
    let mut cache = HashMap::new();
    for stone in stones {
        rv += stone_count(stone, 25, &mut cache);
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let stones: Vec<String> = data.split_whitespace().map(|s| s.to_owned()).collect();
    let mut rv = 0;
    let mut cache = HashMap::new();
    for stone in stones {
        let add = stone_count(stone, 75, &mut cache);
        rv += add;
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("11");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_a(indoc!("125 17")), 55312);
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b(indoc!("")), 0);
}
