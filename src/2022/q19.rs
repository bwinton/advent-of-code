//-----------------------------------------------------
// Setup.

use std::collections::HashMap;

use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    multi::separated_list1,
    sequence::tuple,
};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

static INPUT: &str = include_str!("data/q19.data");

#[derive(Debug)]
struct Recipe {
    product: String,
    ingredients: Vec<(String, u32)>,
}

#[derive(Debug)]
struct Blueprint {
    id: u32,
    recipes: HashMap<String, Recipe>,
    max_costs: HashMap<String, u32>,
}

#[derive(Debug, Clone)]
struct State {
    time: usize,
    ores: HashMap<String, u32>,
    robots: HashMap<String, u32>,
}
impl State {
    fn get_next(&self, blueprint: &Blueprint, time_limit: usize) -> Vec<State> {
        let mut rv = vec![];

        if self.time > time_limit {
            return rv;
        }

        let robot = "geode";
        // Find out how long it'll take to make.
        let recipe = &blueprint.recipes[robot];
        let mut wait_time = 0;
        for ingredient in &recipe.ingredients {
            if self.ores[&ingredient.0] >= ingredient.1 {
                // We've got enough for this!
                continue;
            }
            // Otherwise, we've got to wait a while.
            let missing = ingredient.1 - self.ores[&ingredient.0];
            let robots = self.robots[&ingredient.0];
            if robots == 0 {
                // We'll never be able to build this, so say it'll take too long.
                wait_time = time_limit + 100;
                break;
            }
            let mut time = (missing / robots) as usize;
            if missing % robots != 0 {
                // If there's a remaineder, add an extra minute to account for it.
                time += 1;
            }
            wait_time = wait_time.max(time);
        }
        wait_time += 1;

        // If it's too long, skip it.
        if self.time + wait_time <= time_limit {
            // Skip forward that many minutes and build the robot!
            let mut next = self.clone();
            next.time += wait_time;
            for robot in next.robots.iter_mut() {
                *next.ores.get_mut(robot.0).unwrap() += *robot.1 * wait_time as u32;
            }
            for ingredient in &recipe.ingredients {
                assert!(next.ores[&ingredient.0] >= ingredient.1);
                *next.ores.get_mut(&ingredient.0).unwrap() -= ingredient.1;
            }
            // Just add the geodes, instead of tracking the robots.
            *next.ores.get_mut("geode").unwrap() += (time_limit - next.time) as u32;
            rv.push(next);
            if wait_time == 1 {
                // If we can build a geode robot now, don't consider any other options.
                return rv;
            }
        }

        // Try to build a robot, or wait.
        for robot in ["obsidian", "clay", "ore"] {
            // Do we have enough? If so, skip it.

            // From Reddit:
            // if you already have X robots creating resource R,
            // a current stock of Y for that resource,
            // T minutes left, and no robot requires more than Z of resource R to build,
            // and X * T+Y >= T * Z, then you never need to build another robot mining R anymore.
            let x = self.robots[robot] as u64;
            let y = self.ores[robot] as u64;
            let z = blueprint.max_costs[robot] as u64;
            let t = (time_limit - self.time) as u64;
            if x * y + y >= t * z {
                continue;
            }

            if self.robots[robot] >= blueprint.max_costs[robot] {
                continue;
            }

            // Find out how long it'll take to make.
            let recipe = &blueprint.recipes[robot];
            let mut wait_time = 0;
            for ingredient in &recipe.ingredients {
                if self.ores[&ingredient.0] >= ingredient.1 {
                    // We've got enough for this!
                    continue;
                }
                // Otherwise, we've got to wait a while.
                let missing = ingredient.1 - self.ores[&ingredient.0];
                let robots = self.robots[&ingredient.0];
                if robots == 0 {
                    // We'll never be able to build this, so say it'll take too long.
                    wait_time = time_limit + 100;
                    break;
                }
                let mut time = (missing / robots) as usize;
                if missing % robots != 0 {
                    // If there's a remaineder, add an extra minute to account for it.
                    time += 1;
                }
                wait_time = wait_time.max(time);
            }
            wait_time += 1;

            // If it's too long, skip it.
            if self.time + wait_time > time_limit {
                continue;
            }

            // Skip forward that many minutes and build the robot!
            let mut next = self.clone();
            next.time += wait_time;
            for robot in next.robots.iter_mut() {
                *next.ores.get_mut(robot.0).unwrap() += *robot.1 * wait_time as u32;
            }
            for ingredient in &recipe.ingredients {
                assert!(next.ores[&ingredient.0] >= ingredient.1);
                *next.ores.get_mut(&ingredient.0).unwrap() -= ingredient.1;
            }
            if robot == "geode" {
                // Just add the geodes, instead of tracking the robots.
                *next.ores.get_mut("geode").unwrap() += (time_limit - next.time) as u32;
            } else {
                *next.robots.get_mut(robot).unwrap() += 1;
            }
            rv.push(next);
        }

        // If we haven't built any robots, then wait until the end.
        if rv.is_empty() && self.time <= time_limit {
            let mut next = self.clone();
            let wait_time = time_limit - next.time;
            next.time += wait_time;
            for robot in next.robots.iter_mut() {
                *next.ores.get_mut(robot.0).unwrap() += *robot.1 * wait_time as u32;
            }
            rv.push(next);
        }

        rv.reverse();

        rv
    }

    fn get_max_geodes(&self, time_limit: usize) -> u32 {
        let time_remaining = (time_limit - self.time) as u32;
        self.ores["geode"] + time_remaining * (time_remaining + 1) / 2
    }
}

// 2 ore
fn ingredient(i: &str) -> IResult<&str, (String, u32)> {
    let (input, (size, _, name)) = tuple((complete::u32, tag(" "), alpha1))(i)?;

    Ok((input, (name.to_owned(), size)))
}

// Each obsidian robot costs 2 ore and 20 clay.
fn recipe(i: &str) -> IResult<&str, Recipe> {
    let (input, (_, name, _, ingredients, _)) = tuple((
        tag("Each "),
        alpha1,
        tag(" robot costs "),
        separated_list1(tag(" and "), ingredient),
        tag("."),
    ))(i)?;

    Ok((input, Recipe {
        product: name.to_owned(),
        ingredients,
    }))
}

// Blueprint 1: Each ore robot costs 2 ore. Each clay robot costs 2 ore. Each obsidian robot costs 2 ore and 20 clay. Each geode robot costs 2 ore and 14 obsidian.
fn blueprint(i: &str) -> IResult<&str, Blueprint> {
    let (input, (_, id, _, recipes)) = tuple((
        tag("Blueprint "),
        complete::u32,
        tag(": "),
        separated_list1(tag(" "), recipe),
    ))(i)?;
    let recipes = HashMap::from_iter(recipes.into_iter().map(|r| (r.product.clone(), r)));
    let mut max_costs = HashMap::new();
    for costs in recipes.values() {
        for (ore, cost) in costs.ingredients.iter() {
            let entry = max_costs.entry(ore.clone()).or_insert(0);
            *entry = *cost.max(entry);
        }
    }
    Ok((input, Blueprint {
        id,
        recipes,
        max_costs,
    }))
}

fn parser(i: &str) -> IResult<&str, Vec<Blueprint>> {
    let (input, list) = separated_list1(line_ending, blueprint)(i)?;
    Ok((input, list))
}

fn run_blueprint(blueprint: &Blueprint, time_limit: usize) -> u32 {
    let mut states = vec![];
    let mut max = 0;
    states.push(State {
        time: 0,
        ores: HashMap::from_iter([
            ("ore".to_owned(), 0),
            ("clay".to_owned(), 0),
            ("obsidian".to_owned(), 0),
            ("geode".to_owned(), 0),
        ]),
        robots: HashMap::from_iter([
            ("ore".to_owned(), 1),
            ("clay".to_owned(), 0),
            ("obsidian".to_owned(), 0),
        ]),
    });

    // Use a heap of states, not a simulation of minutes…
    while let Some(state) = states.pop() {
        if state.ores["geode"] > max {
            max = state.ores["geode"];
        }

        for next_move in state.get_next(blueprint, time_limit) {
            if state.get_max_geodes(time_limit) > max {
                states.push(next_move);
            }
        }
    }
    max
}

fn process_data_a(data: &str) -> u32 {
    const TIME_LIMIT: usize = 24;
    let blueprints = parser(data).unwrap().1;

    // 817
    blueprints
        .par_iter()
        .map(|blueprint| blueprint.id * run_blueprint(blueprint, TIME_LIMIT))
        .reduce(|| 0, |a, b| a + b)
}

fn process_data_b(data: &str) -> u32 {
    const TIME_LIMIT: usize = 32;
    let mut blueprints = parser(data).unwrap().1;
    blueprints.truncate(3);

    // 4216
    blueprints
        .par_iter()
        .map(|blueprint| run_blueprint(blueprint, TIME_LIMIT))
        .reduce(|| 1, |a, b| a * b)
}

//-----------------------------------------------------
// Questions.

q_impl!("19");

#[test]
fn a() {
    use pretty_assertions::assert_eq;
    assert_eq!(process_data_a(indoc!("
        Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
    ")), 9);

    assert_eq!(process_data_a(indoc!("
        Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
        Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
    ")), 33);
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b(indoc!("
        Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
    ")), 56);

    assert_eq!(process_data_b(indoc!("
        Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
        Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
    ")), 3472);
}
