fn get_cup_idx(cups: &Vec<i32>, cup: i32) -> usize {
    return cups.iter().position(|&x| x == cup).unwrap();
}

#[derive(PartialEq)]
struct State{
    cups: Vec<i32>,
    current_cup: i32,
}

fn run(original_cups: &Vec<i32>, n_moves: i32) -> Vec<i32>{
    let mut cups = original_cups.clone();
    let mut current_cup = cups[0];

    for i in 0..n_moves{
        if (i % 10000) == 0{
            println!("Iteration {}/{} ({}%)", i, n_moves, (i as f32)/(n_moves as f32) * 100.0);
        }
        // Take 3 cups
        let mut cups_to_move = Vec::new();
        for _ in 0..3{
            let idx = (get_cup_idx(&cups, current_cup) + 1) % cups.len();
            let cup = cups[idx];
            cups_to_move.push(cup);
            cups.remove(idx);
        }

        // Get destination cup
        let mut destination_cup = current_cup - 1;
        let min_label_remaining = *cups.iter().min().unwrap();
        let max_label_remaining = *cups.iter().max().unwrap();

        while !cups.contains(&destination_cup){
            destination_cup -= 1;
            if destination_cup < min_label_remaining{
                destination_cup = max_label_remaining;
            }
        }

        assert!(destination_cup > 0);

        // Move cups
        for i in 0..cups_to_move.len(){
            let destination_cup_idx = get_cup_idx(&cups, destination_cup);
            cups.insert((destination_cup_idx + i + 1) % cups.len(), cups_to_move[i]);
        }

        // Select new current cup
        let next_current_cup_idx = (get_cup_idx(&cups, current_cup) + 1) % cups.len();
        current_cup = cups[next_current_cup_idx];
    }

    return cups;
}

pub fn part1(contents: String) -> String{
    let cups: Vec<i32> = contents.chars().map(|x| x.to_digit(10).unwrap() as i32).collect();

    // Run for 100 moves
    let cups = run(&cups, 100);

    // Find cup with label 1
    let cup_1_idx = cups.iter().position(|&x| x == 1).unwrap();

    // Get output
    let mut output = Vec::new();
    for i in 0..cups.len() - 1{
        output.push(cups[(cup_1_idx + i + 1) % cups.len()].to_string());
    }
    return output.join("");
}

pub fn part2(contents: String) -> i64{
    let mut cups: Vec<i32> = contents.chars().map(|x| x.to_digit(10).unwrap() as i32).collect();

    for i in 10..=1000000{
        cups.push(i);
    }

    // Run for 10 000 000 moves
    let cups = run(&cups, 10000000);

    // Get output
    let cup_1_idx = cups.iter().position(|&x| x == 1).unwrap();

    let c1 = cups[(cup_1_idx + 1) % cups.len()] as i64;
    let c2 = cups[(cup_1_idx + 2) % cups.len()] as i64;
    return c1 * c2;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_contents() -> String{
        return "389125467".to_owned();
    }

    #[test]
    fn test_part1() {
        assert_eq!("67384529", part1(get_test_contents()));
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(111057672960, part2(get_test_contents()));
    // }
}
