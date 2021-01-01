use std::env;
use std::fs;
use regex::Regex;

struct Size {
    x: i32,
    y: i32,
}

struct Image {
    size: Size,
    data: Vec<char>,
}

struct ImageTile {
    id: i32,
    image: Image,
}

impl Image {
    fn at(&self, x: i32, y: i32) -> char {
        let idx = y * self.size.x + x;
        return self.data[idx as usize];
    }
}

fn parse_image_tiles(contents: String) -> Vec<ImageTile> {
    let mut output = Vec::new();

    let re_tile_number = Regex::new(r"^Tile (\d+):$").unwrap();
    for tile_contents in contents.trim().split("\n\n"){
        let mut it = tile_contents.lines();
        let tile_id = re_tile_number.captures(it.next().unwrap()).unwrap()[1].parse::<i32>().unwrap();
        let mut tile_data = Vec::new();

        for line in it{
            for c in line.chars(){
                tile_data.push(c);
            }
        }

        let size_x = tile_contents.lines().nth(1).unwrap().chars().count() as i32;
        let size_y = (tile_contents.lines().count() - 1) as i32;

        output.push(ImageTile{id: tile_id, image: Image{size: Size{x: size_x, y: size_y}, data: tile_data}});
    }
    return output;
}

fn get_all_possible_edges(tile: &ImageTile, include_reverse: bool) -> Vec<Vec<char>>{
    let mut output = Vec::new();

    let mut edge_a = Vec::new();
    let mut edge_b = Vec::new();
    let mut edge_c = Vec::new();
    let mut edge_d = Vec::new();

    for x in 0..tile.image.size.x{
        edge_a.push(tile.image.at(x,                     0));
        edge_b.push(tile.image.at(x, tile.image.size.y - 1));
    }

    for y in 0..tile.image.size.y{
        edge_c.push(tile.image.at(0,                     y));
        edge_d.push(tile.image.at(tile.image.size.x - 1, y));
    }

    output.push(edge_a);
    output.push(edge_b);
    output.push(edge_c);
    output.push(edge_d);

    if include_reverse{
        let tmp = output.clone();
        for mut edge in tmp{
            edge.reverse();
            output.push(edge);
        }
    }

    return output;
}

fn get_nr_matching_edges(a: &ImageTile, b: &ImageTile) -> i32{
    let mut output = 0;

    let edges_a = get_all_possible_edges(a, true);
    let edges_b = get_all_possible_edges(b, false);

    for edge_a in &edges_a{
        for edge_b in &edges_b{
            if edge_a == edge_b{
                output += 1;
            }
        }
    }

    return output;
}

fn day1(contents: String){
    let tiles = parse_image_tiles(contents);
    let mut corner_tiles = Vec::new();

    for i in 0..tiles.len() {
        let tile_i = &tiles[i];
        let mut n_matched_edges = 0;
        for j in 0..tiles.len() {
            if i != j{
                let tile_j = &tiles[j];
                n_matched_edges += get_nr_matching_edges(tile_i, tile_j);
            }
        }

        // It's a corner if only 2 edges match
        if n_matched_edges < 2{
            panic!("Not enough matched edges!");
        }
        // assert!(n_matched_edges >= 2);
        if n_matched_edges == 2 {
            corner_tiles.push(tile_i);
        }
    }

    assert!(corner_tiles.len() == 4);
    let mut result: i64 = 1;
    for tile in corner_tiles{
        result *= tile.id as i64;
    }
    println!("Part 1 solution: {}", result);
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
