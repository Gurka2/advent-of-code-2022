use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

const INPUT: &str = "input.txt";

// A = X = Rock
// B = Y = Paper
// C = Z = Scissors
// A < Y = B < Z = C < X = A

fn main() {

    let mut score: u32 = 0;

    let mut value_points: HashMap<char, u32> = HashMap::new();
    value_points.insert('X', 1);
    value_points.insert('Y', 2);
    value_points.insert('Z', 3);

    // Parse the imput file
    if let Ok(lines) = read_lines(INPUT) {
        for line in lines {
            if let Ok(ip) = line {

                /* Part 1
                let my_hand: char = get_string_char(&ip, 2);
                let other_hand: char = get_string_char(&ip, 0);
                score += get_match_points(my_hand, other_hand);
                score += value_points.get(&my_hand).unwrap();
                */

                let outcome: char = get_string_char(&ip, 2);
                let other_hand: char = get_string_char(&ip, 0);
                let my_hand: char = get_my_hand(other_hand, outcome);
                score += get_match_points(my_hand, other_hand);
                score += value_points.get(&my_hand).unwrap();

            }
        }
    }
    println!("Score: {}", score);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Part 2

fn get_my_hand(other_hand: char, outcome: char) -> char {

    // Draw
    if outcome == 'Y' {
        return get_even_hand(other_hand);

    // Lose
    } else if outcome == 'X'{
        return get_losing_hand(other_hand);
        
    }

    // Win
    return get_winning_hand(other_hand);
}

fn get_even_hand(other_hand: char) -> char {
    if other_hand == 'A' {
        return 'X';
    } else if other_hand == 'B' {
        return 'Y';
    }
    return 'Z';
}

fn get_losing_hand(other_hand: char) -> char {
    if other_hand == 'A' {
        return 'Z';
    } else if other_hand == 'B' {
        return 'X';
    }
    return 'Y';
}

fn get_winning_hand(other_hand: char) -> char {
    if other_hand == 'A' {
        return 'Y';
    } else if other_hand == 'B' {
        return 'Z';
    }
    return 'X';
}

// Part 1
fn get_string_char(string: &str, index: usize) -> char {
    let ret: char = string.chars().nth(index).unwrap();
    return ret;
}

fn get_match_points(my_hand: char, other_hand: char) -> u32 {
    let my_hand_as_u32 = my_hand as u32;
    let other_hand_as_u32 = other_hand as u32;
    let res = (my_hand_as_u32 - other_hand_as_u32) % 3;

    if res == 0 {
        return 6;
    } else if res == 1 {
        return 0;
    }
    return 3;
}
