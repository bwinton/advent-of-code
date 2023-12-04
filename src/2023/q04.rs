//-----------------------------------------------------
// Setup.

use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("data/q04.data");

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    for line in data.lines() {
        let (_card, lines) = line.split_once(':').unwrap();
        let (winning, test) = lines.split_once('|').unwrap();
        let winning: HashSet<&str> = winning.split_ascii_whitespace().collect();
        let test: HashSet<&str> = test.split_ascii_whitespace().collect();
        let wins: Vec<_> = test.intersection(&winning).collect();

        if !wins.is_empty() {
            rv += 2usize.pow(wins.len() as u32 - 1);
        }
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut card_results = HashMap::new();
    let mut cards = HashMap::new();
    for line in data.lines() {
        let (card, lines) = line.split_once(':').unwrap();
        let card: usize = card
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        let (winning, test) = lines.split_once('|').unwrap();
        let winning: HashSet<&str> = winning.split_ascii_whitespace().collect();
        let test: HashSet<&str> = test.split_ascii_whitespace().collect();
        let wins = test.intersection(&winning).collect::<Vec<_>>().len();
        card_results.insert(card, wins);
        cards.insert(card, 1usize);
    }

    for card in 1..cards.len() {
        let wins = card_results[&card];
        let mult = cards[&card];
        for i in card + 1..=card + wins {
            let entry = cards.get_mut(&i).unwrap();
            *entry += mult;
        }
    }

    let rv = cards.values().sum();
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("4");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "
        )),
        13
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "
        )),
        30
    );
}
