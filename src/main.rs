use crate::AOC15::aoc15_1::{hash, main15_1};
use crate::AOC15::aoc15_2::main15_2;

mod python_builtins;
mod AOC15;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    main15_2();
    println!("{}", hash("qp"));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}