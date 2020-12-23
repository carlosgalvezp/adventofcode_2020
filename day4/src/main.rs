use std::env;
use std::fs;

fn day1(contents: String){
    let passport_lines = contents.split("\n\n");

    let mut n_valid_passports = 0;

    for passport in passport_lines{
        if passport.contains("byr") &&
           passport.contains("iyr") &&
           passport.contains("eyr") &&
           passport.contains("hgt") &&
           passport.contains("hcl") &&
           passport.contains("ecl") &&
           passport.contains("pid") {
            n_valid_passports += 1;
        }
    }

    println!("Numer of valid passports: {}", n_valid_passports);
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
