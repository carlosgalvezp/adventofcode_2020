use std::env;
use std::fs;

#[derive(Clone)]
struct Map {
    size_x: i32,
    size_y: i32,
    state: Vec<char>,
}

impl Map{
    fn new(contents: String) -> Map{
        let size_y = contents.lines().count() as i32;
        let size_x = contents.lines().next().unwrap().chars().count() as i32;
        let mut state = Vec::new();

        for line in contents.lines(){
            for c in line.chars(){
                state.push(c);
            }
        }

        return Map{size_x, size_y, state};
    }

    fn idx(&self, x: i32, y: i32) -> usize{
        return (y * self.size_x + x) as usize;
    }

    fn set(&mut self, x: i32, y: i32, c: char){
        let idx = self.idx(x, y);
        self.state[idx] = c;
    }

    fn get(&self, x: i32, y: i32) -> char{
        let idx = self.idx(x, y);
        return self.state[idx];
    }
}

fn get_nr_occupied_part1(map: &Map, x: i32, y: i32) -> i32 {
    let mut result = 0;

    let x_min = std::cmp::max(x - 1, 0);
    let y_min = std::cmp::max(y - 1, 0);

    let x_max = std::cmp::min(x + 1, map.size_x - 1);
    let y_max = std::cmp::min(y + 1, map.size_y - 1);

    for yi in y_min..y_max + 1{
        for xi in x_min..x_max + 1{
            if (xi != x || yi != y) && map.get(xi, yi) == '#'{
                result += 1;
            }
        }
    }

    return result;
}

fn get_nr_occupied_part2(map: &Map, x: i32, y: i32) -> i32 {
    let mut result = 0;

    let dirs_x = vec![1, -1, 0,  0, 1,  1, -1, -1];
    let dirs_y = vec![0,  0, 1, -1, 1, -1,  1, -1];

    for i in 0..dirs_x.len(){
        let mut xi = x;
        let mut yi = y;
        loop{
            xi += dirs_x[i];
            yi += dirs_y[i];

            if xi < 0 || yi < 0 || xi >= map.size_x || yi >= map.size_y{
                break;
            }

            if map.get(xi, yi) == '#'{
                result += 1;
                break;
            }
            else if map.get(xi, yi) == 'L'{
                break;
            }
        }
    }

    return result;
}

fn simulate_seating(contents: String, occupancy_threshold: i32, occupancy_fn: &dyn Fn(&Map, i32, i32) -> i32) -> i32{
    let mut map_current = Map::new(contents);
    let mut map_next = map_current.clone();

    // Update map until stabilized
    loop{
        map_current = map_next.clone();

        for y in 0..map_current.size_y{
            for x in 0..map_current.size_x{
                if map_current.get(x, y) == 'L'{
                    let n_occupied = occupancy_fn(&map_current, x, y);
                    if n_occupied == 0{
                        map_next.set(x, y, '#');
                    }
                }
                else if map_current.get(x, y) == '#'{
                    let n_occupied = occupancy_fn(&map_current, x, y);
                    if n_occupied >= occupancy_threshold {
                        map_next.set(x, y, 'L');
                    }
                }
            }
        }

        if map_next.state == map_current.state{
            break;
        }
    }

    // Count the number of occupied seats
    return map_current.state.iter().filter(|&&x| x == '#').count() as i32;
}

fn day1(contents: String) {
    let occupancy_threshold = 4;
    let occupancy_fn = &get_nr_occupied_part1;
    let n_occupied = simulate_seating(contents, occupancy_threshold, occupancy_fn);
    println!("Number of occupied places after stabilize: {}", n_occupied);
}

fn day2(contents:String) {
    let occupancy_threshold = 5;
    let occupancy_fn = &get_nr_occupied_part2;
    let n_occupied = simulate_seating(contents, occupancy_threshold, occupancy_fn);
    println!("Number of occupied places after stabilize: {}", n_occupied);
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
