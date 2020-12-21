#![feature(iterator_fold_self)]

use commons::io::load_file_lines;
use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug)]
struct FoodItem {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

#[derive(Error, Debug)]
enum ParseError {}

impl FromStr for FoodItem {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ingredients = HashSet::new();
        let mut allergens = HashSet::new();
        let mut done_ingredients = false;

        for p in s.split_whitespace() {
            if !done_ingredients {
                if p == "(contains" {
                    done_ingredients = true;
                } else {
                    ingredients.insert(p.to_string());
                }
            } else {
                allergens.insert(p.chars().filter(|c| c.is_alphabetic()).collect::<String>());
            }
        }

        Ok(FoodItem {
            ingredients,
            allergens,
        })
    }
}

fn main() {
    let food_items: Vec<FoodItem> = load_file_lines("input.txt")
        .map(|res| res.unwrap())
        .collect();
    let mut allergens: HashMap<&String, Vec<&FoodItem>> = food_items
        .iter()
        .flat_map(|item| item.allergens.iter())
        .map(|allergen| (allergen, Vec::new()))
        .collect();

    for item in &food_items {
        for allergen in &item.allergens {
            allergens.get_mut(allergen).unwrap().push(item);
        }
    }

    let mut found = HashMap::new();
    let mut to_find: VecDeque<&String> = allergens.keys().copied().collect();
    while !to_find.is_empty() {
        let allergen_name = to_find.pop_front().unwrap();
        let possible_ingredients: HashSet<String> = allergens
            .get(allergen_name)
            .unwrap()
            .iter()
            .map(|item| item.ingredients.clone())
            .fold_first(|a, c| {
                a.intersection(&c)
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap()
            .iter()
            .filter(|s| {
                found.values().find(|x| x == s).is_none()
            })
            .map(|s| s.to_string())
            .collect();
        if possible_ingredients.len() == 1 {
            found.insert(
                allergen_name,
                possible_ingredients.iter().next().unwrap().clone(),
            );
        } else {
            to_find.push_back(allergen_name);
        }
    }

    let identified_ingredients: HashSet<&String> = found.values().collect();
    let part1: usize = food_items.iter().map(|item| item.ingredients.iter().filter(|i| !identified_ingredients.contains(i)).count()).sum();
    println!("{}", part1);
}
