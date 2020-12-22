use std::env;
use std::fs;

fn day1(contents: String)
{
    let lines = contents.lines();
    let mut valid_passwords = 0;

    for line in lines{
        let space_tokens: Vec<&str> = line.split(' ').collect();
        let size_part : &str = space_tokens[0];
        let letter_part : &str = space_tokens[1];
        let password : &str = space_tokens[2];

        let min_max_letters_tokens: Vec<&str> = size_part.split('-').collect();
        let min_letters = min_max_letters_tokens[0].parse::<i32>().unwrap();
        let max_letters = min_max_letters_tokens[1].parse::<i32>().unwrap();

        let letter : char  = letter_part.chars().next().unwrap();

        let mut count = 0;

        for c in password.chars(){
            if c == letter{
                count += 1;
            }
        }

        if count >= min_letters && count <= max_letters{
            valid_passwords += 1;
        }
    }

    println!("Number of valid passwords: {}", valid_passwords);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let contents = fs::read_to_string(input).expect("Something went wrong reading the file");

    day1(contents);
}
