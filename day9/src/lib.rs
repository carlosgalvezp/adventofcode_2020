fn get_numbers(contents: String) -> Vec<i64> {
    let mut numbers = Vec::new();

    for line in contents.lines(){
        numbers.push(line.parse::<i64>().unwrap());
    }

    return numbers;
}

fn get_invalid_number(numbers: &Vec<i64>, preamble_length: usize) -> i64{
    for i in preamble_length..numbers.len(){
        let mut found = false;
        for j in i-preamble_length..i{
            for k in j+1..i{
                if numbers[j] + numbers[k] == numbers[i]{
                    found = true;
                    break;
                }
            }
            if found{
                break;
            }
        }

        if !found{
            return numbers[i];
        }
    }
    panic!("Could not find number");
}

pub fn part1(contents: String, preamble_length: usize) -> i64{
    let numbers = get_numbers(contents);
    let invalid_number = get_invalid_number(&numbers, preamble_length);
    return invalid_number;
}

pub fn part2(contents: String, preamble_length: usize) -> i64{
    let numbers = get_numbers(contents);
    let invalid_number = get_invalid_number(&numbers, preamble_length);

    for i in 0..numbers.len() - 1{
        for j in i+1..numbers.len(){
            let mut sum = 0;

            let mut min_number = i64::MAX;
            let mut max_number = 0;
            for k in i..j+1{
                sum += numbers[k];

                min_number = std::cmp::min(min_number, numbers[k]);
                max_number = std::cmp::max(max_number, numbers[k]);
            }

            if sum == invalid_number{
                return min_number + max_number;
            }
            if sum > invalid_number{
                break;
            }
        }
    }
    panic!("Could not solve part2");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_contents() -> String{
        return vec![
            "35",
            "20",
            "15",
            "25",
            "47",
            "40",
            "62",
            "55",
            "65",
            "95",
            "102",
            "117",
            "150",
            "182",
            "127",
            "219",
            "299",
            "277",
            "309",
            "576",
        ].join("\n");
    }

    #[test]
    fn test_part1() {
        assert_eq!(127, part1(get_test_contents(), 5));
    }

    #[test]
    fn test_part2() {
        assert_eq!(62, part2(get_test_contents(), 5));
    }
}
