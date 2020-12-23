use std::env;
use std::fs;
use regex::Regex;

fn day1(contents: String){
    let passport_lines = contents.split("\n\n").map(|x| x.to_owned() + "\n");

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
    let passport_lines = contents.split("\n\n").map(|x| x.to_owned() + "\n");

    let re_byr = Regex::new(r"byr:(\d{4})[ |\n]").unwrap();
    let re_iyr = Regex::new(r"iyr:(\d{4})[ |\n]").unwrap();
    let re_eyr = Regex::new(r"eyr:(\d{4})[ |\n]").unwrap();
    let re_hgt = Regex::new(r"hgt:(\d+)(cm|in)[ |\n]").unwrap();
    let re_hcl = Regex::new(r"hcl:#([0-9a-f]{6})[ |\n]").unwrap();
    let re_ecl = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)[ |\n]").unwrap();
    let re_pid = Regex::new(r"pid:(\d{9})[ |\n]").unwrap();

    let mut n_valid_passports = 0;

    for passport in passport_lines{
        // Validate birth year
        let byr = re_byr.captures(&passport);
        if byr.is_some(){
            let birth_year: i32 = byr.unwrap()[1].parse().unwrap();
            if birth_year < 1920 || birth_year > 2002{
                continue;
            }
        }
        else{
            continue;
        }

        // Validate issue year
        let iyr = re_iyr.captures(&passport);
        if iyr.is_some(){
            let issue_year: i32 = iyr.unwrap()[1].parse().unwrap();
            if issue_year < 2010 || issue_year > 2020{
                continue;
            }
        }
        else{
            continue;
        }

        // Validate expiration year
        let eyr = re_eyr.captures(&passport);
        if eyr.is_some(){
            let expiration_year: i32 = eyr.unwrap()[1].parse().unwrap();
            if expiration_year < 2020 || expiration_year > 2030{
                continue;
            }
        }
        else{
            continue;
        }

        // Validate height
        let hgt = re_hgt.captures(&passport);
        if hgt.is_some(){
            let hgt_result = hgt.unwrap();
            let hgt_0 = &hgt_result[1];
            let hgt_1 = &hgt_result[2];
            let height: i32 = hgt_0.parse().unwrap();
            let unit = hgt_1;

            if unit == "cm" {
                if height < 150 || height > 193{
                    continue;
                }
            }
            else if unit == "in" {
                if height < 59 || height > 76{
                    continue;
                }
            }
            else {
                continue;
            }
        }
        else{
            continue;
        }

        // Validate hair color
        if !re_hcl.captures(&passport).is_some(){
            continue;
        }

        // Validate eye color
        if !re_ecl.captures(&passport).is_some(){
            continue;
        }

        // Validate passport ID
        if !re_pid.captures(&passport).is_some(){
            continue;
        }

        n_valid_passports += 1;
    }

    println!("Numer of valid passports: {}", n_valid_passports);
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
