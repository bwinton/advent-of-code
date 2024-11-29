//-----------------------------------------------------
// Setup.

static INPUT: &str = "";

use once_cell::sync::Lazy;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Clone, Debug, Eq)]
struct Spell {
    name: String,
    cost: i32,
    damage: i32,
    healing: i32,
    armor: i32,
    mana: i32,
    duration: i32,
}

impl PartialEq for Spell {
    fn eq(&self, other: &Spell) -> bool {
        self.name == other.name
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Player {
    hp: i32,
    mana: i32,
    armor: i32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            hp: 50,
            mana: 500,
            armor: 0,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Boss {
    hp: i32,
    damage: i32,
}

impl Boss {
    pub fn new() -> Boss {
        Boss { hp: 55, damage: 8 }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    cost: i32,
    player: Player,
    boss: Boss,
    effects: Vec<Spell>,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        (-self.cost).cmp(&(-other.cost))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some((-self.cost).cmp(&(-other.cost)))
    }
}

static MAGIC_MISSLE: &str = "Magic Missle";
static DRAIN: &str = "Drain";
static SHIELD: &str = "Shield";
static POISON: &str = "Poison";
static RECHARGE: &str = "Recharge";

static SPELLS: Lazy<HashMap<&'static str, Spell>> = Lazy::new(|| {
    hashmap![
        MAGIC_MISSLE => Spell { name: MAGIC_MISSLE.to_string(), cost: 53, damage: 4, healing: 0, armor: 0, mana: 0, duration: -1 },
        DRAIN => Spell { name: DRAIN.to_string(), cost: 73, damage: 2, healing: 2, armor: 0, mana: 0, duration: -1 },
        SHIELD => Spell { name: SHIELD.to_string(), cost: 113, damage: 0, healing: 0, armor: 7, mana: 0, duration: 6 },
        POISON => Spell { name: POISON.to_string(), cost: 173, damage: 3, healing: 0, armor: 0, mana: 0, duration: 6 },
        RECHARGE => Spell { name: RECHARGE.to_string(), cost: 229, damage: 0, healing: 0, armor: 0, mana: 101, duration: 5 },
    ]
});

impl State {
    pub fn new() -> State {
        State {
            cost: 0,
            player: Player::new(),
            boss: Boss::new(),
            effects: Vec::new(),
        }
    }

    fn run_turn(&self, penalty: i32) -> Vec<State> {
        let mut rv = Vec::new();
        for spell in SPELLS.values() {
            if let Some(new_state) = self.cast(spell, penalty) {
                rv.push(new_state);
            }
        }
        rv
    }

    fn cast(&self, spell: &Spell, penalty: i32) -> Option<State> {
        let mut rv = self.clone();

        // Run my turn.
        rv.player.hp += penalty;
        if rv.player.hp <= 0 {
            return None;
        }

        rv.apply_effects();

        if rv.player.mana < spell.cost {
            return None;
        }
        if rv.effects.contains(spell) {
            return None;
        }

        rv.player.mana -= spell.cost;
        rv.cost += spell.cost;

        // Append the spell to the effects.
        rv.effects.push(spell.clone());

        // Run the boss's turn.
        rv.apply_effects();

        if rv.boss.hp <= 0 {
            return Some(rv);
        }

        rv.player.hp -= 1.max(rv.boss.damage - rv.player.armor);

        if rv.player.hp <= 0 { None } else { Some(rv) }
    }

    fn apply_effects(&mut self) {
        let mut rv = Vec::new();
        self.player.armor = 0;
        for spell in &self.effects {
            self.boss.hp -= spell.damage;
            self.player.hp += spell.healing;
            self.player.armor = self.player.armor.max(spell.armor);
            self.player.mana += spell.mana;
            let mut new_spell = spell.clone();
            new_spell.duration -= 1;
            if new_spell.duration > 0 {
                rv.push(new_spell)
            }
        }
        self.effects = rv;
    }

    fn player_wins(&self) -> bool {
        self.player.hp > 0 && self.boss.hp <= 0
    }
}

fn process_data_a(_: &str) -> i32 {
    let mut next = BinaryHeap::new();
    next.push(State::new());
    while !next.is_empty() {
        let state = next.pop().unwrap();
        if state.player_wins() {
            return state.cost;
        }
        for next_state in state.run_turn(0) {
            next.push(next_state);
        }
    }
    0
}

fn process_data_b(_: &str) -> i32 {
    let mut next = BinaryHeap::new();
    next.push(State::new());
    while !next.is_empty() {
        let state = next.pop().unwrap();
        if state.player_wins() {
            return state.cost;
        }
        for next_state in state.run_turn(-1) {
            next.push(next_state);
        }
    }
    0
}

//-----------------------------------------------------
// Questions.

q_impl!("22");

#[allow(clippy::cognitive_complexity)]
#[test]
fn a() {
    use pretty_assertions::assert_eq;

    // First example.
    let mut state = State::new();
    state.player.hp = 10;
    state.player.mana = 250;
    state.boss.hp = 13;
    state.boss.damage = 8;
    assert_eq!(state.player_wins(), false);

    let mut next = state.cast(&SPELLS[POISON], 0);
    assert_ne!(next, None);
    let mut test = next.unwrap();
    assert_eq!(test.player.hp, 2);
    assert_eq!(test.player.mana, 77);
    assert_eq!(test.boss.hp, 10);
    assert_eq!(test.effects.len(), 1);
    assert_eq!(test.effects[0].name, POISON.to_string());
    assert_eq!(test.effects[0].duration, 5);
    assert_eq!(test.player_wins(), false);

    next = test.cast(&SPELLS[MAGIC_MISSLE], 0);
    println!("{:?}", next);
    assert_ne!(next, None);
    test = next.unwrap();
    assert_eq!(test.player.hp, 2);
    assert_eq!(test.player.mana, 24);
    assert_eq!(test.boss.hp, 0);
    assert_eq!(test.effects.len(), 1);
    assert_eq!(test.effects[0].name, POISON.to_string());
    assert_eq!(test.effects[0].duration, 3);
    assert_eq!(test.player_wins(), true);

    // Second example.
    state = State::new();
    state.player.hp = 10;
    state.player.mana = 250;
    state.boss.hp = 14;
    state.boss.damage = 8;
    assert_eq!(state.player_wins(), false);

    let mut next = state.cast(&SPELLS[RECHARGE], 0);
    println!("{:?}", next);
    assert_ne!(next, None);
    let mut test = next.unwrap();
    assert_eq!(test.player.hp, 2);
    assert_eq!(test.player.mana, 122);
    assert_eq!(test.boss.hp, 14);
    assert_eq!(test.effects.len(), 1);
    assert_eq!(test.effects[0].name, RECHARGE.to_string());
    assert_eq!(test.effects[0].duration, 4);
    assert_eq!(test.player_wins(), false);

    next = test.cast(&SPELLS[SHIELD], 0);
    println!("{:?}", next);
    assert_ne!(next, None);
    test = next.unwrap();
    assert_eq!(test.player.hp, 1);
    assert_eq!(test.player.armor, 7);
    assert_eq!(test.player.mana, 211);
    assert_eq!(test.boss.hp, 14);
    assert_eq!(test.effects.len(), 2);
    assert_eq!(test.effects[0].name, RECHARGE.to_string());
    assert_eq!(test.effects[0].duration, 2);
    assert_eq!(test.effects[1].name, SHIELD.to_string());
    assert_eq!(test.effects[1].duration, 5);
    assert_eq!(test.player_wins(), false);

    next = test.cast(&SPELLS[DRAIN], 0);
    println!("{:?}", next);
    assert_ne!(next, None);
    test = next.unwrap();
    assert_eq!(test.player.hp, 2);
    assert_eq!(test.player.armor, 7);
    assert_eq!(test.player.mana, 340);
    assert_eq!(test.boss.hp, 12);
    assert_eq!(test.effects.len(), 1);
    assert_eq!(test.effects[0].name, SHIELD.to_string());
    assert_eq!(test.effects[0].duration, 3);
    assert_eq!(test.player_wins(), false);

    next = test.cast(&SPELLS[POISON], 0);
    println!("{:?}", next);
    assert_ne!(next, None);
    test = next.unwrap();
    assert_eq!(test.player.hp, 1);
    assert_eq!(test.player.armor, 7);
    assert_eq!(test.player.mana, 167);
    assert_eq!(test.boss.hp, 9);
    assert_eq!(test.effects.len(), 2);
    assert_eq!(test.effects[0].name, SHIELD.to_string());
    assert_eq!(test.effects[0].duration, 1);
    assert_eq!(test.effects[1].name, POISON.to_string());
    assert_eq!(test.effects[1].duration, 5);
    assert_eq!(test.player_wins(), false);

    next = test.cast(&SPELLS[MAGIC_MISSLE], 0);
    println!("{:?}", next);
    assert_ne!(next, None);
    test = next.unwrap();
    assert_eq!(test.player.hp, 1);
    assert_eq!(test.player.armor, 0);
    assert_eq!(test.player.mana, 114);
    assert_eq!(test.boss.hp, -1);
    assert_eq!(test.effects.len(), 1);
    assert_eq!(test.effects[0].name, POISON.to_string());
    assert_eq!(test.effects[0].duration, 3);
    assert_eq!(test.player_wins(), true);
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    // First example.
    let mut state = State::new();
    state.player.hp = 1;
    state.player.mana = 250;
    state.boss.hp = 13;
    state.boss.damage = 8;
    assert_eq!(state.player_wins(), false);

    let mut next = state.cast(&SPELLS[POISON], -1);
    assert_eq!(next, None);

    state.player.hp = 9;
    next = state.cast(&SPELLS[POISON], -1);
    assert_eq!(next, None);

    state.player.hp = 10;
    next = state.cast(&SPELLS[RECHARGE], -1);
    println!("{:?}", next);
    assert_ne!(next, None);
    let test = next.unwrap();
    assert_eq!(test.player.hp, 1);
    assert_eq!(test.player.mana, 122);
    assert_eq!(test.boss.hp, 13);
    assert_eq!(test.effects.len(), 1);
    assert_eq!(test.effects[0].name, RECHARGE.to_string());
    assert_eq!(test.effects[0].duration, 4);
    assert_eq!(test.player_wins(), false);
}
