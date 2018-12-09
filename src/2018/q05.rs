//-----------------------------------------------------
// Setup.

use std::iter::Iterator;

static INPUT: &'static str = include_str!("data/q05.data");

fn remove_pairs(data: &str) -> String {
    let mut data: Vec<_> = data.chars().collect();
    let mut found = true;
    while found {
        // add a terminator
        data.push(' ');
        // println!("\"{}\"", data.iter().collect::<String>());
        found = false;
        let mut rv = vec![];
        let mut skip = false;
        for (curr, next) in data.iter().zip(data.iter().skip(1)) {
            if skip {
                skip = false;
                continue;
            }
            // println!("Testing {} {} == {}", curr, next,
            // curr != next && curr.to_lowercase().to_string() == next.to_lowercase().to_string());
            if curr == next || curr.to_lowercase().to_string() != next.to_lowercase().to_string() {
                rv.push(*curr);
            } else {
                // Skip the next one.
                skip = true;
                found = true;
            }
        }
        data = rv.clone();
    }
    // println!("\"{}\"", data.iter().collect::<String>());

    data.iter().collect()
}

fn process_data_a(data: &str) -> usize {
    // 9705 is too high…
    remove_pairs(data.trim()).len()
}

fn process_data_b(data: &str) -> usize {
    let data = data.trim();
    let mut min = (" ".to_owned(), data.len());
    let chars = "abcdefghijklmnopqrstuvwxyz";
    for remove in chars.chars() {
        let remove = remove.to_string();
        let curr: String = data
            .chars()
            .filter(|x| x.to_lowercase().to_string() != remove)
            .collect();
        if curr.len() == data.len() {
            // Didn't find any characters, so no need to continue.
            continue;
        }
        let size = remove_pairs(&curr).len();
        // println!("({}, {}): {:?}", &remove, size, min);
        if size < min.1 {
            min = (remove, size);
        }
    }
    min.1
    // 6943 is too high…
}

//-----------------------------------------------------
// Questions.

q_impl!("5");

#[test]
fn a() {
    assert_eq!(remove_pairs("aA"), "");
    assert_eq!(remove_pairs("abBA"), "");
    assert_eq!(remove_pairs("abAB"), "abAB");
    assert_eq!(remove_pairs("aabAAB"), "aabAAB");
    assert_eq!(remove_pairs("dabAcCaCBAcCcaDA"), "dabCBAcaDA");
    assert_eq!(process_data_a("aA"), 0);
    assert_eq!(process_data_a("abBA"), 0);
    assert_eq!(process_data_a("abAB"), 4);
    assert_eq!(process_data_a("aabAAB"), 6);
    assert_eq!(process_data_a("dabAcCaCBAcCcaDA"), 10);
    assert_eq!(process_data_a("IfreEEeRkKFD"), 2);
}

#[test]
fn b() {
    assert_eq!(process_data_b("dabAcCaCBAcCcaDA"), 4);
}
