use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

const INPUT: &str = "input";
const UPPER: u32 = 38;
const LOWER: u32 = 96;

fn main() {

    let mut sum_p1: u32 = 0;
    let mut sum_p2: u32 = 0;

    let mut counter: i32 = 0;

    let mut string1: String = "".to_string();
    let mut string2: String = "".to_string();
    let mut string3: String = "".to_string();

    // Parse the imput file
    if let Ok(lines) = read_lines(INPUT) {
        for line in lines {
            if let Ok(ip) = line {

                sum_p1 += part_1(&ip);

                match counter {
                    0 => string1 = ip,
                    1 => string2 = ip,
                    2 => string3 = ip,
                    _ => println!("Something is wrong")
                }
                counter += 1;
                if counter == 3 {
                    sum_p2 += part_2(&string1, &string2, &string3);
                    counter = 0;
                }
            }
        }
    }

    println!("Part 1: {}", sum_p1);
    println!("Part 2: {}", sum_p2);
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part_2(string1: &str, string2: &str, string3: &str) -> u32 {

    let duplicates: String = get_duplicate_char(string1, string2);
    let duplicates_final: String = get_duplicate_char(&duplicates, string3);
    return get_priority(duplicates_final);

}


fn part_1(string: &str) -> u32 {

    let len = string.len();
    let string1 = &string[0..len/2];
    let string2 = &string[len/2..];

    let result = get_duplicate_char(string1, string2);
    return get_priority(result);
}

fn get_priority(s: String) -> u32 {

    let mut priority = s.chars().nth(0).unwrap() as u32;
    
    if priority > 96 {
        priority -= LOWER;
    } 

    else {
        priority -= UPPER;
    }

    return priority;
}


fn get_duplicate_char(string1: &str, string2: &str) -> String {
    // Returns all unique duplicates in the strings

    let mut duplicates = String::from("");
    let mut items: HashMap<char, u32> = HashMap::new();
    let mut result: Option<u32>;

    for (_i, c) in string1.chars().enumerate() {

        items.insert(c, c as u32);
    }

    // Look for chars in the hashmap (inte drogen) 
    for (_i, c) in string2.chars().enumerate() {

        result = items.get(&c).copied();

        if result.is_some() && !duplicates.contains(*&c) {
            duplicates.push(c);
        }
    }

    return duplicates;

}
