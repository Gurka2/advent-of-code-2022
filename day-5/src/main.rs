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
    for i in 0..STACKS {
        println!("{}",i*4+1); 
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
