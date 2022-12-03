use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

const INPUT: &str = "input";
const UPPER: u32 = 38;
const LOWER: u32 = 96;

fn main() {

    let mut sum_p1: u32 = 0;

    // Parse the imput file
    if let Ok(lines) = read_lines(INPUT) {
        for line in lines {
            if let Ok(ip) = line {
                sum_p1 += part_1(&ip);
            }
        }
    }

    println!("Part 1: {}", sum_p1);
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn part_1(string: &str) -> u32 {

    let mut priority: u32 = 0;
    let result = get_duplicate_char(string);

    if result.is_some() {
        priority = result.unwrap();
        
        if priority > 96 {
            priority -= LOWER;
        } 

        else {
            priority -= UPPER;
        }

    }
    
    return priority;
}


fn get_duplicate_char(string: &str) -> Option<u32> {

    let mut items: HashMap<char, u32> = HashMap::new();
    let mut result: Option<u32> = None;

    let length: usize = (string.len())/2;

    for (i, c) in string.chars().enumerate() {

        // Iterating over first half
        if i < length {
            items.insert(c, c as u32);
        } 

        // Look for chars in the hashmap (inte drogen) 
        else {

            result = items.get(&c).copied();
            if result.is_some() {
                return result;
            }
        }
    }
    return result;

}
