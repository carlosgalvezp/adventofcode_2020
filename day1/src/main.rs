use std::io::{self, BufRead};

fn main() {
    // Read input
    let mut numbers: Vec<i32> = Vec::new();

    for line in io::stdin().lock().lines() {
        let num: i32 = line
            .expect("Failed to read line")
            .parse()
            .expect("Failed to parse line into number");
        numbers.push(num);
    }

    // Compute output
    let mut found = false;

    let mut i = 0;
    while i < numbers.len() - 1 {
        let mut j = i + 1;
        while j < numbers.len() {
            if numbers[i] + numbers[j] == 2020{
                println!("Found numbers: {} and {}. Product: {}", numbers[i], numbers[j], numbers[i] * numbers[j]);
                found = true;
                break;
            }
            j = j + 1;
        }

        i = i + 1;
        if found{
            break;
        }
    }
}
