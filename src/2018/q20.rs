//-----------------------------------------------------
// Setup.

use std::collections::HashMap;

static INPUT: &str = include_str!("data/q20.data");

#[derive(Clone, Debug)]
struct Room {
    x: i32,
    y: i32,
    distance: usize,
}

fn process_data_a(data: &str) -> usize {
    // Trim off the '^' and '$'
    let data = &data[1..data.len() - 1];
    // println!("len: {}, branches: {}, groups: {}",
    //     data.len() - 2,
    //     data.chars().filter(|&c| c == '|').count(),
    //     data.chars().filter(|&c| c == '(').count());

    let mut board = HashMap::new();
    let mut positions = Vec::new();
    let mut curr = Room {
        x: 0,
        y: 0,
        distance: 0,
    };
    board.insert((curr.x, curr.y), curr.distance);
    for c in data.chars() {
        // println!("{} - positions: {:?}, {:?}", c, curr, positions);
        match c {
            'N' => curr.y -= 1,
            'E' => curr.x += 1,
            'W' => curr.x -= 1,
            'S' => curr.y += 1,
            '(' => {
                // println!("New group!");
                positions.push(curr.clone());
                continue;
            }
            ')' => {
                // println!("End group!");
                curr = positions.pop().unwrap();
                continue;
            }
            '|' => {
                // println!("Branch!");
                curr = positions.pop().unwrap();
                positions.push(curr.clone());
                continue;
            }
            _ => {
                println!("Unknown character! {}", c);
                break;
            }
        }
        // Look for this position in the board.
        curr.distance += 1;
        if let Some(prev) = board.get(&(curr.x, curr.y)) {
            curr.distance = *prev;
        } else {
            board.insert((curr.x, curr.y), curr.distance);
        }
    }
    // println!("\n{:?}", board);
    *board.values().max().unwrap()
    // curr.distance
}

fn process_data_b(data: &str) -> usize {
    // Trim off the '^' and '$'
    let data = &data[1..data.len() - 1];
    // println!("len: {}, branches: {}, groups: {}",
    //     data.len() - 2,
    //     data.chars().filter(|&c| c == '|').count(),
    //     data.chars().filter(|&c| c == '(').count());

    let mut board = HashMap::new();
    let mut positions = Vec::new();
    let mut curr = Room {
        x: 0,
        y: 0,
        distance: 0,
    };
    board.insert((curr.x, curr.y), curr.distance);
    for c in data.chars() {
        // println!("{} - positions: {:?}, {:?}", c, curr, positions);
        match c {
            'N' => curr.y -= 1,
            'E' => curr.x += 1,
            'W' => curr.x -= 1,
            'S' => curr.y += 1,
            '(' => {
                // println!("New group!");
                positions.push(curr.clone());
                continue;
            }
            ')' => {
                // println!("End group!");
                curr = positions.pop().unwrap();
                continue;
            }
            '|' => {
                // println!("Branch!");
                curr = positions.pop().unwrap();
                positions.push(curr.clone());
                continue;
            }
            _ => {
                println!("Unknown character! {}", c);
                break;
            }
        }
        // Look for this position in the board.
        curr.distance += 1;
        if let Some(prev) = board.get(&(curr.x, curr.y)) {
            curr.distance = *prev;
        } else {
            board.insert((curr.x, curr.y), curr.distance);
        }
    }
    // println!("\n{:?}", board);
    board.values().filter(|&x| *x >= 1000).count()
    // curr.distance
}

//-----------------------------------------------------
// Questions.

q_impl!("20");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_a("^WNE$"), 3);
    assert_eq!(process_data_a("^ENWWW(NEEE|SSE(EE|N))$"), 10);
    assert_eq!(
        process_data_a("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"),
        18
    );
    assert_eq!(
        process_data_a("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$"),
        23
    );
    assert_eq!(
        process_data_a("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"),
        31
    );
}

#[test]
fn b() {}
