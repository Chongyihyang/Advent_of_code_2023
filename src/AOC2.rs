pub mod aoc2_1 {
    use crate::python_builtins::builtins::*;
    use crate::str;

    pub fn main2_1() {
        let contents = read("AOC2.txt");
        let limit: [u32; 3] = [12, 13, 14];
        let number_of_lines = contents.lines().count();
        let mut games = vec![];
        for i in (1..=number_of_lines) {
            games.push(i);
        }
        for (j, line) in contents.lines().enumerate() {
            let line = line.split(": ").nth(1).unwrap();
            let mut tmp = str!();
            let mut curr_collect = false;
            for (i, item) in line.chars().enumerate() {
                if item.is_digit(10) && curr_collect == false {
                    curr_collect = true;
                    tmp.push(item);
                } else if item.is_digit(10) && curr_collect == true {
                    tmp.push(item);
                } else if item == ' ' && curr_collect == true {
                    curr_collect = false;
                    let mut color = 0usize;
                    if line[i + 1..].starts_with("red") {
                        color = 0;
                    } else if line[i + 1..].starts_with("green") {
                        color = 1;
                    } else if line[i + 1..].starts_with("blue") {
                        color = 2;
                    }
                    let tmp_num: u32 = tmp.parse().unwrap();
                    if tmp_num > limit[color] {
                        let pos = games.iter().position(|&x| x == j + 1).unwrap();
                        games.remove(pos);
                        break;
                    }
                    tmp = str!();
                }
            }
        }
        println!("{}", games.iter().sum::<usize>());
    }
}

pub mod aoc2_2 {

    use crate::python_builtins::builtins::*;
    use crate::str;
    pub fn main2_2() {
        let contents = read("AOC2.txt");
        let mut total = 0u32;
        for (j, line) in contents.lines().enumerate() {
            let line = line.split(": ").nth(1).unwrap();
            let mut tmp = str!();
            let mut curr_collect = false;
            let mut limit: [u32; 3] = [0, 0, 0];
            for (i, item) in line.chars().enumerate() {
                if item.is_digit(10) && curr_collect == false {
                    curr_collect = true;
                    tmp.push(item);
                } else if item.is_digit(10) && curr_collect == true {
                    tmp.push(item);
                } else if item == ' ' && curr_collect == true {
                    curr_collect = false;
                    let mut color = 0usize;
                    if line[i + 1..].starts_with("red") {
                        color = 0;
                    } else if line[i + 1..].starts_with("green") {
                        color = 1;
                    } else if line[i + 1..].starts_with("blue") {
                        color = 2;
                    }
                    let tmp_num: u32 = tmp.parse().unwrap();
                    if tmp_num > limit[color] {
                        limit[color] = tmp_num;
                    }
                    tmp = str!();
                }
            }
            total += limit.iter().product::<u32>();
        }
        println!("{}", total)
    }
}
