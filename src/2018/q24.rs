//-----------------------------------------------------
// Setup.

use regex::Regex;
use std::borrow::ToOwned;
use std::collections::BTreeMap;
use std::str::Lines;

static INPUT: &str = include_str!("data/q24.data");

#[derive(Clone, Copy, Debug)]
enum Groups {
    ImmuneSystem,
    Infection,
}

#[derive(Clone, Debug)]
struct Group {
    group_type: Groups,
    units: i32,
    hp: i32,
    attack: i32,
    attack_type: String,
    initiative: i32,
    weaknesses: Vec<String>,
    immunities: Vec<String>,
    target: Option<i32>,
}

impl Group {
    fn power(&self) -> i32 {
        self.units * self.attack
    }

    fn get_target<'a>(
        &self,
        mut immunes: &'a mut Vec<i32>,
        mut infections: &'a mut Vec<i32>,
        groups: &'a BTreeMap<i32, Group>,
    ) -> Option<i32> {
        let opponents = match self.group_type {
            Groups::ImmuneSystem => &mut infections,
            Groups::Infection => &mut immunes,
        };
        let mut damage: Vec<_> = opponents
            .iter()
            .map(|i| (groups[i].get_damage(self), groups[i].power(), -i))
            .filter(|(d, _, _)| *d > 0)
            .collect();

        damage.sort();
        let rv = damage.pop().map(|(_, _, i)| -i);
        if let Some(initiative) = rv {
            opponents.retain(|&e| e != initiative);
        };
        rv
    }

    fn get_damage(&self, attacker: &Group) -> i32 {
        if self.immunities.contains(&attacker.attack_type) {
            0
        } else if self.weaknesses.contains(&attacker.attack_type) {
            attacker.power() * 2
        } else {
            attacker.power()
        }
    }

    fn attack(&self, groups: &mut BTreeMap<i32, Group>) -> bool {
        if let Some(target) = self.target {
            let target = groups.get_mut(&target).unwrap();
            let damage = target.get_damage(&self);
            if damage > 0 {
                let killed = damage / target.hp;
                if killed > 0 {
                    target.units -= killed;
                    return true;
                }
            }
        }
        false
    }
}

fn get_groups(lines: &mut Lines, groups: &mut BTreeMap<i32, Group>) -> Vec<i32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+) units each with (\d+) hit points (\([^)]*\) )?with an attack that does (\d+) ([a-z]+) damage at initiative (\d+)").unwrap();
        static ref IMMUNE_RE: Regex = Regex::new(r"immune to ([a-z, ]+)").unwrap();
        static ref WEAK_RE: Regex = Regex::new(r"weak to ([a-z, ]+)").unwrap();
    }

    // Skip the header.
    let group_type = if lines.next().unwrap() == "Immune System:" {
        Groups::ImmuneSystem
    } else {
        Groups::Infection
    };
    let mut rv = vec![];
    for line in lines {
        if line == "" {
            break;
        }
        if let Some(cap) = RE.captures(line) {
            let mut group = Group {
                group_type,
                units: cap[1].parse().unwrap(),
                hp: cap[2].parse().unwrap(),
                attack: cap[4].parse().unwrap(),
                attack_type: cap[5].to_string(),
                initiative: cap[6].parse().unwrap(),
                weaknesses: vec![],
                immunities: vec![],
                target: None,
            };
            group.initiative = -group.initiative;
            if let Some(data) = cap.get(3) {
                let mut modifiers = data.as_str();
                modifiers = &modifiers[1..modifiers.len() - 2];
                let modifiers = modifiers.split("; ").collect::<Vec<&str>>();
                for modifier in modifiers {
                    if let Some(cap) = IMMUNE_RE.captures(modifier) {
                        group.immunities = cap[1].split(", ").map(ToOwned::to_owned).collect();
                    } else if let Some(cap) = WEAK_RE.captures(modifier) {
                        group.weaknesses = cap[1].split(", ").map(ToOwned::to_owned).collect();
                    }
                }
            }
            rv.push(group.initiative);
            groups.insert(group.initiative, group);
        } else {
            println!("Couldn't understand '{}'", line);
        }
    }
    rv
}

fn process_data_a(data: &str) -> i32 {
    let mut lines = data.lines();
    let mut groups = BTreeMap::new();
    let mut immunes = get_groups(&mut lines, &mut groups);
    let mut infections = get_groups(&mut lines, &mut groups);

    while !immunes.is_empty() && !infections.is_empty() {
        // target selection
        let order = groups.clone();
        let mut order: Vec<_> = order.iter().map(|(i, group)| (group.power(), -i)).collect();
        order.sort();
        order.reverse();
        let mut available_immunes = immunes.clone();
        let mut available_infections = infections.clone();

        for (_p, i) in order {
            let target = groups[&-i].get_target(
                &mut available_immunes,
                &mut available_infections,
                &groups.clone(),
            );
            groups.get_mut(&-i).unwrap().target = target;
        }

        // attacking
        for attacker in groups.clone().values() {
            groups.clone()[&attacker.initiative].attack(&mut groups);
        }

        let mut to_remove = vec![];
        for group in groups.values() {
            if group.units <= 0 {
                to_remove.push(group.initiative);
            }
        }
        immunes.retain(|i| !to_remove.contains(i));
        infections.retain(|i| !to_remove.contains(i));
        for i in to_remove {
            groups.remove(&i);
        }
    }

    groups.iter().map(|(_, group)| group.units).sum()
}

fn process_data_b(data: &str) -> i32 {
    let mut lines = data.lines();
    let mut base_groups = BTreeMap::new();
    let base_immunes = get_groups(&mut lines, &mut base_groups);
    let base_infections = get_groups(&mut lines, &mut base_groups);

    let mut boost = 24;
    let mut remaining = 0;
    while remaining == 0 {
        boost += 1;
        let mut groups = base_groups.clone();
        let mut immunes = base_immunes.clone();
        let mut infections = base_infections.clone();
        for i in &immunes {
            groups.get_mut(i).unwrap().attack += boost;
        }

        while !immunes.is_empty() && !infections.is_empty() {
            // target selection
            let order = groups.clone();
            let mut order: Vec<_> = order.iter().map(|(i, group)| (group.power(), -i)).collect();
            order.sort();
            order.reverse();
            let mut available_immunes = immunes.clone();
            let mut available_infections = infections.clone();

            for (_p, i) in order {
                let target = groups[&-i].get_target(
                    &mut available_immunes,
                    &mut available_infections,
                    &groups.clone(),
                );
                groups.get_mut(&-i).unwrap().target = target;
            }

            // attacking
            let mut attacked = false;
            for attacker in groups.clone().values() {
                attacked |= groups.clone()[&attacker.initiative].attack(&mut groups);
            }

            if !attacked {
                // Nothing could land a hit. Bail out.
                println!("Stalemate!!!!");
                break;
            }

            let mut to_remove = vec![];
            for group in groups.values() {
                if group.units <= 0 {
                    to_remove.push(group.initiative);
                }
            }
            immunes.retain(|i| !to_remove.contains(i));
            infections.retain(|i| !to_remove.contains(i));
            for i in to_remove {
                groups.remove(&i);
            }
        }

        if !immunes.is_empty() && infections.is_empty() {
            remaining = groups.iter().map(|(_, group)| group.units).sum();
        }
    }

    // 33544 is too high.
    // 4428 !!!
    // 2076 not right.
    // 1842 is too low.
    // 1811 is too low.

    remaining
}

//-----------------------------------------------------
// Questions.

q_impl!("24");

#[test]
fn a() {
    assert_eq!(process_data_a("Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4"), 5216);
}

#[test]
fn b() {
    assert_eq!(process_data_b("Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4"), 51);
}
