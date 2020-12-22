//-----------------------------------------------------
// Setup.

use std::collections::{HashSet, VecDeque};

static INPUT: &str = include_str!("data/q22.data");

fn process_data_a(data: &str) -> usize {
    let mut players: [VecDeque<usize>; 2] = [VecDeque::new(), VecDeque::new()];
    let mut curr = -1;
    for line in data.lines() {
        if line.starts_with("Player ") {
            curr += 1;
            continue;
        }
        if line.is_empty() {
            continue;
        }
        players[curr as usize].push_back(line.parse().unwrap());
    }

    while !players[0].is_empty() && !players[1].is_empty() {
        let card_1 = players[0].pop_front().unwrap();
        let card_2 = players[1].pop_front().unwrap();
        if card_1 > card_2 {
            players[0].push_back(card_1);
            players[0].push_back(card_2);
        } else {
            players[1].push_back(card_2);
            players[1].push_back(card_1);
        }
    }

    let winner = if players[0].is_empty() { 1 } else { 0 };
    let mut rv = 0;
    let mut multiple = 1;
    for i in players[winner].iter().rev() {
        rv += i * multiple;
        multiple += 1;
    }
    rv
}

fn play(game: usize, players: [VecDeque<usize>; 2]) -> [VecDeque<usize>; 2] {
    let mut seen = HashSet::new();
    let mut players = players;

    while !players[0].is_empty() && !players[1].is_empty() {
        if seen.contains(&players) {
            return [players[0].clone(), VecDeque::new()];
        }
        seen.insert(players.clone());
        let card_1 = players[0].pop_front().unwrap();
        let card_2 = players[1].pop_front().unwrap();

        let winner = if players[0].len() >= card_1 && players[1].len() >= card_2 {
            let sub_players = [
                players[0].iter().take(card_1).cloned().collect(),
                players[1].iter().take(card_2).cloned().collect(),
            ];
            let result = play(game + 1, sub_players);
            if result[0].is_empty() {
                1
            } else {
                0
            }
        } else if card_1 > card_2 {
            0
        } else {
            1
        };
        if winner == 0 {
            players[0].push_back(card_1);
            players[0].push_back(card_2);
        } else {
            players[1].push_back(card_2);
            players[1].push_back(card_1);
        }
    }

    players
}

fn process_data_b(data: &str) -> usize {
    let mut players: [VecDeque<usize>; 2] = [VecDeque::new(), VecDeque::new()];
    let mut curr = -1;
    for line in data.lines() {
        if line.starts_with("Player ") {
            curr += 1;
            continue;
        }
        if line.is_empty() {
            continue;
        }
        players[curr as usize].push_back(line.parse().unwrap());
    }

    let players = play(1, players);

    let winner = if players[0].is_empty() { 1 } else { 0 };

    let mut rv = 0;
    let mut multiple = 1;
    for i in players[winner].iter().rev() {
        rv += i * multiple;
        multiple += 1;
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("22");

#[test]
fn a() {
    assert_eq!(
        process_data_a(
            "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"
        ),
        306
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(
            "Player 1:
43
19

Player 2:
2
29
14"
        ),
        105
    );

    assert_eq!(
        process_data_b(
            "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"
        ),
        291
    );
}
