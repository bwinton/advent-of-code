//-----------------------------------------------------
// Setup.

use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;

static INPUT: &str = include_str!("data/q21.data");

// Player 1 starting position: 5
static START_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new("^Player ([1-2]) starting position: ([0-9]+)$").unwrap());

fn get_players(data: &str) -> [(u32, u32); 2] {
    let mut players = [(0u32, 0u32); 2];
    for line in data.lines() {
        // Do something
        let captures = START_RE.captures(line).unwrap();
        let player: usize = captures[1].parse().unwrap();
        let position: u32 = captures[2].parse().unwrap();
        players[player - 1] = (position, 0);
    }
    players
}

fn process_data_a(data: &str) -> u32 {
    let mut players = get_players(data);
    let mut die = (1..=100).cycle();
    let mut roll_count = 0;
    let mut current_player = 0;
    while players[0].1 < 1000 && players[1].1 < 1000 {
        let rolls = [
            die.next().unwrap(),
            die.next().unwrap(),
            die.next().unwrap(),
        ];
        roll_count += 3;
        let turn: u32 = rolls.iter().sum();
        players[current_player].0 = (players[current_player].0 + turn - 1) % 10 + 1;
        players[current_player].1 += players[current_player].0 as u32;
        current_player = (current_player + 1) % 2;
    }
    players[current_player].1 * roll_count
}

fn process_data_b(data: &str) -> u64 {
    let players = get_players(data);

    let die = HashMap::from([(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]);
    let mut states = HashMap::from([(players, 1)]);
    let mut wins = [0u64, 0u64];
    let mut current_player = 0;

    // Keep track of scores -> number of universes in a HashMap…
    while !states.is_empty() {
        let mut next = HashMap::new();
        for (curr, count) in states {
            for (&turn, &turn_count) in &die {
                let mut players = curr;
                players[current_player].0 = (players[current_player].0 + turn - 1) % 10 + 1;
                players[current_player].1 += players[current_player].0;
                if players[current_player].1 >= 21 {
                    wins[current_player] += count * turn_count;
                    continue;
                }
                *next.entry(players).or_default() += count * turn_count;
            }
        }
        current_player = (current_player + 1) % 2;
        states = next;
    }
    *wins.iter().max().unwrap()
}

//-----------------------------------------------------
// Questions.

q_impl!("21");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "Player 1 starting position: 4
    Player 2 starting position: 8
    "
        )),
        739785
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "Player 1 starting position: 4
    Player 2 starting position: 8
    "
        )),
        444356092776315
    );
}
