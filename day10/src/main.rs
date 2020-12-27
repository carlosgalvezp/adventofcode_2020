use std::{env, hash::Hash};
use std::fs;
use std::collections::HashMap;

fn day1(contents: String) {
    let mut adapters = Vec::new();

    for line in contents.lines(){
        adapters.push(line.parse::<i32>().unwrap());
    }

    adapters.push(0);                            // Outlet
    adapters.sort();
    adapters.push(adapters.last().unwrap() + 3); // Device's built-in adapter

    let mut diff_distribution = HashMap::new();

    for i in 0..adapters.len() - 1{
        let diff = adapters[i+1] - adapters[i];

        if !diff_distribution.contains_key(&diff){
            diff_distribution.insert(diff, 0);
        }
        diff_distribution.insert(diff, diff_distribution[&diff] + 1);
    }

    println!("Part 1 solution: {}", diff_distribution[&1] * diff_distribution[&3]);
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
