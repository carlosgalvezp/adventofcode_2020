use std::env;
use std::fs;

#[derive(Clone, PartialEq)]
struct GameState{
    cards_1 : Vec<i32>,
    cards_2 : Vec<i32>,
}

fn get_player_cards(contents: &String) -> Vec<i32>{
    return contents.lines().skip(1)
                   .map(|x| x.parse::<i32>().unwrap())
                   .collect();
}

fn get_cards(contents: String) -> (Vec<i32>, Vec<i32>){
    let player_data : Vec<String> = contents.split("\n\n").map(|x| x.to_owned()).collect();
    let cards_1 = get_player_cards(&player_data[0]);
    let cards_2 = get_player_cards(&player_data[1]);
    return (cards_1, cards_2);
}

fn play_game(cards_1: &Vec<i32>, cards_2: &Vec<i32>) -> (i32, Option<Vec<i32>>){
    // Normal game
    let mut cards_1_copy = cards_1.clone();
    let mut cards_2_copy = cards_2.clone();
    let mut played_states = Vec::new();

    while !cards_1_copy.is_empty() && !cards_2_copy.is_empty(){
        // First, avoid recursion
        let game_state = GameState{cards_1: cards_1_copy.clone(), cards_2: cards_2_copy.clone()};
        if played_states.contains(&game_state){
            return (1, Some(cards_1_copy));
        } else {
            played_states.push(game_state);
        }

        // Draw a card
        let c1 = cards_1_copy[0];
        let c2 = cards_2_copy[0];
        cards_1_copy.remove(0);
        cards_2_copy.remove(0);

        let (winner, _) =
            if (cards_1_copy.len() as i32 >= c1) && (cards_2_copy.len() as i32 >= c2){
                let next_cards_1 = cards_1_copy[0..c1 as usize].to_vec();
                let next_cards_2 = cards_2_copy[0..c2 as usize].to_vec();
                play_game(&next_cards_1, &next_cards_2)
            }
            else{
                if c1 > c2{(1, None)}
                else{(2, None)}
            };

        if winner == 1{
            cards_1_copy.push(c1);
            cards_1_copy.push(c2);
        }
        else{
            cards_2_copy.push(c2);
            cards_2_copy.push(c1);
        }
    }

    if cards_1_copy.is_empty(){
        return (2, Some(cards_2_copy));
    }
    else{
        return (1, Some(cards_1_copy));
    }
}

fn day1(contents: String){
    let (mut cards_1, mut cards_2) = get_cards(contents);

    while !cards_1.is_empty() && !cards_2.is_empty(){
        let c1 = cards_1[0];
        let c2 = cards_2[0];

        if c1 > c2{
            cards_1.push(c1);
            cards_1.push(c2);
        }
        else{
            cards_2.push(c2);
            cards_2.push(c1);
        }

        cards_1.remove(0);
        cards_2.remove(0);
    }

    let winner = if cards_1.is_empty() {
        &cards_2
    } else {
        &cards_1
    };

    let mut result = 0;
    for i in 0..winner.len(){
        result += winner[i] * ((winner.len() - i) as i32);
    }

    println!("Part 1 solution: {}", result);
}

fn day2(contents: String){
    let (cards_1, cards_2) = get_cards(contents);
    let (_, winner_cards_option) = play_game(&cards_1, &cards_2);

    let mut result = 0;
    let winner_cards = winner_cards_option.unwrap();
    for i in 0..winner_cards.len(){
        result += winner_cards[i] * ((winner_cards.len() - i) as i32);
    }
    println!("Part 2 solution: {}", result);
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
