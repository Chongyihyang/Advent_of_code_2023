pub mod aoc12_1{
    use crate::{int, str};
    use crate::python_builtins::builtins::read;
    use itertools::Itertools;

    pub fn main12_1() {
        let contents = read("AOC12.txt");
        let mut counter = 0u32;
        for line in contents.lines() {
            let line: Vec<_> = line.split(' ').collect();
            let (record, spring) = (line[0], line[1]);
            let mut spring: Vec<_> = spring
                .trim().split(',')
                .map(|x| int!(x, u8)).collect();
        }
    }
}

pub mod aoc12_2{

}
