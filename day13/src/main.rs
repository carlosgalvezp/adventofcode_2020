use std::env;
use std::fs;

fn day1(contents: String) {
    let timestamp = contents.lines().nth(0).unwrap().parse::<i32>().unwrap();
    let buses_line = contents.lines().nth(1).unwrap();

    let mut buses = Vec::new();

    for l in buses_line.split(","){
        if l != "x"{
            buses.push(l.parse::<i32>().unwrap());
        }
    }

    let mut earliest_bus_id = 0;
    let mut min_wait_time = i32::MAX;
    for bus_id in buses {
        let next_departure_after_timestamp = ((timestamp as f32 / bus_id as f32).ceil() as i32) * bus_id;
        let wait_time = next_departure_after_timestamp - timestamp;
        if wait_time < min_wait_time{
            earliest_bus_id = bus_id;
            min_wait_time = wait_time;
        }
    }

    println!("Part 1 solution: {}", min_wait_time * earliest_bus_id);
}

fn day2(contents: String) {
    let bus_frequencies_str = contents.lines().nth(1).unwrap().split(",");

    let mut bus_frequencies = Vec::new();
    let mut bus_indices = Vec::new();

    for (i, bus_frequency) in bus_frequencies_str.enumerate() {
        if bus_frequency != "x" {
            bus_frequencies.push(bus_frequency.parse::<i64>().unwrap());
            bus_indices.push(i as i64);
        }
    }

    let mut result = bus_frequencies[0];
    let mut period = bus_frequencies[0];

    for i in 1..bus_frequencies.len(){
        while ((result + bus_indices[i]) % bus_frequencies[i]) != 0{
            result += period;
        }
        period *= bus_frequencies[i];
    }

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
