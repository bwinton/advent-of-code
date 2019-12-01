//-----------------------------------------------------
// Setup.

use glue::prelude::{
    alphabetic, digit, find, find_all, find_separated, is, take, take_all, whitespace, Parser,
};
use glue::types::MapParserResult;
use itertools::Itertools;

static INPUT: &str = include_str!("data/q21.data");

#[derive(Clone, Debug, PartialEq, Eq)]
struct Item {
    name: String,
    cost: i32,
    damage: i32,
    armor: i32,
}

#[derive(Clone, Debug)]
struct Group {
    name: String,
    items: Vec<Item>,
}

#[derive(Clone, Debug)]
struct Player {
    cost: i32,
    hp: i32,
    damage: i32,
    armor: i32,
    items: Vec<Item>,
}

impl Player {
    pub fn new(input: &(&Item, &Item, &Vec<&Item>)) -> Player {
        let mut items = Vec::new();
        items.push(input.0.clone());
        items.push(input.1.clone());
        for &item in input.2 {
            items.push(item.clone());
        }
        let cost = items.iter().map(|i| i.cost).sum();
        let damage = items.iter().map(|i| i.damage).sum();
        let armor = items.iter().map(|i| i.armor).sum();
        Player {
            cost,
            hp: 100,
            damage,
            armor,
            items,
        }
    }

    fn wins(&mut self) -> bool {
        let mut me = self.clone();
        let mut boss = Player {
            cost: 0,
            hp: 104,
            damage: 8,
            armor: 1,
            items: Vec::new(),
        };
        while me.hp > 0 {
            boss.hp -= 1.max(me.damage - boss.armor);
            if boss.hp <= 0 {
                break;
            }
            me.hp -= 1.max(boss.damage - me.armor);
        }
        me.hp > 0
    }
}

fn modifier_parser<'a>() -> impl Parser<'a, &'a str> {
    move |ctx| take_all((is(" +"), take(1.., is(digit)))).parse(ctx)
}

fn name_parser<'a>() -> impl Parser<'a, &'a str> {
    move |ctx| take_all((take(1.., is(alphabetic)), take(0..1, modifier_parser()))).parse(ctx)
}

fn header_parser<'a>() -> impl Parser<'a, &'a str> {
    move |ctx| {
        find_all((
            take(1.., is(alphabetic)),
            find_all((
                is(":"),
                take(1.., is(whitespace)),
                is("Cost"),
                take(1.., is(whitespace)),
                is("Damage"),
                take(1.., is(whitespace)),
                is("Armor\n"),
            )),
        ))
        .parse(ctx)
        .map_result(|(name, _)| name)
    }
}

fn item_parser<'a>() -> impl Parser<'a, Item> {
    move |ctx| {
        find_all((
            name_parser(),
            take(1.., is(whitespace)),
            take(1.., is(digit)),
            take(1.., is(whitespace)),
            take(1.., is(digit)),
            take(1.., is(whitespace)),
            take(1.., is(digit)),
            is("\n"),
        ))
        .parse(ctx)
        .map_result(|(name, _, cost, _, damage, _, armor, _)| Item {
            name: name.to_string(),
            cost: cost.parse().unwrap(),
            damage: damage.parse().unwrap(),
            armor: armor.parse().unwrap(),
        })
    }
}

fn group_parser<'a>() -> impl Parser<'a, Group> {
    move |ctx| {
        find_all((header_parser(), find(0.., item_parser())))
            .parse(ctx)
            .map_result(|(name, items)| {
                let empty_item = Item {
                    name: "None".to_owned(),
                    cost: 0,
                    damage: 0,
                    armor: 0,
                };
                let mut all_items = items.clone();
                if name != "Weapons" {
                    all_items.insert(0, empty_item.clone());
                }
                if name == "Rings" {
                    all_items.insert(0, empty_item);
                }
                Group {
                    name: name.to_string(),
                    items: all_items,
                }
            })
    }
}

fn store_parser<'a>() -> impl Parser<'a, Vec<Group>> {
    move |ctx| find_separated(0.., group_parser(), is("\n")).parse(ctx)
}

fn process_data_a(data: &str) -> i32 {
    let mut players = Vec::new();
    let store = store_parser().parse(data).unwrap().1;
    for items in iproduct!(
        store[0].items.iter(),
        store[1].items.iter(),
        store[2]
            .items
            .iter()
            .combinations(2)
            .collect::<Vec<_>>()
            .iter()
    ) {
        players.push(Player::new(&items));
    }
    players.sort_by_key(|x| x.cost);
    for mut player in players {
        if player.wins() {
            // println!("{:?}", player);
            return player.cost;
        }
    }
    0
}

// use std::thread;
// use std::sync::mpsc;

fn process_data_b(data: &str) -> i32 {
    let mut players = Vec::new();
    let store = store_parser().parse(data).unwrap().1;
    for items in iproduct!(
        store[0].items.iter(),
        store[1].items.iter(),
        store[2]
            .items
            .iter()
            .combinations(2)
            .collect::<Vec<_>>()
            .iter()
    ) {
        players.push(Player::new(&items));
    }
    players.sort_by_key(|x| -x.cost);
    for mut player in players {
        if !player.wins() {
            // println!("{:?}", player);
            return player.cost;
        }
    }
    0
    // let (tx, rx) = mpsc::channel();
    //
    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    // });
    //
    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);
}

//-----------------------------------------------------
// Questions.

q_impl!("21");

#[test]
fn a() {
    let mut player = Player {
        cost: 0,
        hp: 8,
        damage: 5,
        armor: 5,
        items: Vec::new(),
    };
    assert_eq!(player.wins(), false);
}

#[test]
fn b() {
    // assert_eq!(process_data_b(""), 0);
}
