use std::env;
use std::fs;

struct ParenthesisBlock {
    start : usize,
    end: usize,
    depth: i32,
}

fn process_simple_line(line: &str) -> i64 {
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

fn process_complex_line(line: &str) -> i64 {
    let preprocessed_line = preprocess_line(line);
    return process_simple_line(&preprocessed_line);
}

fn preprocess_line(line: &str) -> String {
    let mut output = line.to_owned().clone();

    while output.contains("(") || output.contains(")"){
        // Find where all parenthesis are
        let mut start_stack = Vec::new();
        let mut parenthesis = Vec::new();

        let mut depth = 0;

        for (i, c) in output.chars().enumerate(){
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

        // Take the deepest parenthesis and replace with its result
        let deepest_parenthesis = parenthesis.first().unwrap();
        let parenthesis_substr = &output[deepest_parenthesis.start+1..deepest_parenthesis.end];
        let parenthesis_result = process_simple_line(parenthesis_substr);
        output.replace_range(deepest_parenthesis.start..deepest_parenthesis.end+1, &parenthesis_result.to_string());
    }

    return output;
}

fn day1(contents: String) {
    let mut result = 0;
    for line in contents.lines() {
        result += process_complex_line(line);
    }
    println!("Part 1 solution: {}", result);
}

fn day2(contents: String) {

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
