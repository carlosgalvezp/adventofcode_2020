struct LineInfo {
    min: i32,
    max: i32,
    letter: char,
    password: String,
}

fn get_line_info(line: &str) -> LineInfo {
    let space_tokens: Vec<&str> = line.split(' ').collect();
    let size_part: &str = space_tokens[0];
    let letter_part: &str = space_tokens[1];
    let password: &str = space_tokens[2];

    let min_max: Vec<&str> = size_part.split('-').collect();
    let min = min_max[0].parse::<i32>().unwrap();
    let max = min_max[1].parse::<i32>().unwrap();

    let letter: char = letter_part.chars().next().unwrap();

    return LineInfo {
        min,
        max,
        letter,
        password: password.to_owned(),
    };
}

pub fn part1(contents: String) -> i32{
    let lines = contents.lines();
    let mut valid_passwords = 0;

    for line in lines {
        let line_info = get_line_info(line);

        let mut count = 0;

        for c in line_info.password.chars() {
            if c == line_info.letter {
                count += 1;
            }
        }

        if (count >= line_info.min) && (count <= line_info.max) {
            valid_passwords += 1;
        }
    }

    return valid_passwords;
}

pub fn part2(contents: String) -> i32{
    let lines = contents.lines();
    let mut valid_passwords = 0;

    for line in lines {
        let line_info = get_line_info(line);

        let mut count = 0;

        let idx_min = (line_info.min - 1) as usize;
        let idx_max = (line_info.max - 1) as usize;

        if line_info.password.chars().nth(idx_min).unwrap() == line_info.letter {
            count += 1;
        }

        if line_info.password.chars().nth(idx_max).unwrap() == line_info.letter {
            count += 1;
        }

        if count == 1 {
            valid_passwords += 1;
        }
    }

    return valid_passwords;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_contents() -> String{
        return vec![
            "1-3 a: abcde",
            "1-3 b: cdefg",
            "2-9 c: ccccccccc",
        ].join("\n");
    }

    #[test]
    fn test_part1() {
        assert_eq!(2, part1(get_test_contents()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1, part2(get_test_contents()));
    }
}
