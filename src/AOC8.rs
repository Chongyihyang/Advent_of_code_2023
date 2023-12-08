pub mod aoc8_1{
    use crate::python_builtins::builtins::{clear_line, read};
    use std::collections::HashMap;
    use crate::str;

    pub fn mapping_tmp(input: String) -> (String, HashMap<String, (String, String)>){
        let input: Vec<_> = input.lines().filter(|&x| !x.is_empty()).collect();
        let steps = input[0].to_string();
        let mut hashmap = HashMap::new();
        //println!("{:?}", input);
        for &x in input[1..].iter() {
            let i:Vec<_> = x.split(" = ").collect();
            let location = i[0].to_string();
            let j: Vec<_> = i[1]
                .split(", ")
                .map(|x| x.trim_matches('(').trim_matches(')').to_string())
                .collect();
            hashmap.insert(location, (j[0].to_string(), j[1].to_string()));
            }
        (steps, hashmap)
    }

    pub fn main8_1(mut current: String, file: &str) -> u32{
        let mut counter = 0u32;
        let contents = read(file);
        let (steps, mappings) = mapping_tmp(contents);
        let mut steps_pointer = 0u32;
        loop{
            if current.ends_with("Z"){
                //println!("{counter}");
                break;
            }
            let next_pos = mappings.get(&current).unwrap();
            match steps.as_bytes().get(steps_pointer as usize).unwrap(){
                b'L' => {
                    current = next_pos.clone().0;
                },
                b'R' => {
                    current = next_pos.clone().1;
                },
                _ => {}
            }
            counter += 1;
            steps_pointer = (steps_pointer + 1) % (steps.len() as u32);
        }
        counter
    }
}

#[cfg(test)]
pub mod test{
    use crate::AOC8::aoc8_1::main8_1;

    #[test]
    fn aoc8_works(){
        //assert_eq!(main8_1(String::from("AAA"), "AOC8.txt"), 6);
    }
}

pub mod aoc8_2{
    use crate::AOC8::aoc8_1::main8_1;
    use crate::str;
    use num::integer::lcm;

    pub fn main8_2() -> u64{
        let lst = ["AAA", "DVA", "MPA", "TDA", "FJA", "XPA"];
        let mut number = 1u64;
        for &item in lst.iter(){
            println!("{}", main8_1(str!(item), "AOC8.txt"));
            number = lcm(main8_1(str!(item), "AOC8.txt") as u64, number);
        }
        number
    }
}