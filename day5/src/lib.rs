fn get_id(line: &str) -> i32{
    let line_binary = line.replace("B", "1")
    .replace("F", "0")
    .replace("R", "1")
    .replace("L", "0");

    let row = &line_binary[..7];
    let col = &line_binary[7..];

    let row_i32 = i32::from_str_radix(row, 2).unwrap();
    let col_i32 = i32::from_str_radix(col, 2).unwrap();
    let id = row_i32 * 8 + col_i32;

    return id;
}

pub fn part1(contents: String) -> i32{
    let mut id_max = 0;

    for line in contents.lines(){
        let id = get_id(line);
        id_max = std::cmp::max(id, id_max);
    }
    return id_max;
}

pub fn part2(contents: String) -> i32{
    let mut ids = Vec::new();

    for line in contents.lines(){
        let id = get_id(line);
        ids.push(id);
    }

    ids.sort();

    for (i, _) in ids.iter().enumerate() {
        if i < ids.len() - 1{
            if (ids[i+1] - ids[i]) == 2{
                return  ids[i] + 1;
            }
        }
    }
    panic!("Could not solve part2");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_id() {
        assert_eq!(357, get_id("FBFBBFFRLR"));
        assert_eq!(567, get_id("BFFFBBFRRR"));
        assert_eq!(119, get_id("FFFBBBFRRR"));
        assert_eq!(820, get_id("BBFFBBFRLL"));
    }
}
