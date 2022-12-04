use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

const INPUT: &str = "input";

fn main() {

    let mut sum_p1: i32 = 0;
    let mut sum_p2: i32 = 0;

    // Parse the imput file
    if let Ok(lines) = read_lines(INPUT) {
        for line in lines {
            if let Ok(ip) = line {
                sum_p1 += part_1(&ip);
                sum_p2 += part_2(&ip);
            }
        }
    }
    println!("Part 1: {}", sum_p1);
    println!("Part 2: {}", sum_p2);
}

fn part_2(line: &str) -> i32 {

    let range_values = get_range_values(line);

    if range_values.0 < range_values.2 && range_values.1 < range_values.2 {
        return 0;
    }

    if range_values.3 < range_values.0 && range_values.2 < range_values.0 {
        return 0;
    }

    return 1;
}

fn part_1(line: &str) -> i32 {

    let range_values = get_range_values(line);

    if range_values.0 - range_values.2 <= 0 && range_values.1 - range_values.3 >= 0 {
        return 1;
    } 

    else if range_values.2 - range_values.0 <= 0 && range_values.3 - range_values.1 >= 0 {
        return 1;
    }

    return 0;

}

fn get_range_values(line: &str) -> (i32, i32, i32, i32) {

    let ranges = line.rsplit_once(",");
    let (range1, range2) = ranges.unwrap();

    let values_r1 = range1.rsplit_once("-");
    let (range1_min, range1_max) = values_r1.unwrap();

    let values_r2 = range2.rsplit_once("-");
    let (range2_min, range2_max) = values_r2.unwrap();

    return (FromStr::from_str(range1_min).unwrap(),
            FromStr::from_str(range1_max).unwrap(),
            FromStr::from_str(range2_min).unwrap(),
            FromStr::from_str(range2_max).unwrap());

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
