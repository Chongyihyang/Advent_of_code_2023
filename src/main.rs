use crate::AOC3::aoc3_2::main3_2;

mod python_builtins;
mod AOC3;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    main3_2();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}