mod lib;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let option: i32 = args[1].parse().unwrap();
    let contents = "157623984".to_owned();

    if option == 1 {
        println!("Part 1 solution: {}", lib::part1(contents));
    } else if option == 2 {
        println!("Part 2 solution: {}", lib::part2(contents));
    } else {
        panic!("Wrong option!");
    }
}
