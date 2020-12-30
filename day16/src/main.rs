use std::env;
use std::collections::HashMap;
use std::fs;
use regex::Regex;

#[derive(Clone)]
struct FieldRange{
    min: i32,
    max: i32,
}

impl FieldRange{
    fn contains(&self, value: i32) -> bool {
        return value >= self.min && value <= self.max;
    }
}

#[derive(Clone)]
struct Field{
    name: String,
    ranges: Vec<FieldRange>,
}

type Ticket = Vec<i32>;

fn get_fields(fields_contents: &str) -> Vec<Field>{
    let re_field = Regex::new(r"^([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();

    let mut output = Vec::new();

    for line in fields_contents.lines(){
        let re_result = re_field.captures(&line).unwrap();

        let name = re_result[1].to_owned();
        let min_range_1 = re_result[2].parse::<i32>().unwrap();
        let max_range_1 = re_result[3].parse::<i32>().unwrap();
        let min_range_2 = re_result[4].parse::<i32>().unwrap();
        let max_range_2 = re_result[5].parse::<i32>().unwrap();

        let range_1 = FieldRange{min: min_range_1, max: max_range_1};
        let range_2 = FieldRange{min: min_range_2, max: max_range_2};

        output.push(Field{name, ranges: vec![range_1, range_2]});
    }

    return output;
}

fn get_ticket(ticket_contents: &str) -> Ticket{
    return ticket_contents.split(",")
                          .map(|x| x.parse::<i32>().unwrap())
                          .collect();

}

fn get_my_ticket(my_ticket_contents: &str) -> Ticket{
    return get_ticket(my_ticket_contents.lines().nth(1).unwrap());
}

fn get_nearby_tickets(nearby_tickets_contents: &str) -> Vec<Ticket>{
    let mut output = Vec::new();

    let mut it = nearby_tickets_contents.lines().into_iter();
    it.next(); // Skip title line

    for line in it{
        output.push(get_ticket(line));
    }

    return output;
}

fn day1(contents: String) {
    let content_groups : Vec<&str> = contents.split("\n\n").collect();

    let fields = get_fields(content_groups[0]);
    let nearby_tickets = get_nearby_tickets(content_groups[2]);

    let mut output = 0;

    for ticket in nearby_tickets{
        for value in ticket{
            let mut is_valid = false;
            for field in &fields{
                if field.ranges[0].contains(value) || field.ranges[1].contains(value) {
                    is_valid = true;
                    break;
                }
            }

            if !is_valid{
                output += value;
            }
        }
    }

    println!("Part 1 solution: {}", output);
}

fn day2(contents: String) {
    let content_groups : Vec<&str> = contents.split("\n\n").collect();

    let fields = get_fields(content_groups[0]);
    let my_ticket = get_my_ticket(content_groups[1]);
    let nearby_tickets = get_nearby_tickets(content_groups[2]);

    // Keep only the valid tickets
    let mut valid_tickets = Vec::new();

    for ticket in &nearby_tickets{
        let mut is_ticket_valid = true;
        for value in ticket{
            let mut is_value_valid = false;
            for field in &fields{
                if field.ranges[0].contains(*value) || field.ranges[1].contains(*value) {
                    is_value_valid = true;
                    break;
                }
            }

            if !is_value_valid{
                is_ticket_valid = false;
                break;
            }
        }
        if is_ticket_valid{
            valid_tickets.push(ticket.clone());
        }
    }

    // Find order
    let mut fields_to_associate : Vec<Field> = fields.clone();
    let mut associations : HashMap<String, i32> = HashMap::new();

    while !fields_to_associate.is_empty() {
        for (i, _) in valid_tickets[0].iter().enumerate(){
            let mut fields_that_match = Vec::new();

            for field in &fields_to_associate{
                let mut all_tickets_match_field = true;
                for ticket in &valid_tickets{
                    if !field.ranges[0].contains(ticket[i]) && !field.ranges[1].contains(ticket[i]){
                        all_tickets_match_field = false;
                        break;
                    }
                }

                if all_tickets_match_field {
                    fields_that_match.push(field);
                }
            }

            // If we have a unique match, store the association
            if fields_that_match.len() == 1{
                let field_name = fields_that_match.first().unwrap().name.clone();
                associations.insert(field_name.clone(), i as i32);
                fields_to_associate.remove(fields_to_associate.iter().position(|x| x.name == field_name).unwrap());
            }
        }
    }

    // Get Part 2 solution
    let mut result: i64 = 1;
    for (association, idx) in associations{
        if association.contains("departure"){
            result *= my_ticket[idx as usize] as i64;
        }
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
