use crate::AOC15::aoc15_1::{hash, main15_1};
use crate::AOC15::aoc15_2::main15_2;
use crate::AOC16::aoc16_1::main16_1;
use crate::AOC16::aoc16_2::main16_2;

mod python_builtins;
mod AOC15;
mod AOC16;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    main16_2();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}