#[derive(Clone, Copy, PartialEq)]
enum Color{
    White,
    Black,
}

#[derive(Clone)]
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

    fn at_grid(&self, x: i32, y: i32) -> &Color{
        let idx = y * self.size + x;
        return &self.data[idx as usize];
    }

    fn at_grid_mut(&mut self, x: i32, y: i32) -> &mut Color{
        let idx = y * self.size + x;
        return &mut self.data[idx as usize];
    }
}

fn get_map(contents: String) -> Map{
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

    return map;
}

fn get_n_black_neighbors(map: &Map, x: i32, y: i32) -> i32{
    let diffs = vec![
        vec![ 2,  0],  // East
        vec![-2,  0],  // West
        vec![ 1,  1],  // North-east
        vec![ 1, -1],  // South-east
        vec![-1,  1],  // North-west
        vec![-1, -1],  // South-west
    ];

    let mut n_black = 0;

    for diff in diffs{
        let px = x + diff[0];
        let py = y + diff[1];

        if (px >= 0) && (px < map.size) && (py >= 0) && (py < map.size){
            if *map.at_grid(px, py) == Color::Black{
                n_black += 1;
            }
        }
    }

    return n_black;
}

pub fn part1(contents: String) -> i64{
    let map = get_map(contents);
    let n_black = map.data.iter().filter(|&&x| x == Color::Black).count() as i64;
    return n_black;
}

pub fn part2(contents: String) -> i64{
    let mut map = get_map(contents);
    let mut next_map = map.clone();

    let n_days = 100;
    for d in 0..n_days{
        let mut x_begin = 1;

        for y in 0..map.size{
            x_begin = 1 - x_begin;
            for x in (x_begin..map.size).step_by(2){
                let n_black = get_n_black_neighbors(&map, x, y);

                match map.at_grid(x, y){
                    Color::Black => {
                        if (n_black == 0) || (n_black > 2) {
                            *next_map.at_grid_mut(x, y) = Color::White;
                        }
                    },
                    Color::White => {
                        if n_black == 2{
                            *next_map.at_grid_mut(x, y) = Color::Black;
                        }
                    },
                }
            }
        }

        map = next_map.clone();

        let n_black_day = map.data.iter().filter(|&&x| x == Color::Black).count();
        println!("Day {}: {}", d + 1, n_black_day);
    }

    let n_black = map.data.iter().filter(|&&x| x == Color::Black).count() as i64;
    return n_black;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_contents() -> String{
        return vec![
            "sesenwnenenewseeswwswswwnenewsewsw",
            "neeenesenwnwwswnenewnwwsewnenwseswesw",
            "seswneswswsenwwnwse",
            "nwnwneseeswswnenewneswwnewseswneseene",
            "swweswneswnenwsewnwneneseenw",
            "eesenwseswswnenwswnwnwsewwnwsene",
            "sewnenenenesenwsewnenwwwse",
            "wenwwweseeeweswwwnwwe",
            "wsweesenenewnwwnwsenewsenwwsesesenwne",
            "neeswseenwwswnwswswnw",
            "nenwswwsewswnenenewsenwsenwnesesenew",
            "enewnwewneswsewnwswenweswnenwsenwsw",
            "sweneswneswneneenwnewenewwneswswnese",
            "swwesenesewenwneswnwwneseswwne",
            "enesenwswwswneneswsenwnewswseenwsese",
            "wnwnesenesenenwwnenwsewesewsesesew",
            "nenewswnwewswnenesenwnesewesw",
            "eneswnwswnwsenenwnwnwwseeswneewsenese",
            "neswnwewnwnwseenwseesewsenwsweewe",
            "wseweeenwnesenwwwswnew",
        ].join("\n");
    }

    #[test]
    fn test_part1() {
        assert_eq!(10, part1(get_test_contents()));
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(2208, part2(get_test_contents()));
    // }
}
