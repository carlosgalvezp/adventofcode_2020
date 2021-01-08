use std::env;

fn get_cup_idx(cups: &Vec<i32>, cup: i32) -> usize {
    return cups.iter().position(|&x| x == cup).unwrap();
}

#[derive(PartialEq)]
struct State{
    cups: Vec<i32>,
    current_cup: i32,
}

fn run(original_cups: &Vec<i32>, n_moves: i32) -> Vec<i32>{
    let mut cups = original_cups.clone();
    let mut current_cup = cups[0];

    for i in 0..n_moves{
        if (i % 10000) == 0{
            println!("Iteration {}/{} ({}%)", i, n_moves, (i as f32)/(n_moves as f32) * 100.0);
        }
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

        assert!(destination_cup > 0);

        // Move cups
        for i in 0..cups_to_move.len(){
            let destination_cup_idx = get_cup_idx(&cups, destination_cup);
            cups.insert((destination_cup_idx + i + 1) % cups.len(), cups_to_move[i]);
        }

        // Select new current cup
        let next_current_cup_idx = (get_cup_idx(&cups, current_cup) + 1) % cups.len();
        current_cup = cups[next_current_cup_idx];
    }

    return cups;
}

fn day1(contents: String){
    let cups: Vec<i32> = contents.chars().map(|x| x.to_digit(10).unwrap() as i32).collect();

    // Run for 100 moves
    let cups = run(&cups, 100);

    // Find cup with label 1
    let cup_1_idx = cups.iter().position(|&x| x == 1).unwrap();
    for i in 0..cups.len() - 1{
        print!("{}", cups[(cup_1_idx + i + 1) % cups.len()]);
    }
    println!();
}

fn day2(contents: String){
    let mut cups: Vec<i32> = contents.chars().map(|x| x.to_digit(10).unwrap() as i32).collect();

    for i in 10..=1000000{
        cups.push(i);
    }

    // Run for 10 000 000 moves
    let cups = run(&cups, 10000000);

    // Get output
    let cup_1_idx = cups.iter().position(|&x| x == 1).unwrap();

    let c1 = cups[(cup_1_idx + 1) % cups.len()] as i64;
    let c2 = cups[(cup_1_idx + 2) % cups.len()] as i64;
    println!("Part 2 solution: {}", c1 * c2);
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
