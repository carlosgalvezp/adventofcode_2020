use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Key{
    subject_number: i64,
    loop_size: i64,
}

fn transform(subject_number: i64, loop_size: i64, cache: &mut HashMap<Key, i64>) -> i64{
    let k = Key{subject_number, loop_size};
    if cache.contains_key(&k){
        return cache[&k];
    }

    let mut value = 1;
    let k_1 = Key{subject_number, loop_size: loop_size - 1};
    if cache.contains_key(&k_1){
        value = cache[&k_1];
        value *= subject_number;
        value = value % 20201227;
    }
    else{
        for _ in 0..loop_size{
            value *= subject_number;
            value = value % 20201227;
        }
    }

    cache.insert(k, value);

    return value;
}

fn get_secret_loop_size(public_key: i64, cache: &mut HashMap<Key, i64>) -> i64{
    println!("Getting secret loop size for key {}...", public_key);
    let mut loop_size = 1;
    loop{
        let result = transform(7, loop_size, cache);
        if result == public_key{
            break;
        }
        loop_size += 1;
    }
    return loop_size;
}

pub fn part1(contents: String) -> i64{
    let card_public_key = contents.lines().into_iter().nth(0).unwrap().parse::<i64>().unwrap();
    let door_public_key = contents.lines().into_iter().nth(1).unwrap().parse::<i64>().unwrap();

    let mut cache = HashMap::new();
    let card_secret_loop_size = get_secret_loop_size(card_public_key, &mut cache);
    let door_secret_loop_size = get_secret_loop_size(door_public_key, &mut cache);

    let result_1 = transform(card_public_key, door_secret_loop_size, &mut cache);
    let result_2 = transform(door_public_key, card_secret_loop_size, &mut cache);
    assert!(result_1 == result_2);

    return result_1;
}

pub fn part2(_: String) -> String{
    return "Congratulations for completing AoC 2020!".to_owned();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_contents() -> String{
        return vec![
            "5764801",
            "17807724",
        ].join("\n");
    }

    #[test]
    fn test_part1() {
        assert_eq!(14897079, part1(get_test_contents()));
    }

    #[test]
    fn test_part2() {
        assert_eq!("Congratulations for completing AoC 2020!", part2(get_test_contents()));
    }
}
