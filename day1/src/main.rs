use std::env;
use std::fs;

fn day1(contents: String){
    let numbers : Vec<i32> = contents.lines().map(|x| x.parse::<i32>().unwrap()).collect();

    for i in 0..numbers.len() - 1 {
        for j in i + 1..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                println!(
                    "Found numbers: {} and {}. Product: {}",
                    numbers[i],
                    numbers[j],
                    numbers[i] * numbers[j]
                );
                return
            }
        }
    }
}

fn day2(contents: String){
    let numbers : Vec<i32> = contents.lines().map(|x| x.parse::<i32>().unwrap()).collect();

    for i in 0..numbers.len() - 2 {
        for j in i + 1..numbers.len() - 1 {
            for k in j + 1..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    println!(
                        "Found numbers: {}, {} and {}. Product: {}",
                        numbers[i],
                        numbers[j],
                        numbers[k],
                        numbers[i] * numbers[j] * numbers[k]
                    );
                    return
                }
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
