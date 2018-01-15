//-----------------------------------------------------
// Setup.

use aoc::Day;

use itertools::Itertools;
use nom::alpha;
use nom::digit;
use nom::space;

static INPUT : &'static str = "Weapons:    Cost  Damage  Armor
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0

Armor:      Cost  Damage  Armor
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5

Rings:      Cost  Damage  Armor
Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3";

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
struct Item {
  name: String,
  cost: i32,
  damage: i32,
  armor: i32
}

#[derive(Clone)]
#[derive(Debug)]
struct Group {
  name: String,
  items: Vec<Item>
}

#[derive(Clone)]
#[derive(Debug)]
struct Player {
  cost: i32,
  hp: i32,
  damage: i32,
  armor: i32,
  items: Vec<Item>
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
      cost: cost,
      hp: 100,
      damage: damage,
      armor: armor,
      items: items
    }
  }

  fn wins(&mut self) -> bool {
    let mut me = self.clone();
    let mut boss = Player {
      cost: 0, hp: 104, damage: 8, armor: 1, items: Vec::new()
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

named!(modifier_parser<&str, String>, do_parse!(
  space >>
  tag!("+") >>
  digits: digit >>
  (" +".to_owned() + digits)
));

named!(name_parser<&str, String>, do_parse!(
  base: alpha >>
  modifier: opt!(modifier_parser) >>
  (base.to_owned() + &modifier.unwrap_or_default())
));

named!(header_parser<&str, String>, do_parse!(
  name: alpha >>
  tag!(":") >>
  space >>
  tag!("Cost") >>
  space >>
  tag!("Damage") >>
  space >>
  tag!("Armor") >>
  eat_separator!("\n") >>
  (name.to_owned())
));

named!(item_parser<&str, Item>, do_parse!(
  name: name_parser >>
  space >>
  cost: digit >>
  space >>
  damage: digit >>
  space >>
  armor: digit >>
  eat_separator!("\n") >>
  (Item { name: name.to_owned(), cost: cost.parse().unwrap(), damage: damage.parse().unwrap(), armor: armor.parse().unwrap()})
));

named!(group_parser<&str, Group>, do_parse!(
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
    Group { name: name.to_owned(), items: all_items }
  })
));

named!(store_parser<&str, Vec<Group>>, do_parse!(
  groups: many0!(group_parser) >>
  (groups)
));

fn process_data_a(data: &str) -> i32 {
  let mut players = Vec::new();
  let store = store_parser(data).unwrap().1;
  for items in iproduct!(
    store[0].items.iter(),
    store[1].items.iter(),
    store[2].items.iter().combinations(2).collect::<Vec<_>>().iter()
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
  let store = store_parser(data).unwrap().1;
  for items in iproduct!(
    store[0].items.iter(),
    store[1].items.iter(),
    store[2].items.iter().combinations(2).collect::<Vec<_>>().iter()
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
  assert_eq!(header_parser("Weapons:    Cost  Damage  Armor").unwrap().1, "Weapons".to_owned());
  assert_eq!(header_parser("Armor:      Cost  Damage  Armor").unwrap().1, "Armor".to_owned());
  assert_eq!(header_parser("Rings:      Cost  Damage  Armor").unwrap().1, "Rings".to_owned());

  let dagger = Item { name: "Dagger".to_owned(), cost: 8, damage: 4, armor: 0 };
  let banded_mail = Item { name: "Bandedmail".to_owned(), cost: 75, damage: 0, armor: 4 };
  let damage_3 = Item { name: "Damage +3".to_owned(), cost: 100, damage: 3, armor: 0 };
  let defense_2 = Item { name: "Defense +2".to_owned(), cost: 40, damage: 0, armor: 2 };
  assert_eq!(item_parser("Dagger        8     4       0").unwrap().1, dagger);
  assert_eq!(item_parser("Bandedmail   75     0       4").unwrap().1, banded_mail);
  assert_eq!(item_parser("Damage +3   100     3       0").unwrap().1, damage_3);
  assert_eq!(item_parser("Defense +2   40     0       2").unwrap().1, defense_2);

  let mut player = Player {
    cost: 0,
    hp: 8,
    damage: 5,
    armor: 5,
    items: Vec::new()
  };
  assert_eq!(player.wins(), true);
}

#[test]
fn b() {
  // assert_eq!(process_data_b(""), 0);
}
