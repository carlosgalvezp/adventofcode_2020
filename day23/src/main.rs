use std::{env, thread::current};

fn get_cup_idx(cups: &Vec<i32>, cup: i32) -> usize {
    return cups.iter().position(|&x| x == cup).unwrap();
}

fn day1(contents: String){
    let mut cups: Vec<i32> = contents.chars().map(|x| x.to_digit(10).unwrap() as i32).collect();
    let mut current_cup = cups[0];
    let n_moves = 100;

    for _ in 0..n_moves{
        // Take 3 cups
        let mut cups_to_move = Vec::new();
        for _ in 0..3{
            let idx = (get_cup_idx(&cups, current_cup) + 1) % cups.len();
            let cup = cups[idx];
            cups_to_move.push(cup);
            cups.remove(idx);
        }

        // Get destination cup
        let mut destination_cup = current_cup - 1;
        let min_label_remaining = *cups.iter().min().unwrap();
        let max_label_remaining = *cups.iter().max().unwrap();

        while !cups.contains(&destination_cup){
            destination_cup -= 1;
            if destination_cup < min_label_remaining{
                destination_cup = max_label_remaining;
            }
        }

        let destination_cup_idx = get_cup_idx(&cups, destination_cup);

        // Move cups
        for i in 0..cups_to_move.len(){
            cups.insert(destination_cup_idx + i + 1, cups_to_move[i]);
        }

        // Select new current cup
        let next_current_cup_idx = (get_cup_idx(&cups, current_cup) + 1) % cups.len();
        current_cup = cups[next_current_cup_idx];
    }

    // Find cup with label 1
    let cup_1_idx = cups.iter().position(|&x| x == 1).unwrap();
    for i in 0..cups.len() - 1{
        print!("{}", cups[(cup_1_idx + i + 1) % cups.len()]);
    }
    println!();
}

fn day2(contents: String){
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let option: i32 = args[1].parse().unwrap();
    let contents = "157623984".to_owned();

    if option == 1 {
        day1(contents);
    } else if option == 2 {
        day2(contents);
    } else {
        panic!("Wrong option!");
    }
}
