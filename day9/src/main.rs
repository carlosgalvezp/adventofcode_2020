use std::env;
use std::fs;

fn day1(contents: String) {
    let mut numbers = Vec::new();

    for line in contents.lines(){
        numbers.push(line.parse::<i64>().unwrap());
    }

    for i in 25..numbers.len(){
        let mut found = false;
        for j in i-25..i{
            for k in j+1..i{
                if numbers[j] + numbers[k] == numbers[i]{
                    found = true;
                    break;
                }
            }
            if found{
                break;
            }
        }

        if !found{
            println!("The first number that doesn't follow the rule is {}", numbers[i]);
            break;
        }
    }
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
