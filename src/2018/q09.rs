//-----------------------------------------------------
// Setup.

use regex::Regex;
use std::collections::LinkedList;

static INPUT: &'static str = "458 players; last marble is worth 72019 points";

fn get_data(data: &str) -> (usize, usize) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
    }
    let cap = RE.captures(data).unwrap();
    (cap[1].parse().unwrap(), cap[2].parse().unwrap())
}

// fn print_state(i: &str, circle: &LinkedList<usize>) {
//     print!("[{}]: ", i);
//     for (i, value) in circle.iter().enumerate() {
//         if i == 0 {
//             print!("({}) ", value);
//         } else {
//             print!("{} ", value);
//         }
//     }
//     println!();
// }

fn process_data_a(data: &str) -> usize {
    let (players, final_point) = get_data(data);
    let mut circle = LinkedList::new();
    circle.push_front(0);
    // let mut current_index = 0;
    // let players = vec![1,2,3,4,5,6,7,8,9];
    let mut players = vec![0; players];
    let p_len = &players.len();
    // print_state(&"-", &circle);
    for i in 1..=final_point {
        if i % 23 == 0 {
            for _ in 0..6 {
                let first = circle.pop_back().unwrap();
                circle.push_front(first);
            }
            players[i % p_len] += i + circle.pop_back().unwrap();
            // print_state(&(i % p_len).to_string(), &circle);
            continue;
        }
        let first = circle.pop_front().unwrap();
        circle.push_back(first);
        let first = circle.pop_front().unwrap();
        circle.push_back(first);
        circle.push_front(i);
        // print_state(&(i % p_len).to_string(), &circle);
    }
    *players.iter().max().unwrap()
}

fn process_data_b(data: &str) -> usize {
    let (players, final_point) = get_data(data);
    let final_point = final_point * 100;
    let mut circle = LinkedList::new();
    circle.push_front(0);
    // let mut current_index = 0;
    // let players = vec![1,2,3,4,5,6,7,8,9];
    let mut players = vec![0; players];
    let p_len = &players.len();
    // print_state(&"-", &circle);
    for i in 1..=final_point {
        if i % 23 == 0 {
            for _ in 0..6 {
                let first = circle.pop_back().unwrap();
                circle.push_front(first);
            }
            players[i % p_len] += i + circle.pop_back().unwrap();
            // print_state(&(i % p_len).to_string(), &circle);
            continue;
        }
        let first = circle.pop_front().unwrap();
        circle.push_back(first);
        let first = circle.pop_front().unwrap();
        circle.push_back(first);
        circle.push_front(i);
        // print_state(&(i % p_len).to_string(), &circle);
    }
    *players.iter().max().unwrap()
}

//-----------------------------------------------------
// Questions.

q_impl!("9");

#[test]
fn a() {
    assert_eq!(
        process_data_a("9 players; last marble is worth 25 points"),
        32
    );
    assert_eq!(
        process_data_a("10 players; last marble is worth 1618 points"),
        8317
    );
    assert_eq!(
        process_data_a("13 players; last marble is worth 7999 points"),
        146373
    );
    assert_eq!(
        process_data_a("17 players; last marble is worth 1104 points"),
        2764
    );
    assert_eq!(
        process_data_a("21 players; last marble is worth 6111 points"),
        54718
    );
    assert_eq!(
        process_data_a("30 players; last marble is worth 5807 points"),
        37305
    );
}

#[test]
fn b() {}
