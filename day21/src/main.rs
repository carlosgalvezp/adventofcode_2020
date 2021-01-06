use std::env;
use std::fs;
use regex::Regex;
use std::collections::HashMap;

struct Meal{
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

struct AllergenCandidates{
    name: String,
    candidates: Vec<String>,
}

fn is_valid_ingredient_allergent_match(meals: &Vec<Meal>, ingredient: &String, allergen: &String) -> bool{
    for meal in meals{
        if meal.allergens.contains(allergen) && !meal.ingredients.contains(ingredient){
            return false;
        }
    }

    return true;
}

fn parse_input(contents: String) -> (Vec<Meal>, Vec<String>, Vec<String>, HashMap<String, String>){
    let mut meals : Vec<Meal> = Vec::new();
    let mut ingredients : Vec<String> = Vec::new();
    let mut allergens : Vec<String> = Vec::new();

    let re_ingredients_split = Regex::new(r"^([a-z ]+) \(contains ([a-z ,]+)\)$").unwrap();

    // Parse information into outputs
    for line in contents.lines(){
        let split_result = re_ingredients_split.captures(&line).unwrap();
        let ingredient_list = split_result[1].to_owned();
        let allergen_list = split_result[2].to_owned();

        let mut meal_ingredients : Vec<String> = Vec::new();
        for ingredient in ingredient_list.split(" "){
            meal_ingredients.push(ingredient.to_owned());

            if !ingredients.contains(&ingredient.to_owned()){
                ingredients.push(ingredient.to_owned());
            }
        }

        let mut meal_allergens : Vec<String> = Vec::new();
        for allergen in allergen_list.split(", "){
            meal_allergens.push(allergen.to_owned());

            if !allergens.contains(&allergen.to_owned()){
                allergens.push(allergen.to_owned());
            }
        }

        meals.push(Meal{ingredients: meal_ingredients, allergens: meal_allergens});
    }

    // Create map of allergens to ingredients
    // 1. Get all possible valid ingredients for each allergen
    let mut allergen_candidates = Vec::new();
    for allergen in &allergens{
        let mut valid_ingredients = Vec::new();
        for ingredient in &ingredients{
            if is_valid_ingredient_allergent_match(&meals, ingredient, allergen){
                valid_ingredients.push(ingredient.to_owned());
            }
        }
        allergen_candidates.push(AllergenCandidates{name: allergen.clone(), candidates: valid_ingredients.clone()});
    }

    // 2. Remove allergens by elimination
    let mut allergen_to_ingredient_map: HashMap<String, String> = HashMap::new();

    while !allergen_candidates.is_empty(){
        // Sort candidates according to least number of options
        // The first one must always have only one option
        allergen_candidates.sort_by(|a, b| a.candidates.len().cmp(&b.candidates.len()));

        assert!(allergen_candidates[0].candidates.len() == 1);

        let allergen = allergen_candidates[0].name.clone();
        let ingredient = allergen_candidates[0].candidates[0].clone();

        // Remove ingredient from the rest of candidates
        for i in 0..allergen_candidates.len(){
            if allergen_candidates[i].candidates.contains(&ingredient){
                let index = allergen_candidates[i].candidates.iter().position(|x| *x == ingredient).unwrap();
                allergen_candidates[i].candidates.remove(index);
            }
        }

        // Remove first candidate from list, since it has now been consumed
        allergen_candidates.remove(0);

        // Add info to hashmap
        allergen_to_ingredient_map.insert(allergen, ingredient);
    }

    return (meals, ingredients, allergens, allergen_to_ingredient_map);
}

fn day1(contents: String){
    let (meals, _, _, allergen_to_ingredient_map) = parse_input(contents);

    let mut output = 0;
    for meal in &meals{
        for ingredient in &meal.ingredients{
            if !allergen_to_ingredient_map.values().any(|x| x == ingredient){
                output += 1;
            }
        }
    }

    println!("Part 1 solution: {}", output);
}

fn day2(contents: String){
    let (_, _, mut allergens, allergen_to_ingredient_map) = parse_input(contents);
    allergens.sort();

    println!("Part 2 solution:");
    for allergen in allergens{
        print!("{},", allergen_to_ingredient_map[&allergen]);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let option: i32 = args[1].parse().unwrap();
    let fname = &args[2];
    let contents = fs::read_to_string(fname).expect("Something went wrong reading the file");

    if option == 1 {
        day1(contents);
    } else if option == 2 {
        day2(contents);
    } else {
        panic!("Wrong option!");
    }
}
