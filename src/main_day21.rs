use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;
use crate::utils;

type Ingredient = String;
type Allergen = String;
type IngredientList = Vec<Ingredient>;
type IngredientSet = HashSet<Ingredient>;
type AllergenList = Vec<Allergen>;
type AllergenSet = HashSet<Allergen>;

lazy_static! { static ref RE: Regex = Regex::new(r"^(.*) \(contains (.*)\)$").unwrap(); }

fn solve(input: &Vec<(HashSet<String>, HashSet<String>)>) -> (IngredientSet, HashMap<Ingredient, Allergen>, HashMap<Allergen, Ingredient>) {

    let mut data = input.clone();
    let mut ingrediants_with_no_allergen = HashSet::new() as IngredientSet;
    let mut ingrediants_allergens_map = HashMap::new() as HashMap<Ingredient, Allergen>;
    let mut allergens_ingredients_map = HashMap::new() as HashMap<Allergen, Ingredient>;

    loop {
        let ingredients_set = data.iter().map(|x| x.0.clone()).flatten().collect::<IngredientSet>();
        step_solve(&data, ingredients_set, &mut ingrediants_with_no_allergen, &mut ingrediants_allergens_map, &mut allergens_ingredients_map);

        // simplify by removing already solved items
        data = data.iter()
            .map(|x| {
                let mut ingredients = x.0.clone();
                ingrediants_with_no_allergen.iter().for_each(|i| { ingredients.remove(i); });
                ingrediants_allergens_map.keys().for_each(|k| { ingredients.remove(k); });
                let mut allergens = x.1.clone();
                allergens_ingredients_map.keys().for_each(|k| {allergens.remove(k);});
                (ingredients, allergens)
            })
            .filter(|x| !x.0.is_empty() && !x.1.is_empty())
            .collect();

        if data.is_empty() { break; }
    }

    (ingrediants_with_no_allergen, ingrediants_allergens_map, allergens_ingredients_map)
}

fn step_solve(data: &Vec<(HashSet<String>, HashSet<String>)>, ingredients_set: HashSet<String>, ingrediants_with_no_allergen: &mut HashSet<String>, ingrediants_allergens_map: &mut HashMap<String, String>, allergens_ingredients_map: &mut HashMap<String, String>) {
    for ingredient in ingredients_set {
        let possible_allergens = data.iter()
            .filter(|x| x.0.contains(&ingredient))
            .map(|x| x.1.clone()).flatten().collect::<AllergenSet>();

        let ingredient_matches = possible_allergens.iter()
            .cloned()
            .filter(|possible_allergen| {
                data.iter()
                    .filter(|x| x.1.contains(possible_allergen))
                    .all(|x| x.0.contains(&ingredient))
            })
            .collect::<AllergenSet>();

        if ingredient_matches.is_empty() {
            ingrediants_with_no_allergen.insert(ingredient.clone());
        } else if ingredient_matches.len() == 1 {
            let allergen = ingredient_matches.iter().next().unwrap();
            ingrediants_allergens_map.insert(ingredient.clone(), allergen.to_owned());
            allergens_ingredients_map.insert(allergen.to_owned(), ingredient.clone());
        }
    }
}

pub fn main() -> std::io::Result<()> {

    fn parse_line(s: &String) -> (IngredientSet, AllergenSet) {
        let matches = RE.captures(s).unwrap();
        let ingredients = matches.get(1).unwrap().as_str().split(" ").map(|s| s.to_string()).collect::<IngredientSet>();
        let allergen = matches.get(2).unwrap().as_str().split(", ").map(|s| s.to_string()).collect::<AllergenSet>();
        (ingredients, allergen)
    }

    let data = utils::read_lines_from_file("inputs/day21/input.txt")?.iter()
        .map(parse_line)
        .collect::<Vec<(IngredientSet, AllergenSet)>>();

    /* Solve the list */
    let solution = solve(&data);

    let result1 = solution.0.iter()
        .map(|ingredient| data.iter().filter(|x| x.0.contains(ingredient)).count())
        .sum::<usize>();
    println!("Result day21 part1: {}", result1);

    let mut allergens = solution.2.keys().cloned().collect::<AllergenList>();
    allergens.sort();
    let result2 = allergens.iter()
        .map(|allergen| solution.2.get(allergen).unwrap())
        .cloned().collect::<IngredientList>().join(",");

    println!("Result day21 part2: {}", result2);
    Ok(())
}