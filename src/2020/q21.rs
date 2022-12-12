//-----------------------------------------------------
// Setup.

use std::collections::{HashMap, HashSet};

use once_cell::sync::Lazy;
use regex::Regex;

static INPUT: &str = include_str!("data/q21.data");

// mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
static FOOD_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(.*) \(contains (.*)\)$").unwrap());

fn process_data_a(data: &str) -> usize {
    let mut mapping: HashMap<String, HashSet<String>> = HashMap::new();
    let mut all_ingredients = vec![];
    for line in data.lines() {
        let captures = FOOD_RE.captures(line).unwrap();
        let ingredients: HashSet<String> = captures[1]
            .replace(',', "")
            .split_ascii_whitespace()
            .map(|x| x.to_string())
            .collect();
        all_ingredients.extend(ingredients.iter().cloned());
        let allergens: Vec<String> = captures[2]
            .replace(',', "")
            .split_ascii_whitespace()
            .map(|x| x.to_string())
            .collect();
        for allergen in allergens {
            // println!("{:?}: Adding {:?} to {:?}", &allergen, ingredients, mapping.get(&allergen));
            let entry = mapping
                .entry(allergen.clone())
                .or_insert_with(|| ingredients.clone());
            *entry = entry.intersection(&ingredients).cloned().collect();
            // println!("{:?}\n", mapping.get(&allergen));
        }
    }
    // println!("mapping: {:?}", mapping);
    all_ingredients.retain(|ingredient| {
        let mut safe = true;
        for ingredients in mapping.values() {
            if ingredients.contains(ingredient) {
                safe = false;
                break;
            }
        }
        safe
    });
    all_ingredients.len()
}

fn process_data_b(data: &str) -> String {
    let mut mapping: HashMap<String, HashSet<String>> = HashMap::new();
    let mut all_ingredients = vec![];
    for line in data.lines() {
        let captures = FOOD_RE.captures(line).unwrap();
        let ingredients: HashSet<String> = captures[1]
            .replace(',', "")
            .split_ascii_whitespace()
            .map(|x| x.to_string())
            .collect();
        all_ingredients.extend(ingredients.iter().cloned());
        let allergens: Vec<String> = captures[2]
            .replace(',', "")
            .split_ascii_whitespace()
            .map(|x| x.to_string())
            .collect();
        for allergen in allergens {
            let entry = mapping
                .entry(allergen.clone())
                .or_insert_with(|| ingredients.clone());
            *entry = entry.intersection(&ingredients).cloned().collect();
        }
    }
    // println!("mapping: {:?}", mapping);

    let mut dangerous_ingredients: HashMap<String, String> = HashMap::new();
    while !mapping.is_empty() {
        let mut ingredient = String::new();
        for (allergen, ingredients) in &mapping {
            if ingredients.len() == 1 {
                ingredient = ingredients.iter().next().unwrap().clone();
                // println!("Removing {}", &ingredient);
                dangerous_ingredients.insert(ingredient.clone(), allergen.clone());
            }
        }
        if !ingredient.is_empty() {
            for ingredients in mapping.values_mut() {
                ingredients.remove(&ingredient);
            }
        }
        mapping.retain(|_, v| !v.is_empty());
        // println!("mapping: {:?}", mapping);
    }

    // println!("dangerous: {:?}", dangerous_ingredients);
    let mut rv: Vec<_> = dangerous_ingredients.iter().collect();
    rv.sort_by_key(|(_, v)| *v);
    let rv: Vec<String> = rv.into_iter().map(|(k, _)| k).cloned().collect();

    rv.join(",")
}

//-----------------------------------------------------
// Questions.

q_impl!("21");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
            trh fvjkl sbzzf mxmxvkd (contains dairy)
            sqjhc fvjkl (contains soy)
            sqjhc mxmxvkd sbzzf (contains fish)"
        )),
        5
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
            trh fvjkl sbzzf mxmxvkd (contains dairy)
            sqjhc fvjkl (contains soy)
            sqjhc mxmxvkd sbzzf (contains fish)"
        )),
        "mxmxvkd,sqjhc,fvjkl".to_string()
    );
}
