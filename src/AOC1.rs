pub mod aoc_1_1{
    use crate::int;
    use crate::python_builtins::builtins::read;

    pub fn main1_1() {
        let contents = read("AOC1.txt");
        let mut counter = 0u32;
        for x in contents.lines() {
            let tmp: Vec<_> = x.chars()
                .filter(|y| y.is_digit(10))
                .collect();
            if tmp.len() == 1{
                counter += int!(tmp[0].to_string(), u32) * 10 + int!(tmp[0].to_string(), u32)
            } else {
                counter += int!(tmp[0].to_string(), u32) * 10 + int!(tmp[tmp.len()-1].to_string(), u32)
            }

        };
        println!("{counter}");
    }
}

pub mod aoc1_2{
    use crate::{int, str};
    use crate::python_builtins::builtins::read;

    pub fn check_start(input: &String) -> u8 {
        let words = ["one", "two", "three", "four", "five",
            "six", "seven", "eight", "nine"];
        let n = input.len() as u32 - 1;
        let mut tmp_counter = 0u32;
        while tmp_counter < input.len() as u32 {
            let slice= &input[tmp_counter as usize..n as usize];
            let first = slice.chars().nth(0).unwrap_or('0');
            if first.is_digit(10) {
                return int!(input.chars().nth(tmp_counter as usize).unwrap().to_string(), u8);
            } else {
                for (i, &word) in words.iter().enumerate(){
                    if slice.starts_with(word){
                        return i as u8 + 1
                    }
                    let word = reverse_string(word.to_string());
                    if slice.starts_with(&word){
                        return i as u8 + 1
                    }
                }
                tmp_counter += 1
            }
        }
        0
    }

    pub fn reverse_string(input: String) -> String{
        let mut new = str!();
        for char in input.chars().rev(){
            new.push(char)
        }
        new
    }

    pub fn main1_2() {
        let contents = read("AOC1.txt");
        let mut counter = 0u32;
        for x in contents.lines() {
            let x = x.to_string();
            let first = check_start(&x);
            let x = reverse_string(x);
            let last = check_start(&x);
            if first == 0{
                counter += last as u32 * 10 + last as u32
            }
            if first == last{
                counter += first as u32 * 10 + first as u32
            } else {
                counter += first as u32 * 10 + last as u32
            }
        };
        println!("{counter}");
    }
}