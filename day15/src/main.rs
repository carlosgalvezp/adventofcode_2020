use std::env;
use std::collections::HashMap;

fn day1(input: Vec<i32>){
    let mut spoken_numbers = HashMap::new();
    let mut last_number = 0;

    for turn in 0..2020{
        if (turn as usize) < input.len(){
            last_number = input[turn as usize];
            spoken_numbers.insert(last_number, vec![turn]);
        }
        else{
            let turns_spoked = &spoken_numbers[&last_number];

            // If it had been spoked only once
            if turns_spoked.len() == 1{
                last_number = 0;
            }
            else{
                last_number = turns_spoked.last().unwrap() - turns_spoked.first().unwrap();
            }

            // Update spoken list
            let mut new_turns_spoked = vec![turn];
            if spoken_numbers.contains_key(&last_number){
                new_turns_spoked.insert(0, *spoken_numbers[&last_number].last().unwrap());
            }
            spoken_numbers.insert(last_number, new_turns_spoked);
        }
    }

    println!("Part 1 solution: {}", last_number);
}

fn day2(input: Vec<i32>){

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let option: i32 = args[1].parse().unwrap();
    let input = vec![18,8,0,5,4,1,20];

    if option == 1 {
        day1(input);
    } else if option == 2 {
        day2(input);
    } else {
        panic!("Wrong option!");
    }
}
