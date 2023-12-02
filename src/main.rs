mod solution_1;
mod solution_2;

use std::time::SystemTime;

fn main() {
    let now = SystemTime::now();
    println!("{}", solution_2::main());
    println!("{:?}", now.elapsed().unwrap());
}
