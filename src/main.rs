use crate::AOC19::aoc19_1::main19_1;

mod AOC19;
mod python_builtins;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    main19_1();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
