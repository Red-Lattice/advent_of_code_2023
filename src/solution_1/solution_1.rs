use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn main() -> u64 {
    let input_file = File::open("problem_1_input.txt").unwrap();

    let mut lines_vec: Vec<String> = vec![];
    let mut finish_vec: Vec<u32> = vec![];

    // TODO: Optimize this down to calculate each line live instead of reading and then calculating. (currently O(2n) instead of O(n))
    for line in BufReader::new(input_file).lines() {
        lines_vec.push(line.unwrap().parse().unwrap());
    }
    for line in lines_vec {
        finish_vec.push(collapse_beginning(&line));
    }

    return 0;
}

fn collapse_beginning(input: &str) -> u32 {
    let mut total: u32 = 0;
    for (i, c) in input.chars().enumerate() {
        if let Some(c) = c.to_digit(10) {return c;}
        total = (total << 4) | dict(c);
        if check_current_total(total) != 0 {return check_current_total(total);} 
        total = (total << 12) >> 12; // 00000000000010001000100010001000
    }
    return 0;
}

fn collapse_end(input: &str) -> u32 {
    let mut total: u32 = 0;
    for (i, c) in input.chars().enumerate() {
        if let Some(c) = c.to_digit(10) {return c;}
        total = (total << 4) | dict(c);
        if check_current_total_end(total) != 0 {return check_current_total(total);} 
        total = (total << 12) >> 12; // 00000000000010001000100010001000
    }
    return 0;
}

fn check_current_total(val: u32) -> u32
{
    match val {
        673809 => 30,
        597014 => 70,
        86858 => 80,
        _ => match (val << 16) >> 16 {
            10168 => 40,
            9665 => 50,
            25953 => 90,
            _ => match (val << 20) >> 20 {
                1889 => 10,
                2775 => 20,
                2398 => 60,
                _ => 0,
            },
        }
    }
    /*
    one = 0111 0110 0001 = 1889
    two = 1010 1101 0111 = 2775
    three = 1010 0100 1000 0001 0001 = 673809
    four = 0010 0111 1011 1000 = 10168
    five = 0010 0101 1100 0001 = 9665
    six = 1001 0101 1110 = 2398
    seven = 1001 0001 1100 0001 0110 = 597014
    eight = 0001 0101 0011 0100 1010 = 86858
    nine = 0110 0101 0110 0001 = 25953
     */
}

fn check_current_total_end(val: u32) -> u32
{
    match val {
        673809 => 3,
        597014 => 7,
        86858 => 8,
        _ => match (val << 16) >> 16 {
            10168 => 4,
            9665 => 5,
            25953 => 9,
            _ => match (val << 20) >> 20 {
                1889 => 1,
                2775 => 2,
                2398 => 6,
                _ => 0,
            },
        }
    }
    /*
    one (eno) = (0001 0110 0111) = 1889
    two (owt) = (0111 1101 1010) = 2775
    three (eerht) = (0001 0001 1000 0100 1010)= 673809
    four (ruof) = (1000 1011 0111 0010) = 10168
    five (evif) = (0001 1100 0101 0010) = 9665
    six (xis) = (1110 0101 1001) = 2398
    seven (neves) = (0110 0001 1100 0001 1001) = 597014
    eight (thgie) = (1010 0100 0011 0101 0001) = 86858
    nine (enin) = (0001 0110 0101 0110) = 25953
     */
}

fn dict(input: char) -> u32 {
    match input {
        'e' => 1, // 0001
        'f' => 2, // 0010
        'g' => 3, // 0011
        'h' => 4, // 0100
        'i' => 5, // 0101
        'n' => 6, // 0110
        'o' => 7, // 0111
        'r' => 8, // 1000
        's' => 9, // 1001
        't' => 10, // 1010
        'u' => 11, // 1011
        'v' => 12, // 1100
        'w' => 13, // 1101
        'x' => 14, // 1110
        _ => 0,
    }
}
