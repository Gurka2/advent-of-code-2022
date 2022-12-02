use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::Ordering;

const INPUT: &str = "input.txt";

fn main() {

    let mut elf_calories: Vec<i32> = [].to_vec();
    let mut current_elf_calorie: i32 = 0;
    let mut top_three_total: i32 = 0;

    // Parse the imput file
    if let Ok(lines) = read_lines(INPUT) {
        for line in lines {
            if let Ok(ip) = line {

                if !ip.is_empty() {

                    // Cast string to int
                    let my_i32 : i32 = ip.parse().unwrap();

                    current_elf_calorie += my_i32;

                }

                // Empty line => New elf time
                else {
                    elf_calories.push(current_elf_calorie);
                    current_elf_calorie = 0;
                }
            }
        }
    }

    // Most calorie carrying elfs first
    let sorted_elfs = sort_vec(elf_calories);

    // Print top three
    for n in 0..3 {
        top_three_total += sorted_elfs[n];
    }

    println!("Top three: {top_three_total}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn sort_vec(mut vector: Vec<i32>) -> Vec<i32> {
    vector.sort_by(|a, b| {
        if a < b {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    return vector;
}
