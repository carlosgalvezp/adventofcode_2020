use std::env;
use std::fs;

fn get_numbers(contents: String) -> Vec<i64> {
    let mut numbers = Vec::new();

    for line in contents.lines(){
        numbers.push(line.parse::<i64>().unwrap());
    }

    return numbers;
}

fn get_invalid_number(numbers: &Vec<i64>) -> i64{
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
            return numbers[i];
        }
    }
    panic!("Could not find number");
}

fn day1(contents: String) {
    let numbers = get_numbers(contents);
    let invalid_number = get_invalid_number(&numbers);
    println!("Invalid number: {}", invalid_number);
}

fn day2(contents: String) {
    let numbers = get_numbers(contents);
    let invalid_number = get_invalid_number(&numbers);

    for i in 0..numbers.len() - 1{
        for j in i+1..numbers.len(){
            let mut sum = 0;

            let mut min_number = i64::MAX;
            let mut max_number = 0;
            for k in i..j+1{
                sum += numbers[k];

                min_number = std::cmp::min(min_number, numbers[k]);
                max_number = std::cmp::max(max_number, numbers[k]);
            }

            if sum == invalid_number{
                println!("Day 2 solution: {}", min_number + max_number);
                return;
            }
            if sum > invalid_number{
                break;
            }
        }
    }
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
