use std::env;
use std::fs;

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
