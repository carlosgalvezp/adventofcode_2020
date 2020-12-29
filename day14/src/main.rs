use std::env;
use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn day1(contents: String){
    let mut memory = HashMap::new();
    let mut mask_clear: u64 = 0;
    let mut mask_set: u64 = 0;

    let re_mask = Regex::new(r"^mask = ([0X1]{36})$").unwrap();
    let re_memory = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    for line in contents.lines() {
        let re_mask_result = re_mask.captures(&line);
        if re_mask_result.is_some(){
            let mask_str = re_mask_result.unwrap()[1].to_owned();
            let mask_clear_str = mask_str.replace("1", "0").replace("X", "1");
            let mask_set_str = mask_str.replace("X", "0");

            mask_clear = u64::from_str_radix(&mask_clear_str, 2).unwrap();
            mask_set = u64::from_str_radix(&mask_set_str, 2).unwrap();
        }

        let re_memory_result = re_memory.captures(&line);
        if re_memory_result.is_some(){
            let re_memory_result_unwrapped = re_memory_result.unwrap();
            let address = re_memory_result_unwrapped[1].parse::<u64>().unwrap();
            let mut content = re_memory_result_unwrapped[2].parse::<u64>().unwrap();

            content &= mask_clear;
            content |= mask_set;

            memory.insert(address, content);
        }
    }

    let mut result = 0;
    for c in memory.values() {
        result += c;
    }

    println!("Part 1 solution: {}", result);
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
