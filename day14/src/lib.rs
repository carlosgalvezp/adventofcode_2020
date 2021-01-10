use std::collections::HashMap;
use regex::Regex;

pub fn part1(contents: String) -> u64{
    let mut memory = HashMap::new();
    let mut mask_clear: u64 = 0;
    let mut mask_set: u64 = 0;

    let re_mask = Regex::new(r"^mask = ([0X1]{36})$").unwrap();
    let re_memory = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    for line in contents.lines() {
        let re_mask_result = re_mask.captures(&line);
        if re_mask_result.is_some(){
            let mask_str = re_mask_result.unwrap()[1].to_owned();
            let mask_clear_str = mask_str.replace("1", "0").replace("X", "1");
            let mask_set_str = mask_str.replace("X", "0");

            mask_clear = u64::from_str_radix(&mask_clear_str, 2).unwrap();
            mask_set = u64::from_str_radix(&mask_set_str, 2).unwrap();
        }

        let re_memory_result = re_memory.captures(&line);
        if re_memory_result.is_some(){
            let re_memory_result_unwrapped = re_memory_result.unwrap();
            let address = re_memory_result_unwrapped[1].parse::<u64>().unwrap();
            let mut content = re_memory_result_unwrapped[2].parse::<u64>().unwrap();

            content &= mask_clear;
            content |= mask_set;

            memory.insert(address, content);
        }
    }

    let mut result = 0;
    for c in memory.values() {
        result += c;
    }

    return result;
}

pub fn part2(contents: String) -> u64{
    let mut memory = HashMap::new();

    let re_mask = Regex::new(r"^mask = ([0X1]{36})$").unwrap();
    let re_memory = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    let contents_grouped = contents.replace("\nmask", "\n\nmask");
    let contents_grouped = contents_grouped.split("\n\n");

    // Process one mask - memory group at a time
    for group in contents_grouped {
        // Get mask as a string
        let mut it = group.lines().into_iter();
        let mask_str = re_mask.captures(it.next().unwrap()).unwrap()[1].to_owned();

        // Create a vector containing the positions where the mask has an 'X'
        let mut mask_x_positions = Vec::new();
        for (i, c) in mask_str.chars().into_iter().enumerate(){
            if c == 'X'{
                mask_x_positions.push(i);
            }
        }

        // Process memory lines
        for mem_line in it{
            let mem_capture = re_memory.captures(mem_line).unwrap();
            let address = mem_capture[1].to_owned();
            let content = mem_capture[2].parse::<u64>().unwrap();

            let mut address_binary : Vec<char> = format!("{:036b}", address.parse::<u64>().unwrap()).chars().collect();

            // Add 1's from the mask
            for (i, c) in mask_str.chars().into_iter().enumerate(){
                if c == '1'{
                    address_binary[i] = '1';
                }
            }

            // Decode address for all possible combinations
            for i in 0..2i32.pow(mask_x_positions.len() as u32){
                let mut decoded_address = address_binary.clone();

                let mut i_binary : Vec<char> = format!("{:b}", i).chars().collect();
                while i_binary.len() < mask_x_positions.len(){
                    i_binary.insert(0, '0');
                }

                for (j, _) in mask_x_positions.iter().enumerate(){
                    decoded_address[mask_x_positions[j]] = i_binary[j];
                }

                // Convert decoded address to int
                let decoded_address_str: String = decoded_address.iter().collect();
                let decoded_address_u64 = u64::from_str_radix(&decoded_address_str, 2).unwrap();
                memory.insert(decoded_address_u64, content);
            }
        }
    }

    let mut result = 0;
    for c in memory.values() {
        result += c;
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
            "mem[8] = 11",
            "mem[7] = 101",
            "mem[8] = 0",
        ].join("\n");

        assert_eq!(165, part1(contents));
    }

    #[test]
    fn test_part2() {
        let contents = vec![
            "mask = 000000000000000000000000000000X1001X",
            "mem[42] = 100",
            "mask = 00000000000000000000000000000000X0XX",
            "mem[26] = 1",
        ].join("\n");

        assert_eq!(208, part2(contents));
    }
}
