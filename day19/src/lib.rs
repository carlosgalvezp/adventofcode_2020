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

fn get_solution(contents: String, is_part2: bool) -> i32{
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

    return output;
}

pub fn part1(contents: String) -> i32{
    return get_solution(contents, false);
}

pub fn part2(contents: String) -> i32{
    return get_solution(contents, true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = vec![
            "0: 4 1 5",
            "1: 2 3 | 3 2",
            "2: 4 4 | 5 5",
            "3: 4 5 | 5 4",
            "4: \"a\"",
            "5: \"b\"",
            "",
            "ababbb",
            "bababa",
            "abbbab",
            "aaabbb",
            "aaaabbb",
        ].join("\n");

        assert_eq!(2, part1(contents));
    }

    #[test]
    fn test_part2() {
        let contents = vec![
            "42: 9 14 | 10 1",
            "9: 14 27 | 1 26",
            "10: 23 14 | 28 1",
            "1: \"a\"",
            "11: 42 31",
            "5: 1 14 | 15 1",
            "19: 14 1 | 14 14",
            "12: 24 14 | 19 1",
            "16: 15 1 | 14 14",
            "31: 14 17 | 1 13",
            "6: 14 14 | 1 14",
            "2: 1 24 | 14 4",
            "0: 8 11",
            "13: 14 3 | 1 12",
            "15: 1 | 14",
            "17: 14 2 | 1 7",
            "23: 25 1 | 22 14",
            "28: 16 1",
            "4: 1 1",
            "20: 14 14 | 1 15",
            "3: 5 14 | 16 1",
            "27: 1 6 | 14 18",
            "14: \"b\"",
            "21: 14 1 | 1 14",
            "25: 1 1 | 1 14",
            "22: 14 14",
            "8: 42",
            "26: 14 22 | 1 20",
            "18: 15 15",
            "7: 14 5 | 1 21",
            "24: 14 1",
            "",
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
            "bbabbbbaabaabba",
            "babbbbaabbbbbabbbbbbaabaaabaaa",
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
            "bbbbbbbaaaabbbbaaabbabaaa",
            "bbbababbbbaaaaaaaabbababaaababaabab",
            "ababaaaaaabaaab",
            "ababaaaaabbbaba",
            "baabbaaaabbaaaababbaababb",
            "abbbbabbbbaaaababbbbbbaaaababb",
            "aaaaabbaabaaaaababaa",
            "aaaabbaaaabbaaa",
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
            "babaaabbbaaabaababbaabababaaab",
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
        ].join("\n");

        assert_eq!(12, part2(contents));
    }
}
