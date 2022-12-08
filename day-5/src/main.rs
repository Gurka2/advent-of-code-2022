use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const INPUT: &str = "test";
const STACKS: i32 = 9;

// Inputs on index 1,5,9 => index n on (n-1)*4+1?
// Good enough

fn main() {

    // Parse the imput file
    if let Ok(lines) = read_lines(INPUT) {
        for line in lines {
            if let Ok(ip) = line {
                part_1(&ip);
            }
        }
    }
}


fn part_1(input: &str) {
    parse_input(input);
}

fn parse_input(input: &str) {
    for (_i, c) in input.chars().enumerate() {
        println!("{}", c);
    }
}

fn parse_char(input: &char) {

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

