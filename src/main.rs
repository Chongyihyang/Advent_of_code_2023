use crate::AOC10::aoc10_1::main10_1;
use crate::AOC10::aoc10_2::main10_2;

mod AOC1;
mod AOC10;
mod AOC2;
mod AOC9;
mod python_builtins;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        main10_2();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
