use std::env;
use std::fs;
use regex::Regex;

struct BagQuantity{
    name: String,
    quantity: i32,
}

struct BagInfo {
    name: String,
    bags: Vec<BagQuantity>,
}

fn parse_contents(contents: String) -> Vec<BagInfo>{
    let re_full = Regex::new(r"^([\w ]+) bags contain (.*)\.$").unwrap();
    let re_contains = Regex::new(r"(\d+) ([\w ]+) bag[s]?").unwrap();

    // Parse input into vector of BagInfo
    let mut bags = Vec::new();

    for line in contents.lines() {
        let full_sentence_parse = re_full.captures(&line);
        if full_sentence_parse.is_some(){
            let full_sentence_parse_data = full_sentence_parse.unwrap();
            let bag_name = full_sentence_parse_data[1].to_owned();
            let bag_contents = full_sentence_parse_data[2].to_owned();

            let mut contained_bags : Vec<BagQuantity> = Vec::new();

            for bag_str in bag_contents.split(","){
                let bag_quantity = re_contains.captures(&bag_str);
                if bag_quantity.is_some(){
                    let bag_quantity_data = bag_quantity.unwrap();
                    contained_bags.push(BagQuantity{name: bag_quantity_data[2].to_owned(),
                                                    quantity: bag_quantity_data[1].parse::<i32>().unwrap()});
                }
            }

            bags.push(BagInfo{name: bag_name, bags: contained_bags});
        }
    }

    return bags;
}

fn get_nr_bags(bags: &Vec<BagInfo>, bag_name: &str) -> i32 {
    let mut n_bags = 0;

    for bag in bags{
        if bag.name == bag_name{
            for contained_bag in &bag.bags {
                n_bags += contained_bag.quantity * (1 + get_nr_bags(bags, &contained_bag.name));
            }

            break;
        }
    }

    return n_bags;
}

fn day1(contents: String) {
    let bags = parse_contents(contents);

    // Search all bags that can contain (directly or transitively) a shiny gold bag
    // Use breadth-first search to do the search
    let mut to_search_bags: Vec<String> = Vec::new();
    let mut searched_bags: Vec<String> = Vec::new();

    to_search_bags.push("shiny gold".to_string());

    while !to_search_bags.is_empty(){
        // Get next node to search
        let bag_to_search = to_search_bags.pop().unwrap();
        searched_bags.push(bag_to_search.clone());

        // Get all bags that contain the bag_to_search
        for bag in &bags {
            let bag_name = &bag.name;
            if !searched_bags.contains(bag_name) && !to_search_bags.contains(bag_name){
                if bag.bags.iter().any(|x| x.name == bag_to_search){
                    to_search_bags.push(bag_name.to_owned());
                }
            }
        }
    }

    println!("Number of bags that contain shiny gold: {}", searched_bags.len() - 1);
}

fn day2(contents: String) {
    let bags = parse_contents(contents);
    let n_bags_shiny = get_nr_bags(&bags, "shiny gold");
    println!("Number of bags inside shiny gold: {}", n_bags_shiny);
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
