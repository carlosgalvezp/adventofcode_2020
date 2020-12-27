use std::env;
use std::fs;

struct Position{
    x: i32,
    y: i32,
}

struct Pose{
    position: Position,
    orientation: f32,
}

fn manhattan_distance(a: &Position, b: &Position) -> i32{
    return (a.x - b.x).abs() + (a.y - b.y).abs();
}

fn day1(contents: String){
    let origin = Position{x: 0, y: 0};
    let mut pose = Pose{position: origin, orientation: 0f32};

    for line in contents.lines(){
        let action = &line[0..1];
        let value = line[1..].parse::<i32>().unwrap();

        match action{
            "E" => pose.position.x += value,
            "W" => pose.position.x -= value,
            "N" => pose.position.y += value,
            "S" => pose.position.y -= value,
            "L" => pose.orientation += (value as f32).to_radians(),
            "R" => pose.orientation -= (value as f32).to_radians(),
            "F" => {
                pose.position.x += ((value as f32) * pose.orientation.cos()).round() as i32;
                pose.position.y += ((value as f32) * pose.orientation.sin()).round() as i32;
            },
            _ => panic!("Wrong action {}", action),
        };
    }

    let origin = Position{x: 0, y: 0};
    println!("Ship manhattan distance: {}", manhattan_distance(&pose.position, &origin));
}

fn day2(contents:String){

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
