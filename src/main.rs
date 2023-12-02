mod solution_1;
mod solution_2;

use std::time::SystemTime;

fn main() {
    let now = SystemTime::now();
    println!("{}", solution_2::solution_2_part_2::main_2());
    println!("{:?}", now.elapsed().unwrap());
}
