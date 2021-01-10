pub fn part1(contents: String) -> i32{
    let numbers : Vec<i32> = contents.lines().map(|x| x.parse::<i32>().unwrap()).collect();

    for i in 0..numbers.len() - 1 {
        for j in i + 1..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                return numbers[i] * numbers[j];
            }
        }
    }
    panic!("Could not solve part1");
}

pub fn part2(contents: String) -> i32{
    let numbers : Vec<i32> = contents.lines().map(|x| x.parse::<i32>().unwrap()).collect();

    for i in 0..numbers.len() - 2 {
        for j in i + 1..numbers.len() - 1 {
            for k in j + 1..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    return numbers[i] * numbers[j] * numbers[k];
                }
            }
        }
    }
    panic!("Could not solve part2")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_contents() -> String{
        return vec![
            "1721",
            "979",
            "366",
            "299",
            "675",
            "1456",
        ].join("\n");
    }

    #[test]
    fn test_part1() {
        assert_eq!(514579, part1(get_test_contents()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(241861950, part2(get_test_contents()));
    }
}
