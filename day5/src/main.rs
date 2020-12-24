use std::env;
use std::fs;

fn day1(contents: String){
    let mut id_max = 0;

    for line in contents.lines(){
        let line_binary = line.replace("B", "1")
                              .replace("F", "0")
                              .replace("R", "1")
                              .replace("L", "0");

        let row = &line_binary[..7];
        let col = &line_binary[7..];

        let row_i32 = i32::from_str_radix(row, 2).unwrap();
        let col_i32 = i32::from_str_radix(col, 2).unwrap();
        let id = row_i32 * 8 + col_i32;

        id_max = std::cmp::max(id, id_max);
    }
    println!("Max ID: {}", id_max);
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
