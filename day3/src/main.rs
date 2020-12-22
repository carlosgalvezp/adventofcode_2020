use std::env;
use std::fs;

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

fn day1(contents: String) {
    // Create map
    let map = make_map(contents);

    // Get trees
    let slope = Slope { x: 3, y: 1 };
    let n_trees = get_nr_trees(&map, slope);

    println!("Found {} trees", n_trees);
}

fn day2(contents: String) {
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

    println!("Output: {}", output);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let option: i32 = args[1].parse().unwrap();
    let fname = &args[2];
    let contents = fs::read_to_string(fname).expect("Something went wrong reading the file");

    if option == 1 {
        day1(contents);
    } else if option == 2 {
        day2(contents);
    } else {
        panic!("Wrong option!");
    }
}
