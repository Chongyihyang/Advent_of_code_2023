pub mod aoc15_1 {
    use crate::python_builtins::builtins::{ord, read};
    pub fn hash(input: &str) -> u32 {
        let mut total = 0u32;
        for character in input.chars(){
            total += ord(character);
            total *= 17;
            total = total % 256;
            // println!("{}", total);
        }
        total
    }

    pub fn main15_1() {
        let contents = read("AOC15.txt");
        let mut count = 0u32;
        let total: u32 = contents.split(',')
            .map(|x| {count += 1; hash(x)}).sum();
        println!("{total}, {count}");
    }
}

pub mod aoc15_2 {
    use itertools::Itertools;
    use crate::AOC15::aoc15_1::hash;
    use crate::{int, str};
    use crate::python_builtins::builtins::read;



    pub fn main15_2() {
        let contents = read("AOC15.txt");
        let mut counter = 0u32;
        let mut boxes: [Vec<(String, u8)>; 256] = vec![Vec::new(); 256].try_into().expect("static");
        let items: Vec<_> = contents.split(',').collect();
        for item in items{
            if item.chars().contains(&'='){
                let item: Vec<_> = item.split('=').collect();
                let (item, focal, hash_) = (item[0].to_string(), int!(item[1], u8), hash(item[0]));
                let mut box_ = &boxes[hash_ as usize];
                if let Some(x) = box_.iter().map(|x| x.0.clone()).position(|x| x == str!(item)){
                    boxes[hash_ as usize][x] = (item, focal)
                } else {
                    boxes[hash_ as usize].push((item, focal))
                }
            } else if item.chars().contains(&'-') {
                let item: Vec<_> = item.split('-').collect();
                let (item, hash_) = (item[0].to_string(), hash(item[0]));
                let mut box_ = &boxes[hash_ as usize];
                if let Some(x) = box_.iter().map(|x| x.0.clone()).position(|x| x == str!(item)){
                    boxes[hash_ as usize].remove(x);
                }
            }
        }
        for (i, box_) in boxes.iter().enumerate() {
            let i = i as u32 + 1;
            for (j, &ref slot) in box_.iter().enumerate() {
                let j = j as u32 + 1;
                let u = i * j * slot.1 as u32;
                //println!("{i}, {j}, {}, {u}", slot.1);
                counter += u;
            }
        }
        println!("{counter}");
    }
}