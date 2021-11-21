//-----------------------------------------------------
// Setup.

use aoc::nom_util::unsigned_number;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending, space1},
    combinator::{eof, opt},
    multi::{many1, separated_list0},
    sequence::{terminated, tuple},
    IResult,
};

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
    _name: String,
    items: Vec<Item>,
}

#[derive(Clone, Debug)]
struct Player {
    cost: i32,
    hp: i32,
    damage: i32,
    armor: i32,
    _items: Vec<Item>,
}

impl Player {
    pub fn new(input: &(&Item, &Item, &Vec<&Item>)) -> Player {
        let mut items = vec![input.0.clone(), input.1.clone()];
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
            _items: items,
        }
    }

    fn wins(&mut self) -> bool {
        let mut me = self.clone();
        let mut boss = Player {
            cost: 0,
            hp: 104,
            damage: 8,
            armor: 1,
            _items: Vec::new(),
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

fn modifier(i: &str) -> IResult<&str, String> {
    let (input, (first, number)) = tuple((tag(" +"), digit1))(i)?;
    Ok((input, first.to_owned() + number))
}

fn name(i: &str) -> IResult<&str, String> {
    let (input, (first, number)) = tuple((alpha1, opt(modifier)))(i)?;
    Ok((input, first.to_owned() + &number.unwrap_or_default()))
}

fn header(i: &str) -> IResult<&str, &str> {
    let (input, (name, ..)) = tuple((
        alpha1,
        tag(":"),
        space1,
        tag("Cost"),
        space1,
        tag("Damage"),
        space1,
        tag("Armor\n"),
    ))(i)?;
    Ok((input, name))
}

fn item(i: &str) -> IResult<&str, Item> {
    let (input, (name, _, cost, _, damage, _, armor, _)) = tuple((
        name,
        space1,
        unsigned_number,
        space1,
        unsigned_number,
        space1,
        unsigned_number,
        opt(tag("\n")),
    ))(i)?;
    Ok((
        input,
        Item {
            name,
            cost: cost as i32,
            damage: damage as i32,
            armor: armor as i32,
        },
    ))
}

fn group(i: &str) -> IResult<&str, Group> {
    let (input, (name, items)) = tuple((header, many1(item)))(i)?;

    let empty_item = Item {
        name: "None".to_owned(),
        cost: 0,
        damage: 0,
        armor: 0,
    };
    let mut all_items = items;
    if name != "Weapons" {
        all_items.insert(0, empty_item.clone());
    }
    if name == "Rings" {
        all_items.insert(0, empty_item);
    }
    Ok((
        input,
        Group {
            _name: name.to_string(),
            items: all_items,
        },
    ))
}

fn store(i: &str) -> IResult<&str, Vec<Group>> {
    let (input, groups) = terminated(separated_list0(line_ending, group), eof)(i)?;
    Ok((input, groups))
}

fn process_data_a(data: &str) -> i32 {
    let mut players = Vec::new();
    let store = store(data).unwrap().1;
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
    let store = store(data).unwrap().1;
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
        _items: Vec::new(),
    };
    assert!(!player.wins());
}

#[test]
fn b() {
    // assert_eq!(process_data_b(""), 0);
}
