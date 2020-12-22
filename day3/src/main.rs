use std::env;
use std::fs;

fn day1(contents: String){
    let lines = contents.lines();

    // Create map
    let mut map = Vec::new();

    for line in lines{
        let mut map_row = Vec::new();
        for c in line.chars(){
            map_row.push(c);
        }
        map.push(map_row);
    }

    let n_rows = map.len();
    let x_max = map[0].len();

    let mut x = 0;
    let mut y = 0;
    let mut n_trees = 0;

    while y < n_rows{
        if map[y][x] == '#'{
            n_trees += 1;
        }

        x = (x + 3) % x_max;
        y = y + 1;
    }

    println!("Found {} trees", n_trees);
}

fn day2(contents: String){

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let option : i32 = args[1].parse().unwrap();
    let fname = &args[2];
    let contents = fs::read_to_string(fname).expect("Something went wrong reading the file");

    if option == 1{
        day1(contents);
    }
    else if option == 2{
        day2(contents);
    }
    else{
        panic!("Wrong option!");
    }
}
