use regex::Regex;

pub fn part1(contents: String) -> i32{
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

    return n_valid_passports;
}

pub fn part2(contents: String) -> i32{
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

    return n_valid_passports;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_contents() -> String{
        return vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
            "byr:1937 iyr:2017 cid:147 hgt:183cm",
            "",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
            "hcl:#cfa07d byr:1929",
            "",
            "hcl:#ae17e1 iyr:2013",
            "eyr:2024",
            "ecl:brn pid:760753108 byr:1931",
            "hgt:179cm",
            "",
            "hcl:#cfa07d eyr:2025 pid:166559648",
            "iyr:2011 ecl:brn hgt:59in",
        ].join("\n");
    }

    #[test]
    fn test_part1() {
        assert_eq!(2, part1(get_test_contents()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2, part2(get_test_contents()));
    }
}
