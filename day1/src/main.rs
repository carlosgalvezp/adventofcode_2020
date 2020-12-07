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
    while i < numbers.len() - 2 {
        let mut j = i + 1;
        while j < numbers.len() - 1 {
            let mut k = j + 1;
            while k < numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == 2020{
                    println!("Found numbers: {}, {} and {}. Product: {}",
                             numbers[i], numbers[j], numbers[k],
                             numbers[i] * numbers[j] * numbers[k]);
                    found = true;
                    break;
                }
                k = k + 1;
            }
            j = j + 1;
            if found{
                break;
            }
        }

        i = i + 1;
        if found{
            break;
        }
    }
}
