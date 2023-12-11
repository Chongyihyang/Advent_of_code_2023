use crate::AOC11::aoc11_1::main11_1;
use crate::AOC11::aoc11_2::main11_2;

mod python_builtins;
mod AOC11;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    main11_1();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}