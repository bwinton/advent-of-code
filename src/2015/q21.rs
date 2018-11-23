//-----------------------------------------------------
// Setup.

use aoc::Day;

use itertools::Itertools;
use nom::alpha;
use nom::digit;
use nom::space;
use nom::types::CompleteStr;

static INPUT: &'static str = include_str!("data/q21.data");

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
struct Item {
  name: String,
  cost: i32,
  damage: i32,
  armor: i32,
}

#[derive(Clone)]
#[derive(Debug)]
struct Group {
  name: String,
  items: Vec<Item>,
}

#[derive(Clone)]
#[derive(Debug)]
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

named!(modifier_parser<CompleteStr, String>, do_parse!(
  space >>
  tag!("+") >>
  digits: digit >>
  (" +".to_owned() + &digits)
));

named!(name_parser<CompleteStr, String>, do_parse!(
  base: alpha >>
  modifier: opt!(modifier_parser) >>
  (base.to_string() + &modifier.unwrap_or_default())
));

named!(header_parser<CompleteStr, String>, complete!(do_parse!(
  name: alpha >>
  tag!(":") >>
  space >>
  tag!("Cost") >>
  space >>
  tag!("Damage") >>
  space >>
  tag!("Armor") >>
  eat_separator!("\n") >>
  (name.to_string())
)));

named!(item_parser<CompleteStr, Item>, do_parse!(
  name: name_parser >>
  space >>
  cost: digit >>
  space >>
  damage: digit >>
  space >>
  armor: digit >>
  eat_separator!("\n") >>
  (Item { name: name.to_string(), cost: cost.parse().unwrap(), damage: damage.parse().unwrap(), armor: armor.parse().unwrap()})
));

named!(group_parser<CompleteStr, Group>, do_parse!(
  name: header_parser >>
  items: many0!(item_parser) >>
  eat_separator!("\n") >>
  ({let empty_item = Item { name: "None".to_owned(), cost: 0, damage: 0, armor: 0 };
    let mut all_items = items.clone();
    if name != "Weapons" {
      all_items.insert(0, empty_item.clone());
    }
    if name == "Rings" {
      all_items.insert(0, empty_item);
    }
    Group { name: name.to_string(), items: all_items }
  })
));

named!(store_parser<CompleteStr, Vec<Group>>, do_parse!(
  groups: many0!(group_parser) >>
  (groups)
));

fn process_data_a(data: &str) -> i32 {
  let mut players = Vec::new();
  let store = store_parser(CompleteStr(data)).unwrap().1;
  for items in iproduct!(
    store[0].items.iter(),
    store[1].items.iter(),
    store[2]
      .items
      .iter()
      .combinations(2)
      .collect::<Vec<_>>()
      .iter()
  )
  {
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
  let store = store_parser(CompleteStr(data)).unwrap().1;
  for items in iproduct!(
    store[0].items.iter(),
    store[1].items.iter(),
    store[2]
      .items
      .iter()
      .combinations(2)
      .collect::<Vec<_>>()
      .iter()
  )
  {
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

pub struct Q;

impl Day for Q {
  fn number(&self) -> String {
    String::from("21")
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    let result = process_data_a(INPUT);
    println!("Result = {}", result);
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    let result = process_data_b(INPUT);
    println!("Result = {}", result);
  }
}

#[test]
fn a() {
  assert_eq!(
    header_parser(CompleteStr("Weapons:    Cost  Damage  Armor")).unwrap().1,
    "Weapons".to_owned()
  );
  assert_eq!(header_parser(CompleteStr("Armor:      Cost  Damage  Armor")).unwrap().1, "Armor".to_owned());
  assert_eq!(header_parser(CompleteStr("Rings:      Cost  Damage  Armor")).unwrap().1, "Rings".to_owned());

  let dagger = Item {
    name: "Dagger".to_owned(),
    cost: 8,
    damage: 4,
    armor: 0,
  };
  let banded_mail = Item {
    name: "Bandedmail".to_owned(),
    cost: 75,
    damage: 0,
    armor: 4,
  };
  let damage_3 = Item {
    name: "Damage +3".to_owned(),
    cost: 100,
    damage: 3,
    armor: 0,
  };
  let defense_2 = Item {
    name: "Defense +2".to_owned(),
    cost: 40,
    damage: 0,
    armor: 2,
  };
  assert_eq!(item_parser(CompleteStr("Dagger        8     4       0")).unwrap().1, dagger);
  assert_eq!(item_parser(CompleteStr("Bandedmail   75     0       4")).unwrap().1, banded_mail);
  assert_eq!(item_parser(CompleteStr("Damage +3   100     3       0")).unwrap().1, damage_3);
  assert_eq!(item_parser(CompleteStr("Defense +2   40     0       2")).unwrap().1, defense_2);

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
