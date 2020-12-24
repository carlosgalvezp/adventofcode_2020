use std::env;
use std::fs;

fn get_id(line: &str) -> i32{
    let line_binary = line.replace("B", "1")
    .replace("F", "0")
    .replace("R", "1")
    .replace("L", "0");

    let row = &line_binary[..7];
    let col = &line_binary[7..];

    let row_i32 = i32::from_str_radix(row, 2).unwrap();
    let col_i32 = i32::from_str_radix(col, 2).unwrap();
    let id = row_i32 * 8 + col_i32;

    return id;
}

fn day1(contents: String){
    let mut id_max = 0;

    for line in contents.lines(){
        let id = get_id(line);
        id_max = std::cmp::max(id, id_max);
    }
    println!("Max ID: {}", id_max);
}

fn day2(contents: String){
    let mut ids = Vec::new();

    for line in contents.lines(){
        let id = get_id(line);
        ids.push(id);
    }

    ids.sort();

    for (i, _) in ids.iter().enumerate() {
        if i < ids.len() - 1{
            if (ids[i+1] - ids[i]) == 2{
                println!("My ID: {}", ids[i] + 1);
                break;
            }
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
