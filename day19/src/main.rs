use std::env;
use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn get_rule(rules_str: &str, rules_cache: &mut HashMap<i32, String>, rule_number_query: i32) -> String{
    // First, check in cache
    if rules_cache.contains_key(&rule_number_query) {
        return rules_cache[&rule_number_query].clone();
    }

    // Parse rules to get it and populate the cache
    for line in rules_str.lines(){
        let split_colon : Vec<String> = line.split(":").map(|x| x.to_owned()).collect();
        let rule_number = split_colon[0].parse::<i32>().unwrap();

        if rule_number == rule_number_query{
            let rule_contents = split_colon[1].trim();

            let mut output = "(".to_string();

            for subrule in rule_contents.split(" "){
                if subrule == "|"{
                    output += "|";
                }
                else if subrule.parse::<i32>().is_ok(){
                    output += &get_rule(rules_str, rules_cache, subrule.parse::<i32>().unwrap());
                }
                else{
                    output += subrule.trim_matches('"');
                }
            }

            output += ")";
            rules_cache.insert(rule_number_query, output.clone());
            return output;
        }
    }

    panic!("Could not find rule {}", rule_number_query);
}

fn day1(contents: String){
    let groups: Vec<String> = contents.split("\n\n").map(|x| x.to_owned()).collect();
    let rules_str = &groups[0];
    let message = &groups[1];

    // Get Rule 0
    let mut rules_cache : HashMap<i32, String> = HashMap::new();
    let rule_0 = get_rule(rules_str, &mut rules_cache, 0);

    // Determine number of valid messages
    let regex_rule_0 = "^".to_owned() + &rule_0 + "$";
    let re_rule_0 = Regex::new(&regex_rule_0).unwrap();

    let mut output = 0;
    for msg in message.lines() {
        if re_rule_0.captures(&msg).is_some() {
            output += 1;
        }
    }

    println!("Part 1 solution: {}", output);
}

fn day2(contents: String){

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
