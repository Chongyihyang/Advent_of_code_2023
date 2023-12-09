pub mod aoc9_1{
    use crate::int;
    use crate::python_builtins::builtins::read;

    pub fn check_all_0(input: &Vec<i32>) -> bool {
        input.iter().all(|x| *x == 0)
    }

    pub fn generate_differences(input: &Vec<i32>) -> Vec<i32> {
        let mut tmp = Vec::new();
        for counter in (0..input.len() - 1){
            let difference = input[counter + 1] - input[counter];
            tmp.push(difference)
        }
        tmp
    }

    pub fn main_9_1() {
        let mut total = 0;
        let contents = read("AOC9.txt");
        let contents: Vec<_> = contents.lines().map(|x| {
            let x = x.trim();
            let x: Vec<_> = x.split(' ').map(|y| int!(y.trim(), i32)).collect();
            x
        }).collect();
        for mut line in contents{
            let mut tmp_1 = vec![];
            while check_all_0(&line) != true{
                tmp_1.insert(0, line[line.len() - 1]);
                line = generate_differences(&line)
            }
            // println!("{:?}", tmp_1);
            let mut counter = 0;
            for item in tmp_1{
                counter += item;

            }
            total += counter;
            // println!("{counter}");
        }
        println!("{total}")
    }
}

pub mod aoc9_2{
    use crate::AOC9::aoc9_1::{check_all_0, generate_differences};
    use crate::int;
    use crate::python_builtins::builtins::read;

    pub fn main9_2() {
        let mut total = 0;
        let contents = read("AOC9.txt");
        let contents: Vec<_> = contents.lines().map(|x| {
            let x = x.trim();
            let x: Vec<_> = x.split(' ').map(|y| int!(y.trim(), i32)).collect();
            x
        }).collect();
        for mut line in contents{
            let mut tmp_1 = vec![];
            while check_all_0(&line) != true{
                tmp_1.insert(0, line[0]);
                line = generate_differences(&line)
            }
            //println!("{:?}", tmp_1);
            let mut counter = 0;
            for item in tmp_1{
                counter = item - counter;

            }
            total += counter;
            //println!("{counter}");
        }
        println!("{total}")
    }
}