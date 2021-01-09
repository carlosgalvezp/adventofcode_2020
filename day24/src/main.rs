use std::env;
use std::fs;

#[derive(Clone, Copy, PartialEq)]
enum Color{
    White,
    Black,
}

struct Map{
    size: i32,
    data: Vec<Color>,
}

fn idx(x: i32, y: i32, size: i32) -> usize{
    let x_offset = (size / 2) + x;
    let y_offset = (size / 2) + y;
    let idx = y_offset * size + x_offset;
    return idx as usize;
}

impl Map{
    fn new(size: i32) -> Map{
        return Map{size, data: vec![Color::White; (size * size) as usize]};
    }

    fn at(&self, x: i32, y: i32) -> &Color{
        return &self.data[idx(x, y, self.size)];
    }

    fn at_mut(&mut self, x: i32, y: i32) -> &mut Color{
        return &mut self.data[idx(x, y, self.size)];
    }
}

fn day1(contents: String){
    let map_size = 4 * contents.lines().count() as i32;
    let mut map = Map::new(map_size);

    for line in contents.lines(){
        let mut px = 0;
        let mut py = 0;

        let mut it = line.chars().into_iter().peekable();
        while it.peek().is_some(){
            let c = it.next().unwrap();

            match c{
                'w' => {
                    px -= 2;
                },
                'e' => {
                    px += 2;
                },
                'n' => {
                    let c2 = it.next().unwrap();
                    py += 1;
                    match c2{
                        'e' => {
                            px += 1;
                        },
                        'w' => {
                            px -= 1;
                        },
                        _ => { panic!("Bad character {}", c2); },
                    }
                },
                's' => {
                    let c2 = it.next().unwrap();
                    py -= 1;
                    match c2{
                        'e' => {
                            px += 1;
                        },
                        'w' => {
                            px -= 1;
                        },
                        _ => { panic!("Bad character {}", c2); },
                    }
                },
                _ => { panic!("Bad character {}", c); }
            }
        }

        match map.at(px, py){
            Color::Black => *map.at_mut(px, py) = Color::White,
            Color::White => *map.at_mut(px, py) = Color::Black,
        };
    }

    let n_black = map.data.iter().filter(|&&x| x == Color::Black).count();
    println!("Part 1 solution: {}", n_black);
}

fn day2(contents: String){
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
