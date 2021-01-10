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

fn rotate(v: &Position, theta: f32) -> Position {
    let vx = v.x as f32;
    let vy = v.y as f32;
    let px = vx * theta.cos() - vy * theta.sin();
    let py = vx * theta.sin() + vy * theta.cos();

    return Position{x: px.round() as i32, y: py.round() as i32};
}

pub fn part1(contents: String) -> i32{
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
    return manhattan_distance(&pose.position, &origin);
}

pub fn part2(contents:String) -> i32{
    let origin = Position{x: 0, y: 0};
    let mut ship_position = origin;
    let mut waypoint_position = Position{x: 10, y: 1};

    for line in contents.lines(){
        let action = &line[0..1];
        let value = line[1..].parse::<i32>().unwrap();

        match action{
            "E" => waypoint_position.x += value,
            "W" => waypoint_position.x -= value,
            "N" => waypoint_position.y += value,
            "S" => waypoint_position.y -= value,
            "L" => {
                waypoint_position = rotate(&waypoint_position, (value as f32).to_radians());
            },
            "R" => {
                waypoint_position = rotate(&waypoint_position, (-value as f32).to_radians());
            }
            "F" => {
                ship_position.x += value * waypoint_position.x;
                ship_position.y += value * waypoint_position.y;
            },
            _ => panic!("Wrong action {}", action),
        };
    }

    let origin = Position{x: 0, y: 0};
    return manhattan_distance(&ship_position, &origin);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_contents() -> String{
        return vec![
            "F10",
            "N3",
            "F7",
            "R90",
            "F11",
        ].join("\n");
    }

    #[test]
    fn test_part1() {
        assert_eq!(25, part1(get_test_contents()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(286, part2(get_test_contents()));
    }
}
