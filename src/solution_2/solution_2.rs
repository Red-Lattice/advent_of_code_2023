const MAX_RED: u8 = 12;
const MAX_GREEN: u8 = 13;
const MAX_BLUE: u8 = 14;

pub fn main() -> u32 {
    let input_file = include_str!("problem_input_2.txt");
    let mut total = 0;

    for (line_number, line) in (input_file).lines().enumerate() {
        if collapse(line) {
            total += line_number + 1;
        }
    }

    return total.try_into().unwrap();
}

fn collapse(input: &str) -> bool {
    let mut total: u32 = 0;
    for c in input.chars() {
        total = (total << 4) | dict(c);
        total = (total << 16) >> 16; // We check in blocks of 4
        if total < 32768 
        {
            let check = check_current_total(total);
            if !check {return false;}
        }
    }
    return true;
}

fn check_current_total(input: u32) -> bool {
    let mut input_copy = input;
    let mut total = 0;
    let mut nums = false;
    let mut compare_to = 255;

    while input_copy != 0 { // 0001 011(0) 1 111 1 0010
        let mut ran = false;
        let comp = input_copy;
        match (comp << 28) >> 28 {
            0 => {nums = true;}, // No leading zeros
            1 => {if nums {total += 10;} else {total += 1;}; nums = true;},
            2 => {if nums {total += 20;} else {total += 2;}; nums = true;},
            3 => {total += 3; nums = true;},
            4 => {total += 4; nums = true;},
            5 => {total += 5; nums = true;},
            6 => {total += 6; nums = true;},
            7 => {total += 7; nums = true;},
            8 => {total += 8; nums = true;},
            9 => {total += 9; nums = true;}, 
            10 => compare_to = MAX_RED,
            11 => compare_to = MAX_GREEN,
            12 => compare_to = MAX_BLUE,
            _ => {input_copy = input_copy >> 4; ran = true},
        }
        //println!("bit shifter: {}", (comp << 28) >> 28);
        //println!("input-copy: {}", input_copy);
        //println!("nums:{}", nums);
        if !ran {input_copy = input_copy >> 4;}
        //println!("input-copy shifted: {}", input_copy);
    }
    //println!("BEING COMPARED TO: {}", compare_to);
    //println!("TOTAL: {}", total);
    return total <= compare_to;
}

fn dict(input: char) -> u32 {
    match input {
        '0' => 0, // 0000
        '1' => 1, // 0001
        '2' => 2, // 0010
        '3' => 3, // 0011
        '4' => 4, // 0100
        '5' => 5, // 0101
        '6' => 6, // 0110
        '7' => 7, // 0111
        '8' => 8, // 1000
        '9' => 9, // 1001
        'r' => 10, // 1010
        'g' => 11, // 1011
        'b' => 12, // 1100
        _ => 15, // 1111
    }
}