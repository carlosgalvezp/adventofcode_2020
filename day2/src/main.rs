use std::env;
use std::fs;

struct LineInfo
{
    min: i32,
    max: i32,
    letter: char,
    password: String,
}

fn get_line_info(line: &str) -> LineInfo{
    let space_tokens: Vec<&str> = line.split(' ').collect();
    let size_part : &str = space_tokens[0];
    let letter_part : &str = space_tokens[1];
    let password : &str = space_tokens[2];

    let min_max: Vec<&str> = size_part.split('-').collect();
    let min = min_max[0].parse::<i32>().unwrap();
    let max = min_max[1].parse::<i32>().unwrap();

    let letter : char  = letter_part.chars().next().unwrap();

    return LineInfo { min, max, letter, password: password.to_owned()};
}

fn day1(contents: String)
{
    let lines = contents.lines();
    let mut valid_passwords = 0;

    for line in lines{
        let line_info = get_line_info(line);

        let mut count = 0;

        for c in line_info.password.chars(){
            if c == line_info.letter{
                count += 1;
            }
        }

        if (count >= line_info.min) && (count <= line_info.max){
            valid_passwords += 1;
        }
    }

    println!("Number of valid passwords: {}", valid_passwords);
}

fn day2(contents: String)
{
    let lines = contents.lines();
    let mut valid_passwords = 0;

    for line in lines{
        let line_info = get_line_info(line);

        let mut count = 0;

        let idx_min = (line_info.min - 1) as usize;
        let idx_max = (line_info.max - 1) as usize;

        if line_info.password.chars().nth(idx_min).unwrap() == line_info.letter{
            count += 1;
        }

        if line_info.password.chars().nth(idx_max).unwrap() == line_info.letter{
            count += 1;
        }

        if count == 1{
            valid_passwords += 1;
        }
    }

    println!("Number of valid passwords: {}", valid_passwords);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let option : i32 = args[1].parse().unwrap();
    let fname = &args[2];
    let contents = fs::read_to_string(fname).expect("Something went wrong reading the file");

    if option == 1{
        day1(contents);
    }
    else if option == 2{
        day2(contents);
    }
    else{
        panic!("Wrong option!");
    }
}
