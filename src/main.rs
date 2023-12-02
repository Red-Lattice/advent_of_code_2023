mod solution_1;

use std::time::SystemTime;

fn main() {
    let now = SystemTime::now();
    solution_1::main();
    println!("{:?}", now.elapsed().unwrap());
}
