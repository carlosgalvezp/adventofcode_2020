pub fn part1(contents: String) -> i32{
    let group_answers = contents.split("\n\n").map(|x| x.to_owned() + "\n");

    let mut n_yes_answers = 0;

    for mut group_answer in group_answers{
        group_answer = group_answer.replace("\n", "");
        let mut group_unique: Vec<char> = group_answer.chars().collect();
        group_unique.sort();
        group_unique.dedup();
        n_yes_answers += group_unique.len() as i32;
    }

    return n_yes_answers;
}

pub fn part2(contents: String) -> i32{
    let group_answers = contents.split("\n\n").map(|x| x.to_owned());

    let mut n_yes_answers = 0;

    for mut group_answer in group_answers{
        let n_people_in_group = group_answer.lines().count();

        group_answer = group_answer.replace("\n", "");
        let mut group_sorted: Vec<char> = group_answer.chars().collect();
        group_sorted.sort();

        let mut all_yes: Vec<char> = Vec::new();

        for i in 0..group_sorted.len() - (n_people_in_group - 1){
            if group_sorted[i] == group_sorted[i + n_people_in_group - 1]{
                all_yes.push(group_sorted[i]);
            }
        }

        n_yes_answers += all_yes.len() as i32;
    }

    return n_yes_answers;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_contents() -> String{
        return vec![
            "abc",
            "",
            "a",
            "b",
            "c",
            "",
            "ab",
            "ac",
            "",
            "a",
            "a",
            "a",
            "a",
            "",
            "b",
        ].join("\n");
    }

    #[test]
    fn test_part1() {
        assert_eq!(11, part1(get_test_contents()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(6, part2(get_test_contents()));
    }
}
