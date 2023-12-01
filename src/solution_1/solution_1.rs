use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn main() -> u64 {
    let input_file = File::open("problem_1_input.txt").unwrap();

    let mut lines_vec: Vec<String> = vec![];

    // TODO: Optimize this down to calculate each line live instead of reading and then calculating. (currently O(2n) instead of O(n))
    for line in BufReader::new(input_file).lines() {
        lines_vec.push(line.unwrap().parse().unwrap());
    }

    return 0;
}

fn collapse(input: &str) -> u128 {
    let mut total: u64 = 0;
    for (i, c) in input.chars().enumerate() {
        // do something with character `c` and index `i`
    }

    return 0;
}

fn dict(input: char) -> u8 {
    match input {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' => 10,
        'b' => 11,
        'c' => 12,
        'd' => 13,
        'e' => 14,
        'f' => 15,
        'g' => 16,
        'h' => 17,
        'i' => 18,
        'j' => 19,
        'k' => 20,
        'l' => 21,
        'm' => 22,
        'n' => 23,
        'o' => 24,
        'p' => 25,
        'q' => 26,
        'r' => 27,
        's' => 28,
        't' => 29,
        'u' => 30,
        'v' => 31,
        'w' => 32,
        'x' => 33,
        'y' => 34,
        'z' => 35,
        _ => 36,
    }
}
