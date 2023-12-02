pub fn main() -> u32 {
    let input_file = include_str!("problem_1_input.txt");
    let mut total = 0;

    for line in (input_file).lines() {
        total += collapse_beginning(&line) + collapse_end(&line);
    }

    return total;
}

fn collapse_beginning(input: &str) -> u32 {
    let mut total: u32 = 0;
    for c in input.chars() {
        if let Some(c) = c.to_digit(10) {return c * 10;}
        total = (total << 4) | dict(c);
        total = (total << 12) >> 12;
        let check = check_current_total(total);
        if check != 0 {return check;} 
    }
    return 0;
}

fn collapse_end(input: &str) -> u32 {
    let mut total: u32 = 0;
    for c in input.chars().rev() {
        if let Some(c) = c.to_digit(10) {return c;}
        total = (total << 4) | dict(c);
        total = (total << 12) >> 12;
        let check = check_current_total_end(total);
        if check != 0 {return check;} 
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
        71754 => 3,
        400409 => 7,
        672593 => 8,
        _ => match (val << 16) >> 16 {
            35698 => 4,
            7250 => 5,
            5718 => 9,
            _ => match (val << 20) >> 20 {
                359 => 1,
                2010 => 2,
                3673 => 6,
                _ => 0,
            },
        }
    }
    /*
    one (eno) = (0001 0110 0111) = 359
    two (owt) = (0111 1101 1010) = 2010
    three (eerht) = (0001 0001 1000 0100 1010)= 71754
    four (ruof) = (1000 1011 0111 0010) = 35698
    five (evif) = (0001 1100 0101 0010) = 7250
    six (xis) = (1110 0101 1001) = 3673
    seven (neves) = (0110 0001 1100 0001 1001) = 400409
    eight (thgie) = (1010 0100 0011 0101 0001) = 672593
    nine (enin) = (0001 0110 0101 0110) = 5718
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
