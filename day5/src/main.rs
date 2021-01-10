mod lib;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let option: i32 = args[1].parse().unwrap();
    let fname = &args[2];
    let contents = fs::read_to_string(fname).expect("Something went wrong reading the file");

    if option == 1 {
        println!("Part 1 solution: {}", lib::part1(contents));
    } else if option == 2 {
        println!("Part 2 solution: {}", lib::part2(contents));
    } else {
        panic!("Wrong option!");
    }
}
