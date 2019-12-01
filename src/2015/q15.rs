//-----------------------------------------------------
// Setup.

use regex::Regex;
use std;
use std::str::FromStr;

static INPUT: &str = include_str!("data/q15.data");

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl FromStr for Ingredient {
    type Err = ();

    fn from_str(s: &str) -> Result<Ingredient, ()> {
        lazy_static! {
          static ref RE: Regex = Regex::new(r"^([A-Za-z]+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)$").unwrap();
        }
        let captures = RE.captures(s);
        match captures {
            Some(cap) => Ok(Ingredient {
                name: cap[1].to_string(),
                capacity: cap[2].parse().unwrap(),
                durability: cap[3].parse().unwrap(),
                flavor: cap[4].parse().unwrap(),
                texture: cap[5].parse().unwrap(),
                calories: cap[6].parse().unwrap(),
            }),
            None => Err(()),
        }
    }
}

impl Ingredient {
    fn add(&mut self, other: &Ingredient, amount: i32) {
        self.capacity += other.capacity * amount;
        self.durability += other.durability * amount;
        self.flavor += other.flavor * amount;
        self.texture += other.texture * amount;
        self.calories += other.calories * amount;
    }

    fn floor(&mut self) {
        self.capacity = self.capacity.max(0);
        self.durability = self.durability.max(0);
        self.flavor = self.flavor.max(0);
        self.texture = self.texture.max(0);
        self.calories = self.calories.max(0);
    }
}

fn get_score(amounts: &[i32], ingredients: &[Ingredient]) -> (i32, i32) {
    let mut sum = Ingredient {
        name: "Sum".to_string(),
        capacity: 0,
        durability: 0,
        flavor: 0,
        texture: 0,
        calories: 0,
    };
    for (i, amount) in amounts.iter().enumerate() {
        sum.add(&ingredients[i], *amount);
    }
    sum.floor();
    (
        sum.capacity * sum.durability * sum.flavor * sum.texture,
        sum.calories,
    )
}

fn hundred_iter(len: usize) -> impl Iterator<Item = Vec<i32>> {
    let mut curr: Vec<i32> = vec![];
    const MAX: i32 = 100;
    let curr_len: usize = len - 1;

    std::iter::from_fn(move || {
        if curr.is_empty() {
            for _ in 0..curr_len {
                curr.push(0)
            }
        } else {
            curr[curr_len - 1] += 1;
        }

        if curr[0] == MAX {
            return None;
        }

        let mut rest: i32 = MAX - curr.iter().sum::<i32>();

        let mut found = false;
        while rest < 0 {
            for i in 1..curr_len {
                if curr[curr_len - i] != 0 {
                    found = true;
                    curr[curr_len - i] = 0;
                    curr[curr_len - i - 1] += 1;
                    break;
                }
            }
            if !found {
                break;
            }
            rest = MAX - curr.iter().sum::<i32>();
        }
        if !found && rest < 0 {
            None
        } else {
            let mut rv = curr.clone();
            rv.push(rest);

            Some(rv)
        }
    })
}

fn process_data_a(data: &str) -> i32 {
    let mut ingredients: Vec<Ingredient> = Vec::new();
    for line in data.lines() {
        ingredients.push(line.parse().unwrap());
    }

    let mut max = 0;
    let iter = hundred_iter(ingredients.len());
    for x in iter {
        let score = get_score(&x, &ingredients).0;
        if score > max {
            max = score
        }
    }

    max
}

fn process_data_b(data: &str) -> i32 {
    let mut ingredients: Vec<Ingredient> = Vec::new();
    for line in data.lines() {
        ingredients.push(line.parse().unwrap());
    }

    let mut max = 0;
    let iter = hundred_iter(ingredients.len());
    for x in iter {
        let (score, calories) = get_score(&x, &ingredients);
        if score > max && calories == 500 {
            max = score
        }
    }

    max
}

//-----------------------------------------------------
// Questions.

q_impl!("15");

#[test]
fn a() {
    let test: Vec<Vec<i32>> = hundred_iter(2).take(5).collect();
    assert_eq!(
        test,
        vec![
            vec![0, 100],
            vec![1, 99],
            vec![2, 98],
            vec![3, 97],
            vec![4, 96]
        ]
    );

    assert_eq!(
        process_data_a(
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
        ),
        62842880
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
        ),
        57600000
    );
}
