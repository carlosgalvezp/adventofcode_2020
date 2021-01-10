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

pub fn part1(contents: String) -> i32{
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

    return (searched_bags.len() - 1) as i32;
}

pub fn part2(contents: String) -> i32{
    let bags = parse_contents(contents);
    let n_bags_shiny = get_nr_bags(&bags, "shiny gold");
    return n_bags_shiny;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_contents() -> String{
        return vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ].join("\n");
    }

    fn get_extra_test_contents() -> String{
        return vec![
            "shiny gold bags contain 2 dark red bags.",
            "dark red bags contain 2 dark orange bags.",
            "dark orange bags contain 2 dark yellow bags.",
            "dark yellow bags contain 2 dark green bags.",
            "dark green bags contain 2 dark blue bags.",
            "dark blue bags contain 2 dark violet bags.",
            "dark violet bags contain no other bags.",
        ].join("\n");
    }

    #[test]
    fn test_part1() {
        assert_eq!(4, part1(get_test_contents()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(32, part2(get_test_contents()));
        assert_eq!(126, part2(get_extra_test_contents()));
    }
}
