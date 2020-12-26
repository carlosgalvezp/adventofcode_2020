use std::env;
use std::fs;
use regex::Regex;

struct BagInfo {
    name: String,
    bags: Vec<String>,
}

fn day1(contents: String) {
    let re_full = Regex::new(r"^([\w ]+) bags contain (.*)\.$").unwrap();
    let re_contains = Regex::new(r"\d+ ([\w ]+) bag[s]?").unwrap();

    // Parse input into vector of BagInfo
    let mut bags = Vec::new();

    for line in contents.lines() {
        let full_sentence_parse = re_full.captures(&line);
        if full_sentence_parse.is_some(){
            let full_sentence_parse_data = full_sentence_parse.unwrap();
            let bag_name = full_sentence_parse_data[1].to_owned();
            let bag_contents = full_sentence_parse_data[2].to_owned();

            let mut contained_bags : Vec<String> = Vec::new();

            for bag_str in bag_contents.split(","){
                let bag_name = re_contains.captures(&bag_str);
                if bag_name.is_some(){
                    contained_bags.push(bag_name.unwrap()[1].to_owned());
                }
            }

            bags.push(BagInfo{name: bag_name, bags: contained_bags});
        }
    }

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
                if bag.bags.contains(&bag_to_search){
                    to_search_bags.push(bag_name.to_owned());
                }
            }
        }
    }

    println!("Number of bags that contain shiny gold: {}", searched_bags.len() - 1);
}

fn day2(contents: String) {

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
