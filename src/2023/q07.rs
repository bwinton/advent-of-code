//-----------------------------------------------------
// Setup.

use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;

static INPUT: &str = include_str!("data/q07.data");

const A_CARD_ORDER: &str = "23456789TJQKA";
const B_CARD_ORDER: &str = "J23456789TQKA";

#[derive(Debug)]
struct Hand {
    cards: Vec<usize>,
    card_values: Vec<(usize, usize)>,
    bid: usize,
    jokers: bool,
}

impl Hand {
    fn new(cards: Vec<char>, bid: usize, jokers: bool) -> Self {
        let order = if !jokers { A_CARD_ORDER } else { B_CARD_ORDER };
        let mut counts: HashMap<usize, usize> = HashMap::new();
        let cards: Vec<usize> = cards
            .iter()
            .map(|card| order.find(*card).unwrap())
            .collect();
        for card in &cards {
            *counts.entry(*card).or_default() += 1;
        }
        let mut card_values: Vec<(usize, usize)> = counts
            .into_iter()
            .sorted_by_key(|&(a, b)| (b, a))
            .map(|(a, b)| (b, a))
            .collect();
        card_values.reverse();
        if jokers
            && card_values[0].0 != 5
            && let Some(joker_pos) = card_values.iter().position(|&(_, value)| value == 0)
        {
            let update_index = if joker_pos == 0 { 1 } else { 0 };
            card_values[update_index].0 += card_values[joker_pos].0;
            card_values.remove(joker_pos);
        }
        Self {
            cards,
            card_values,
            bid,
            jokers,
        }
    }

    fn get_type(&self) -> usize {
        // higher = better.
        match self.card_values[0].0 {
            // Five of a kind,
            5 => 7,
            // Four of a kind
            4 => 6,
            3 => {
                if self.card_values[1].0 == 2 {
                    // Full house
                    5
                } else {
                    // Three of a kind
                    4
                }
            }
            2 => {
                if self.card_values[1].0 == 2 {
                    // Two pair
                    3
                } else {
                    // One pair
                    2
                }
            }
            // High card
            _ => 1,
        }
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let order = if !self.jokers {
            A_CARD_ORDER
        } else {
            B_CARD_ORDER
        };
        write!(
            f,
            "Hand{{ {}, {}, {}, {:?} }}",
            self.get_type(),
            self.cards
                .iter()
                .map(|&a| order.chars().nth(a).unwrap())
                .collect::<String>(),
            self.bid,
            self.card_values
        )
    }
}

fn process_data_a(data: &str) -> usize {
    let mut hands = vec![];
    for line in data.lines() {
        let (hand, bid) = line.split_once(' ').unwrap();
        let cards: Vec<char> = hand.chars().collect();
        let bid = bid.parse().unwrap();
        hands.push(Hand::new(cards, bid, false));
    }
    hands.sort_by_key(|hand| (hand.get_type(), hand.cards.clone()));
    hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

fn process_data_b(data: &str) -> usize {
    let mut hands = vec![];
    for line in data.lines() {
        let (hand, bid) = line.split_once(' ').unwrap();
        let cards: Vec<char> = hand.chars().collect();
        let bid = bid.parse().unwrap();
        hands.push(Hand::new(cards, bid, true));
    }
    hands.sort_by_key(|hand| (hand.get_type(), hand.cards.clone()));
    // for (i, hand) in hands.iter().enumerate() {
    //     println!("{}: {}", (i + 1), hand);
    // }
    hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

//-----------------------------------------------------
// Questions.

q_impl!("7");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483
    "
        )),
        6440
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483
    "
        )),
        5905
    );
}
