/* static mut MAX_RED: u8 = 0;
static mut MAX_GREEN: u8 = 0;
static mut MAX_BLUE: u8 = 0;*/

pub fn main_2() -> u32 {
    let input_file = include_str!("problem_input_2.txt");
    let mut total: u32 = 0;

    for line in (input_file).lines() {
        total += collapse(line);
    }

    return total;
}

fn collapse(input: &str) -> u32 {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;
    let mut total: u32 = 0;
    let mut skips: u8 = 0;
    for c in input.chars() {
        total = (total << 4) | dict(c);
        total = (total << 16) >> 16; // We check in blocks of 4
        check_current_total(total, &mut min_red, &mut min_green, &mut min_blue);
    }
    //println!("RED:{}, GREEN:{}, BLUE:{}", min_red, min_green, min_blue);
    return min_red * min_green * min_blue;
}

fn check_current_total(input: u32, min_red: &mut u32, min_green: &mut u32, min_blue: &mut u32) {
    let mut input_copy = input;
    let mut total: u32 = 0;
    let mut nums = false;
    let mut color = 3; // 0 = Red, 1 = Green, 2 = Blue | 3 = None
    //println!("Imput: {}", input);
    while input_copy != 0 { // 0001 011(0) 1 111 1 0010
        let mut ran = false;
        match (input_copy << 28) >> 28 {
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
            10 => {color = 0},
            11 => {color = 1},
            12 => {color = 2},
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
    //return total <= compare_to;
    //println!("Color: {}", color);
    match color {
        0 => if total > *min_red {*min_red = total;},
        1 => if total > *min_green {*min_green = total;},
        2 => if total > *min_blue {*min_blue = total;},
        _ => return,
    }
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