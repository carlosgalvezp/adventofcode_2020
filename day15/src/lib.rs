use std::collections::HashMap;

fn run(input: Vec<i32>, target_number: i32) -> i32{
    let mut spoken_numbers = HashMap::new();
    let mut last_number = 0;

    for turn in 0..target_number{
        if (turn as usize) < input.len(){
            last_number = input[turn as usize];
            spoken_numbers.insert(last_number, vec![turn]);
        }
        else{
            let turns_spoked = &spoken_numbers[&last_number];

            // If it had been spoked only once
            if turns_spoked.len() == 1{
                last_number = 0;
            }
            else{
                last_number = turns_spoked.last().unwrap() - turns_spoked.first().unwrap();
            }

            // Update spoken list
            let mut new_turns_spoked = vec![turn];
            if spoken_numbers.contains_key(&last_number){
                new_turns_spoked.insert(0, *spoken_numbers[&last_number].last().unwrap());
            }
            spoken_numbers.insert(last_number, new_turns_spoked);
        }
    }

    return last_number;
}

pub fn part1(input: Vec<i32>) -> i32{
    return run(input, 2020);
}

pub fn part2(input: Vec<i32>) -> i32{
    return run(input, 30000000);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(1, part1(vec![1, 3, 2]));
        assert_eq!(10, part1(vec![2, 1, 3]));
        assert_eq!(27, part1(vec![1, 2, 3]));
        assert_eq!(78, part1(vec![2, 3, 1]));
        assert_eq!(438, part1(vec![3, 2, 1]));
        assert_eq!(1836, part1(vec![3, 1, 2]));
    }

    #[test]
    fn test_part2() {
        // assert_eq!(175594, part2(vec![0, 3, 6]));
        // assert_eq!(2578, part2(vec![1, 3, 2]));
        // assert_eq!(3544142, part2(vec![2, 1, 3]));
        // assert_eq!(261214, part2(vec![1, 2, 3]));
        // assert_eq!(6895259, part2(vec![2, 3, 1]));
        // assert_eq!(18, part2(vec![3, 2, 1]));
        // assert_eq!(362, part2(vec![3, 1, 2]));
    }
}
