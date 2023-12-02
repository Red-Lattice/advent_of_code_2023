mod solution_1;

use std::time::SystemTime;

fn main() {
    let now = SystemTime::now();
    println!("{}", solution_1::main());
    println!("{:?}", now.elapsed().unwrap());
}
