use std::env;
use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn get_rule(rules_str: &str, rules_cache: &mut HashMap<i32, String>, rule_number_query: i32, is_part2: bool) -> String{
    // First, check in cache
    if rules_cache.contains_key(&rule_number_query) {
        return rules_cache[&rule_number_query].clone();
    }

    // Parse rules to get it and populate the cache
    for line in rules_str.lines(){
        let split_colon : Vec<String> = line.split(":").map(|x| x.to_owned()).collect();
        let rule_number = split_colon[0].parse::<i32>().unwrap();

        if rule_number == rule_number_query{
            let mut output = "".to_string();

            if is_part2 && ((rule_number == 8) || (rule_number == 11)){
                if rule_number == 8 {
                    let rule_42 = &get_rule(rules_str, rules_cache, 42, false);
                    output = format!("({})+", rule_42);
                }
                else if rule_number == 11 {
                    let rule_42 = &get_rule(rules_str, rules_cache, 42, is_part2);
                    let rule_31 = &get_rule(rules_str, rules_cache, 31, is_part2);
                    let mut rule_11_variants = Vec::new();
                    for i in 1..5{
                        rule_11_variants.push(format!("(({rule_42}){{{n}}}({rule_42}{rule_31})?({rule_31}){{{n}}})", rule_42=rule_42, rule_31=rule_31, n=i));
                    }
                    output = rule_11_variants.join("|");
                }
            }
            else{
                for subrule in split_colon[1].trim().split(" "){
                    if subrule == "|"{
                        output += "|";
                    }
                    else if subrule.parse::<i32>().is_ok(){
                        output += &get_rule(rules_str, rules_cache, subrule.parse::<i32>().unwrap(), is_part2);
                    }
                    else{
                        output += subrule.trim_matches('"');
                    }
                }
            }

            if output.len() > 1{
                output = format!("({})", output);
            }

            rules_cache.insert(rule_number_query, output.clone());
            return output;
        }
    }

    panic!("Could not find rule {}", rule_number_query);
}

fn get_solution(contents: String, is_part2: bool){
    let groups: Vec<String> = contents.split("\n\n").map(|x| x.to_owned()).collect();
    let rules_str = &groups[0];
    let message = &groups[1];

    // Get Rule 0
    let mut rules_cache : HashMap<i32, String> = HashMap::new();
    let rule_0 = get_rule(rules_str, &mut rules_cache, 0, is_part2);

    // Determine number of valid messages
    let regex_rule_0 = "^".to_owned() + &rule_0 + "$";
    let re_rule_0 = Regex::new(&regex_rule_0).unwrap();

    let mut output = 0;
    for msg in message.lines() {
        if re_rule_0.captures(&msg).is_some() {
            output += 1;
        }
    }

    println!("Solution: {}", output);
}

fn day1(contents: String){
    get_solution(contents, false);
}

fn day2(contents: String){
    get_solution(contents, true);
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
