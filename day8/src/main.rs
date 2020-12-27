use std::env;
use std::fs;

#[derive(Clone, PartialEq)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(Clone)]
struct CodeLine {
    operation: Operation,
    argument: i32,
}

type Code = Vec<CodeLine>;

struct IntCode{
    code: Code,
    pc: i32,
    acc: i32,
    processed_lines: Vec<i32>,
}

impl IntCode{
    fn new(code: Code) -> IntCode{
        return IntCode{code, pc: 0, acc: 0, processed_lines: Vec::new()};
    }

    fn run(&mut self) -> bool{
        while self.pc < (self.code.len() as i32) {
            if self.processed_lines.contains(&self.pc){
                return false;
            }

            self.processed_lines.push(self.pc);

            let line = &self.code[self.pc as usize];
            match line.operation {
                Operation::Acc => {
                    self.acc += line.argument;
                    self.pc += 1;
                },
                Operation::Jmp => {
                    self.pc += line.argument
                },
                Operation::Nop => {
                    self.pc += 1;
                },
            };
        }

        return true;
    }
}

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
    let mut intcode = IntCode::new(code);

    intcode.run();

    println!("Value of accumulator before loop: {}", intcode.acc);
}

fn day2(contents: String) {
    let original_code = get_code(contents);

    for i in 0..original_code.len(){
        let mut code = original_code.clone();

        if code[i].operation == Operation::Nop{
            code[i].operation = Operation::Jmp;
        }
        else if code[i].operation == Operation::Jmp{
            code[i].operation = Operation::Nop;
        }
        else{
            continue;
        }

        let mut intcode = IntCode::new(code);
        let result = intcode.run();
        if result{
            println!("After fixing bug, acc contains {}", intcode.acc);
            return;
        }
    }
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
