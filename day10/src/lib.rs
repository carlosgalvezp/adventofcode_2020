use std::collections::HashMap;

fn get_joltages(contents: String) -> Vec<i32>{
    let mut joltages = Vec::new();
    for line in contents.lines(){
        joltages.push(line.parse::<i32>().unwrap());
    }
    joltages.sort();
    return joltages;
}

fn get_nr_combinations(current_joltage: i32, joltages: &Vec<i32>, combinations_cache: &mut HashMap<i32, i64>) -> i64{
    // First, check if it exists already in cache
    if combinations_cache.contains_key(&current_joltage){
        return combinations_cache[&current_joltage];
    }

    // If we reached the last adapter, there's no more possible combinations
    if &current_joltage == joltages.last().unwrap(){
        return 1;
    }

    // Otherwise, compute the combinations recursively
    let mut n_combinations = 0;
    for joltage in joltages{
        if (joltage - current_joltage) <= 0{
            continue;
        }
        if (joltage - current_joltage) > 3{
            break;
        }

        n_combinations += get_nr_combinations(*joltage, joltages, combinations_cache);
    }

    // Store in cache
    combinations_cache.insert(current_joltage, n_combinations);

    // Return
    return n_combinations;
}

pub fn part1(contents: String) -> i64{
    let mut joltages = get_joltages(contents);

    joltages.insert(0, 0);                       // Initial joltage
    joltages.push(joltages.last().unwrap() + 3); // Device's built-in adapter

    let mut diff_distribution = HashMap::new();

    for i in 0..joltages.len() - 1{
        let diff = joltages[i+1] - joltages[i];

        if !diff_distribution.contains_key(&diff){
            diff_distribution.insert(diff, 0);
        }
        diff_distribution.insert(diff, diff_distribution[&diff] + 1);
    }
    return diff_distribution[&1] * diff_distribution[&3];
}

pub fn part2(contents: String) -> i64 {
    let joltages = get_joltages(contents);
    let mut combinations_cache = HashMap::new();
    let nr_combinations = get_nr_combinations(0, &joltages, &mut combinations_cache);
    return nr_combinations;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_contents() -> String{
        return vec![
            "28",
            "33",
            "18",
            "42",
            "31",
            "14",
            "46",
            "20",
            "48",
            "47",
            "24",
            "23",
            "49",
            "45",
            "19",
            "38",
            "39",
            "11",
            "1",
            "32",
            "25",
            "35",
            "8",
            "17",
            "7",
            "9",
            "4",
            "2",
            "34",
            "10",
            "3",
        ].join("\n");
    }

    #[test]
    fn test_part1() {
        assert_eq!(220, part1(get_test_contents()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(19208, part2(get_test_contents()));
    }
}
