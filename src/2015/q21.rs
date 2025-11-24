//-----------------------------------------------------
// Setup.

use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, i32, line_ending, space1},
    combinator::{eof, opt},
    multi::{many1, separated_list0},
    sequence::terminated,
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
    let (input, (first, number)) = (tag(" +"), digit1).parse(i)?;
    Ok((input, first.to_owned() + number))
}

fn name(i: &str) -> IResult<&str, String> {
    let (input, (first, number)) = (alpha1, opt(modifier)).parse(i)?;
    Ok((input, first.to_owned() + &number.unwrap_or_default()))
}

fn header(i: &str) -> IResult<&str, &str> {
    let (input, (name, ..)) = (
        alpha1,
        tag(":"),
        space1,
        tag("Cost"),
        space1,
        tag("Damage"),
        space1,
        tag("Armor\n"),
    )
        .parse(i)?;
    Ok((input, name))
}

fn item(i: &str) -> IResult<&str, Item> {
    let (input, (name, _, cost, _, damage, _, armor, _)) =
        (name, space1, i32, space1, i32, space1, i32, opt(tag("\n"))).parse(i)?;
    Ok((
        input,
        Item {
            name,
            cost,
            damage,
            armor,
        },
    ))
}

fn group(i: &str) -> IResult<&str, Group> {
    let (input, (name, items)) = (header, many1(item)).parse(i)?;

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
    let (input, groups) = terminated(separated_list0(line_ending, group), eof).parse(i)?;
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
    use pretty_assertions::assert_eq;

    let mut player = Player {
        cost: 0,
        hp: 8,
        damage: 5,
        armor: 5,
        _items: Vec::new(),
    };
    assert_eq!(player.wins(), false);
}

#[test]
fn b() {
    // use pretty_assertions::assert_eq;

    // assert_eq!(process_data_b(""), 0);
}
