use std::env;
use std::fs;
use regex::Regex;

struct ParenthesisBlock {
    start : usize,
    end: usize,
    depth: i32,
}

fn process_simple_line_part1(line: &str) -> i64 {
    let mut result = 0;
    let tokens: Vec<&str> = line.split(" ").into_iter().collect();

    result += tokens[0].parse::<i64>().unwrap();

    for i in (1..tokens.len()).step_by(2){
        let operation = tokens[i];
        let rhs = tokens[i + 1].parse::<i64>().unwrap();

        match operation{
            "+" => result += rhs,
            "*" => result *= rhs,
            _ => panic!("Wrong operation {}", operation),
        };
    }

    return result;
}

fn process_simple_line_part2(line: &str) -> i64 {
    let re_binary_sum = Regex::new(r"^(\d+) \+ (\d+)$").unwrap();
    let re_result = re_binary_sum.captures(&line);

    if re_result.is_some(){
        // If the line is just a binary sum, return the result
        let re_result_unwrapped = re_result.unwrap();
        let a = re_result_unwrapped[1].parse::<i64>().unwrap();
        let b = re_result_unwrapped[2].parse::<i64>().unwrap();
        return a + b;
    }
    else if line.contains("+"){
        // If there are sums remaining to isolate, add parenthesis around them and process them
        let mut tokens : Vec<String> = line.split(" ").into_iter().map(|x| x.to_owned()).collect();
        for i in 1..tokens.len() - 1{
            if tokens[i] == "+"{
                tokens[i - 1] = "(".to_string() + &tokens[i - 1];
                tokens[i + 1] = tokens[i + 1].clone() + ")";
                break;
            }
        }
        let line_with_parenthesis = tokens.join(" ");
        return process_complex_line(&line_with_parenthesis, &process_simple_line_part2);
    }
    else {
        let mut result = 1;

        let tokens: Vec<&str> = line.split(" ").into_iter().collect();

        for token in tokens{
            if token != "*"{
                let n = token.parse::<i64>().unwrap();
                result *= n;
            }
        }

        return result;
    }
}

fn process_complex_line(line: &str, process_simple_line_fn: &dyn Fn(&str) -> i64) -> i64 {
    let preprocessed_line = preprocess_line(line, process_simple_line_fn);
    return process_simple_line_fn(&preprocessed_line);
}

fn get_parenthesis_blocks(line: &str) -> Vec<ParenthesisBlock>{
    // Find where all parenthesis are
    let mut start_stack = Vec::new();
    let mut parenthesis = Vec::new();

    let mut depth = 0;

    for (i, c) in line.chars().enumerate(){
        match c {
            '(' => {
                start_stack.push(i);
                depth += 1;
            },
            ')' => {
                let start = start_stack.pop().unwrap();
                let end = i;
                parenthesis.push(ParenthesisBlock{start, end, depth});
                depth -= 1;
            },
            _ => {},
        };
    }

    // Sort them by descending depth
    parenthesis.sort_by(|a, b| b.depth.cmp(&a.depth));

    return parenthesis;
}

fn preprocess_line(line: &str, process_simple_line_fn: &dyn Fn(&str) -> i64) -> String {
    let mut output = line.to_owned().clone();

    while output.contains("(") || output.contains(")"){
        // Get parenthesis blocks
        let parenthesis = get_parenthesis_blocks(&output);

        // Take the deepest parenthesis and replace with its result
        let deepest_parenthesis = parenthesis.first().unwrap();
        let parenthesis_substr = &output[deepest_parenthesis.start+1..deepest_parenthesis.end];
        let parenthesis_result = process_simple_line_fn(parenthesis_substr);
        output.replace_range(deepest_parenthesis.start..deepest_parenthesis.end+1, &parenthesis_result.to_string());
    }

    return output;
}

fn day1(contents: String) {
    let mut result = 0;
    for line in contents.lines() {
        result += process_complex_line(line, &process_simple_line_part1);
    }
    println!("Part 1 solution: {}", result);
}

fn day2(contents: String) {
    let mut result = 0;
    for line in contents.lines() {
        result += process_complex_line(line, &process_simple_line_part2);
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
