use std::env;
use std::fs;

fn day1(contents: String){
    let group_answers = contents.split("\n\n").map(|x| x.to_owned() + "\n");

    let mut n_yes_answers = 0;

    for mut group_answer in group_answers{
        group_answer = group_answer.replace("\n", "");
        let mut group_unique: Vec<char> = group_answer.chars().collect();
        group_unique.sort();
        group_unique.dedup();
        n_yes_answers += group_unique.len();
    }

    println!("Number of yes answers: {}", n_yes_answers);
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
