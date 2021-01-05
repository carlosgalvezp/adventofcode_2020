use std::env;
use std::fs;
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone)]
struct Image {
    size_x: i32,
    size_y: i32,
    data: Vec<char>,
}

#[derive(Clone)]
struct ImageTile {
    tile_id: i32,
    image: Image,
}

impl Image {
    fn new(size_x: i32, size_y: i32) -> Image {
        return Image{size_x, size_y, data: vec![' '; (size_x * size_y) as usize]};
    }

    fn at(&self, x: i32, y: i32) -> char {
        let idx = y * self.size_x + x;
        return self.data[idx as usize];
    }

    fn set(&mut self, x: i32, y: i32, c: char){
        let idx = y * self.size_x + x;
        return self.data[idx as usize] = c;
    }
}

#[derive(Clone)]
struct TileNode{
    idx: i32,
    top: i32,
    bottom: i32,
    left: i32,
    right: i32,
}

impl TileNode {
    fn new(idx: i32) -> TileNode{
        return TileNode{idx, top: -1, bottom: -1, left: -1, right: -1};
    }

    fn n_neighbors(&self) -> i32{
        return (self.top >= 0)  as i32 +
               (self.bottom >= 0) as i32 +
               (self.left >= 0) as i32 +
               (self.right >= 0) as i32
    }
}

type TileGraph = Vec<TileNode>;

enum EdgePosition{
    Top,
    Bottom,
    Left,
    Right,
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

        let size = (tile_contents.lines().count() - 1) as i32;

        output.push(ImageTile{tile_id, image: Image{size_x: size, size_y: size, data: tile_data}});
    }
    return output;
}

fn get_edge(image: &Image, edge: EdgePosition) -> Vec<char>{
    let mut output = Vec::new();
    for i in 0..image.size_x{
        let c = match edge{
            EdgePosition::Top => image.at(i, 0),
            EdgePosition::Bottom => image.at(i, image.size_x - 1),
            EdgePosition::Left => image.at(0, i),
            EdgePosition::Right => image.at(image.size_x - 1, i),
        };
        output.push(c);
    }
    return output;
}

fn get_tile_configurations(image: &Image) -> Vec<Image> {
    let mut output = Vec::new();

    for n_rotations in vec![0, 1, 2, 3]{
        for do_hflip in vec![false, true] {
            for do_vflip in vec![false, true] {
                let mut new_image = image.clone();

                for _ in 0..n_rotations{
                    new_image = rotate(&new_image);
                }

                if do_hflip{
                    new_image = hflip(&new_image);
                }

                if do_vflip{
                    new_image = vflip(&new_image);
                }

                output.push(new_image);
            }
        }
    }

    return output;
}

fn rotate(image: &Image) -> Image {
    let mut output = image.clone();
    std::mem::swap(&mut output.size_x, &mut output.size_y);
    for y in 0..output.size_y{
        for x in 0..output.size_x{
            output.set(x, y, image.at(y, output.size_x - 1 -x));
        }
    }
    return output;
}

fn hflip(image: &Image) -> Image {
    let mut output = image.clone();
    for y in 0..image.size_y{
        for x in 0..image.size_x{
            output.set(x, y, image.at(image.size_x - 1 - x, y));
        }
    }
    return output;
}

fn vflip(image: &Image) -> Image {
    let mut output = image.clone();
    for y in 0..image.size_y{
        for x in 0..image.size_x{
            output.set(x, y, image.at(x, image.size_y - 1 - y));
        }
    }
    return output;
}

fn get_tile_graph(tiles: Vec<ImageTile>) -> (TileGraph, Vec<ImageTile>){
    let mut tile_graph = TileGraph::new();
    let mut final_tiles = Vec::new();

    let mut tiles_copy = tiles.clone();

    // The first new tile will be unmodified (not rotated or flipped), the rest will
    // be rearranged to fit it
    tile_graph.push(TileNode::new(0));
    final_tiles.push(tiles_copy.pop().unwrap());

    // Get a hashmap of all possible configurations for the remaining pieces
    let mut all_tile_configurations = HashMap::new();
    for tile in &tiles_copy{
        all_tile_configurations.insert(tile.tile_id, get_tile_configurations(&tile.image));
    }

    while !tiles_copy.is_empty(){
        let mut tile_node = TileNode::new(final_tiles.len() as i32);

        for j in 0..tiles_copy.len(){
            let tile = &tiles_copy[j];

            // Get all possible tile configurations
            let tile_configurations = &all_tile_configurations[&tile.tile_id];

            // Match it to any of the graph tiles and add to the graph
            let mut matched = false;
            for c in tile_configurations{
                // Try to match it to any piece in the graph
                for i in 0..tile_graph.len(){
                    let tile_graph_i_image = &final_tiles[tile_graph[i].idx as usize].image;

                    if get_edge(tile_graph_i_image, EdgePosition::Top) == get_edge(c, EdgePosition::Bottom){
                        matched = true;
                        tile_graph[i].top = tile_node.idx;
                        tile_node.bottom = tile_graph[i].idx;
                    }
                    else if get_edge(tile_graph_i_image, EdgePosition::Bottom) == get_edge(c, EdgePosition::Top){
                        matched = true;
                        tile_graph[i].bottom = tile_node.idx;
                        tile_node.top = tile_graph[i].idx;
                    }
                    else if get_edge(tile_graph_i_image, EdgePosition::Left) == get_edge(c, EdgePosition::Right){
                        matched = true;
                        tile_graph[i].left = tile_node.idx;
                        tile_node.right = tile_graph[i].idx;
                    }
                    else if get_edge(tile_graph_i_image, EdgePosition::Right) == get_edge(c, EdgePosition::Left){
                        matched = true;
                        tile_graph[i].right = tile_node.idx;
                        tile_node.left = tile_graph[i].idx;
                    }
                }
                if matched{
                    tile_graph.push(tile_node.clone());
                    final_tiles.push(ImageTile{tile_id: tile.tile_id, image: c.clone()});
                    break;
                }
            }

            if matched {
                tiles_copy.remove(j);
                break;
            }
        }
    }

    return (tile_graph, final_tiles);
}

fn get_image_from_tile_graph(tile_graph: &TileGraph, tiles: &Vec<ImageTile>) -> Image{
    let n_tiles_side = (tile_graph.len() as f64).sqrt().round() as i32;
    let tile_size = tiles[0].image.size_x;
    let image_size = n_tiles_side * (tile_size - 2);

    let mut output = Image::new(image_size, image_size);

    // Find top-left corner first
    let mut idx = 0;
    while tile_graph[idx as usize].top >= 0{
        idx = tile_graph[idx as usize].top;
    }

    while tile_graph[idx as usize].left >= 0{
        idx = tile_graph[idx as usize].left;
    }

    // Traverse graph and write to image
    for y in (0..image_size).step_by((tile_size - 2) as usize){
        for x in (0..image_size).step_by((tile_size - 2) as usize){
            for yi in 0..tile_size - 2{
                for xi in 0..tile_size - 2{
                    output.set(x + xi, y + yi, tiles[idx as usize].image.at(1 + xi, 1 + yi));
                }
            }

            if tile_graph[idx as usize].right >= 0{
                idx = tile_graph[idx as usize].right;
            }else{
                while tile_graph[idx as usize].left >= 0{
                    idx = tile_graph[idx as usize].left;
                }
                if tile_graph[idx as usize].bottom >= 0{
                    idx = tile_graph[idx as usize].bottom;
                }
            }
        }
    }

    return output;
}

fn get_image_from_str(str: String) -> Image{
    let mut data = Vec::new();
    let size_y = str.lines().count() as i32;
    let size_x = str.lines().into_iter().next().unwrap().chars().count() as i32;

    for line in str.lines(){
        for c in line.chars(){
            data.push(c);
        }
    }

    return Image{size_x, size_y, data};
}

fn find_monsters(image: Image) -> Image{
    let mut output = image.clone();

    // Create image of monster pattern
    let monster_pattern = concat!(
        "                  # \n",
        "#    ##    ##    ###\n",
        " #  #  #  #  #  #   \n",
    );
    let monster_pattern_image = get_image_from_str(monster_pattern.to_owned());
    let n_monster_pattern_cells = monster_pattern_image.data.iter().filter(|&&x| x == '#').count();

    // Get all combinations of it
    let all_monster_pattern_configurations = get_tile_configurations(&monster_pattern_image);

    // Pass the pattern through the image and check if it matches the pattern
    for monster in &all_monster_pattern_configurations{
        for dy in 0..image.size_y - monster.size_y{
            for dx in 0..image.size_x - monster.size_x{
                // Count number of cells that match the pattern
                let mut n_cells_matched = 0;
                for y in 0..monster.size_y{
                    for x in 0..monster.size_x{
                        if monster.at(x, y) == '#' &&
                           (output.at(x + dx, y + dy) == '#' || output.at(x + dx, y + dy) == 'O'){
                            n_cells_matched += 1;
                        }
                    }
                }

                // If all cells fit the monster pattern, fill with "O" where the monster is
                if n_cells_matched == n_monster_pattern_cells{
                    for y in 0..monster.size_y{
                        for x in 0..monster.size_x{
                            if monster.at(x, y) == '#'{
                                output.set(x + dx, y + dy, 'O');
                            }
                        }
                    }
                }
            }
        }
    }

    return output;
}

fn day1(contents: String){
    let tiles = parse_image_tiles(contents);
    let (tile_graph, final_tiles) = get_tile_graph(tiles);
    let mut result: i64  = 1;

    for node in tile_graph{
        if node.n_neighbors() == 2{
            let tile_id = final_tiles[node.idx as usize].tile_id;
            println!("Match id: {}", tile_id);
            result *= tile_id as i64;
        }
    }

    println!("Part 1 solution: {}", result);
}

fn day2(contents: String){
    let tiles = parse_image_tiles(contents);
    let (tile_graph, arranged_tiles) = get_tile_graph(tiles);
    let full_image = get_image_from_tile_graph(&tile_graph, &arranged_tiles);
    let image_with_monster = find_monsters(full_image);
    let result = image_with_monster.data.iter().filter(|&&x| x == '#').count();
    println!("Part 2 solution: {}", result);
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
