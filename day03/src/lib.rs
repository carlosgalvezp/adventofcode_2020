struct Slope {
    x: usize,
    y: usize,
}

fn make_map(contents: String) -> Vec<Vec<char>> {
    let lines = contents.lines();

    let mut map = Vec::new();

    for line in lines {
        let mut map_row = Vec::new();
        for c in line.chars() {
            map_row.push(c);
        }
        map.push(map_row);
    }

    return map;
}

fn get_nr_trees(map: &Vec<Vec<char>>, slope: Slope) -> i32 {
    let n_rows = map.len();
    let x_max = map[0].len();

    let mut x = 0;
    let mut y = 0;
    let mut n_trees = 0;

    while y < n_rows {
        if map[y][x] == '#' {
            n_trees += 1;
        }

        x = (x + slope.x) % x_max;
        y = y + slope.y;
    }

    return n_trees;
}

pub fn part1(contents: String) -> i32{
    // Create map
    let map = make_map(contents);

    // Get trees
    let slope = Slope { x: 3, y: 1 };
    let n_trees = get_nr_trees(&map, slope);

    return n_trees;
}

pub fn part2(contents: String) -> i64{
    // Create map
    let map = make_map(contents);

    // Define slope combinations
    let slopes = vec![
        Slope { x: 1, y: 1 },
        Slope { x: 3, y: 1 },
        Slope { x: 5, y: 1 },
        Slope { x: 7, y: 1 },
        Slope { x: 1, y: 2 },
    ];

    // Get trees for combinations and multiply
    let mut output: i64 = 1;
    for slope in slopes {
        output = output * (get_nr_trees(&map, slope) as i64);
    }

    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_contents() -> String{
        return vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ].join("\n");
    }

    #[test]
    fn test_part1() {
        assert_eq!(7, part1(get_test_contents()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(336, part2(get_test_contents()));
    }
}
