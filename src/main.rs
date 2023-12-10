use crate::AOC1::aoc1_2::main1_2;
use crate::AOC1::aoc_1_1::main1_1;
use crate::AOC2::aoc2_2::main2_2;
use crate::AOC9::aoc9_1::main9_1;
use crate::AOC9::aoc9_2::main9_2;

mod AOC1;
mod AOC2;
mod AOC9;
mod python_builtins;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        main2_2()
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
