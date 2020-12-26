use std::env;
use std::fs;

enum Operation {
    Acc,
    Jmp,
    Nop,
}

struct CodeLine {
    operation: Operation,
    argument: i32,
}

type Code = Vec<CodeLine>;

fn get_code(contents: String) -> Code {
    let mut code = Code::new();

    for line in contents.lines() {
        let line_tokens: Vec<&str> = line.split(" ").collect();
        let argument = line_tokens[1].parse::<i32>().unwrap();
        let operation = match line_tokens[0] {
            "acc" => Operation::Acc,
            "jmp" => Operation::Jmp,
            "nop" => Operation::Nop,
            _ => panic!("Bad operation {}", line_tokens[0]),
        };

        code.push(CodeLine{operation, argument});
    }

    return code;
}

fn day1(contents: String) {
    let code = get_code(contents);

    let mut processed_lines = Vec::new();
    let mut acc = 0;
    let mut pc = 0;

    loop {
        if processed_lines.contains(&pc){
            break;
        }
        processed_lines.push(pc);
        let line = &code[pc as usize];
        match line.operation {
            Operation::Acc => {
                acc += line.argument;
                pc += 1;
            },
            Operation::Jmp => {
                pc += line.argument
            },
            Operation::Nop => {
                pc += 1;
            },
        };
    }

    println!("Value of accumulator before loop: {}", acc);
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
