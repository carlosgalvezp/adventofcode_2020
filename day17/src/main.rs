use std::env;
use std::fs;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Position2 {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}


#[derive(Clone, PartialEq)]
enum State{
    Active,
    Inactive,
}

fn print_cells(cells: &HashMap<Position, State>, frontier_min: &Vec<i32>, frontier_max: &Vec<i32>){
    for z in frontier_min[2]..frontier_max[2]{
        println!("z = {}", z);
        for y in frontier_min[0]..frontier_max[1]{
            for x in frontier_min[0]..frontier_max[0]{
                if cells[&Position{x, y, z}] == State::Active {
                    print!("{}", "#");
                }
                else{
                    print!("{}", ".");
                }
            }
            print!("{}", "\n");
        }
    }
}

fn day1(contents: String) {
    let mut cells : HashMap<Position, State> = HashMap::new();

    // Parse input
    let size_x = contents.lines().next().unwrap().len() as i32;
    let size_y = contents.lines().count() as i32;
    let size_z = 1 as i32;
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let position = Position{x: x as i32, y: y as i32, z: 0};
            match c {
                '#' => cells.insert(position, State::Active),
                '.' => cells.insert(position, State::Inactive),
                _ => panic!("Wrong input {}", c),
            };
        }
    }

    // Simulate for 6 iterations
    let mut frontier_min = vec![0, 0, 0];
    let mut frontier_max = vec![size_x, size_y, size_z];

    let n_iterations = 6;
    for _ in 0..n_iterations{
        // Extend frontier
        for i in 0..frontier_min.len(){
            frontier_min[i] -= 1;
            frontier_max[i] += 1;
        }

        // Add inactive neighbors to the frontier
        // Fixed X, vary over YZ
        for yi in frontier_min[1]..frontier_max[1] {
            for zi in frontier_min[2]..frontier_max[2] {
                cells.insert(Position{x: frontier_min[0],     y: yi, z: zi}, State::Inactive);
                cells.insert(Position{x: frontier_max[0] - 1, y: yi, z: zi}, State::Inactive);
            }
        }

        // Fixed Y, vary over XZ
        for xi in frontier_min[0]..frontier_max[0] {
            for zi in frontier_min[2]..frontier_max[2] {
                cells.insert(Position{x: xi, y: frontier_min[1],     z: zi}, State::Inactive);
                cells.insert(Position{x: xi, y: frontier_max[1] - 1, z: zi}, State::Inactive);
            }
        }
        // Fixed Z, vary over XY
        for xi in frontier_min[0]..frontier_max[0] {
            for yi in frontier_min[1]..frontier_max[1] {
                cells.insert(Position{x: xi, y: yi, z: frontier_min[2]    }, State::Inactive);
                cells.insert(Position{x: xi, y: yi, z: frontier_max[2] - 1}, State::Inactive);
            }
        }

        // Update all cells based on neighbors
        let mut next_cells = cells.clone();
        for (cell, state) in &cells{
            let mut n_active_neighbors = 0;

            for xi in -1..2{
                for yi in -1..2{
                    for zi in -1..2{
                        if (xi != 0) || (yi != 0) || (zi != 0){
                            let px = cell.x + xi;
                            let py = cell.y + yi;
                            let pz = cell.z + zi;

                            if (px >= frontier_min[0]) && (px < frontier_max[0]) &&
                               (py >= frontier_min[1]) && (py < frontier_max[1]) &&
                               (pz >= frontier_min[2]) && (pz < frontier_max[2]){
                                let p = Position{x: px, y: py, z: pz};
                                if cells[&p] == State::Active{
                                    n_active_neighbors += 1;
                                }
                            }
                        }
                    }
                }
            }

            match state{
                State::Active => {
                    if !((n_active_neighbors == 2) || (n_active_neighbors == 3)){
                        next_cells.insert(cell.clone(), State::Inactive);
                    }
                },
                State::Inactive => {
                    if n_active_neighbors == 3 {
                        next_cells.insert(cell.clone(), State::Active);
                    }
                }
            }
        }

        cells = next_cells.clone();
    }

    // Count active blocks
    let mut n_active = 0;
    for state in cells.values(){
        if state == &State::Active{
            n_active += 1;
        }
    }

    println!("Part 1 solution: {}", n_active);
}

fn day2(contents: String) {
    let mut cells : HashMap<Position2, State> = HashMap::new();

    // Parse input
    let size_x = contents.lines().next().unwrap().len() as i32;
    let size_y = contents.lines().count() as i32;
    let size_z = 1 as i32;
    let size_w = 1 as i32;
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let position = Position2{x: x as i32, y: y as i32, z: 0, w: 0};
            match c {
                '#' => cells.insert(position, State::Active),
                '.' => cells.insert(position, State::Inactive),
                _ => panic!("Wrong input {}", c),
            };
        }
    }

    // Simulate for 6 iterations
    let mut frontier_min = vec![0, 0, 0, 0];
    let mut frontier_max = vec![size_x, size_y, size_z, size_w];

    let n_iterations = 6;
    for _ in 0..n_iterations{
        // Extend frontier
        for i in 0..frontier_min.len(){
            frontier_min[i] -= 1;
            frontier_max[i] += 1;
        }

        // Add inactive neighbors to the frontier
        // Fixed X, vary over YZW
        for yi in frontier_min[1]..frontier_max[1] {
            for zi in frontier_min[2]..frontier_max[2] {
                for wi in frontier_min[3]..frontier_max[3] {
                    cells.insert(Position2{x: frontier_min[0],     y: yi, z: zi, w: wi}, State::Inactive);
                    cells.insert(Position2{x: frontier_max[0] - 1, y: yi, z: zi, w: wi}, State::Inactive);
                }
            }
        }

        // Fixed Y, vary over XZW
        for xi in frontier_min[0]..frontier_max[0] {
            for zi in frontier_min[2]..frontier_max[2] {
                for wi in frontier_min[3]..frontier_max[3] {
                    cells.insert(Position2{x: xi, y: frontier_min[1],     z: zi, w: wi}, State::Inactive);
                    cells.insert(Position2{x: xi, y: frontier_max[1] - 1, z: zi, w: wi}, State::Inactive);
                }
            }
        }
        // Fixed Z, vary over XYW
        for xi in frontier_min[0]..frontier_max[0] {
            for yi in frontier_min[1]..frontier_max[1] {
                for wi in frontier_min[3]..frontier_max[3] {
                    cells.insert(Position2{x: xi, y: yi, z: frontier_min[2],     w: wi}, State::Inactive);
                    cells.insert(Position2{x: xi, y: yi, z: frontier_max[2] - 1, w: wi}, State::Inactive);
                }
            }
        }
        // Fixed W, vary over XYZ
        for xi in frontier_min[0]..frontier_max[0] {
            for yi in frontier_min[1]..frontier_max[1] {
                for zi in frontier_min[2]..frontier_max[2] {
                    cells.insert(Position2{x: xi, y: yi, z: zi, w: frontier_min[3]    }, State::Inactive);
                    cells.insert(Position2{x: xi, y: yi, z: zi, w: frontier_max[3] - 1}, State::Inactive);
                }
            }
        }

        // Update all cells based on neighbors
        let mut next_cells = cells.clone();
        for (cell, state) in &cells{
            let mut n_active_neighbors = 0;

            for xi in -1..2{
                for yi in -1..2{
                    for zi in -1..2{
                        for wi in -1..2{
                            if (xi != 0) || (yi != 0) || (zi != 0 || (wi != 0)){
                                let px = cell.x + xi;
                                let py = cell.y + yi;
                                let pz = cell.z + zi;
                                let pw = cell.w + wi;

                                if (px >= frontier_min[0]) && (px < frontier_max[0]) &&
                                   (py >= frontier_min[1]) && (py < frontier_max[1]) &&
                                   (pz >= frontier_min[2]) && (pz < frontier_max[2]) &&
                                   (pw >= frontier_min[3]) && (pw < frontier_max[3]){
                                    let p = Position2{x: px, y: py, z: pz, w: pw};
                                    if cells[&p] == State::Active{
                                        n_active_neighbors += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            match state{
                State::Active => {
                    if !((n_active_neighbors == 2) || (n_active_neighbors == 3)){
                        next_cells.insert(cell.clone(), State::Inactive);
                    }
                },
                State::Inactive => {
                    if n_active_neighbors == 3 {
                        next_cells.insert(cell.clone(), State::Active);
                    }
                }
            }
        }

        cells = next_cells.clone();
    }

    // Count active blocks
    let mut n_active = 0;
    for state in cells.values(){
        if state == &State::Active{
            n_active += 1;
        }
    }

    println!("Part 2 solution: {}", n_active);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let option: i32 = args[1].parse().unwrap();
    let fname = &args[2];
    let contents = fs::read_to_string(fname).expect("Something went wrong reading the file");

    // let contents = ".#.\n\
    //                 ..#\n\
    //                 ###\n".to_string();

    if option == 1 {
        day1(contents);
    } else if option == 2 {
        day2(contents);
    } else {
        panic!("Wrong option!");
    }
}
