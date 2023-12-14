    pub mod aoc14_1 {
        use std::iter::zip;
        use crate::python_builtins::builtins::read;

        pub fn sort_by_line(input: &mut Vec<Vec<char>>, vertical : bool, reverse: bool, line: usize){

            if vertical {
                if reverse {
                    for mut i in (0..input.len()-1).rev(){
                        if input[i][line] == 'O' {
                            while i < input.len() - 1 && input[i+1][line] != 'O' && input[i+1][line] != '#' {
                                let tmp = input[i][line];
                                input[i][line] = input[i+1][line];
                                input[i+1][line] = tmp;
                                i += 1;
                            }
                        }
                    }
                } else {
                    for mut i in 0..input.len(){
                        if input[i][line] == 'O' {
                            while i > 0 && input[i-1][line] != 'O' && input[i-1][line] != '#' {
                                let tmp = input[i][line];
                                input[i][line] = input[i-1][line];
                                input[i-1][line] = tmp;
                                i -= 1;
                            }
                        }
                    }
                }
            } else {
                if reverse {
                    for mut i in (0..input[0].len() - 1).rev(){
                        if input[line][i] == 'O' {
                            while i < input[0].len() - 1 && input[line][i+1] != 'O' && input[line][i+1] != '#' {
                                input[line].swap(i, i+1);
                                i += 1;
                            }
                        }
                    }
                } else {
                    for mut i in 1..input[0].len() {
                        if input[line][i] == 'O' {
                            while i > 0 && input[line][i-1] != 'O' && input[line][i-1] != '#' {
                                input[line].swap(i, i-1);
                                i -= 1;
                            }
                        }
                    }
                }
            }
        }

        pub fn count_by_line(input: &Vec<Vec<char>>, length: usize) -> u32 {
            let mut counter = 0u32;
            for q in 0..length{
                for (i, j) in zip((0..input.len()), (1..=input.len()).rev())  {
                    if input[i][q] == 'O' {
                        counter += j as u32
                    }
                }
            }

            counter
        }

    pub fn main14_1() {
        let content = read("AOC14.txt");
        let length = content.lines().nth(0).unwrap().len();
        let line_size = content.lines().count();
        let mut content = {
            let mut tmp = vec![];
            for line in content.lines(){
                let line: Vec<_> = line.chars().collect();
                tmp.push(line)
            }
            tmp
        };
        for i in 0..line_size{
           sort_by_line(&mut content, true, false, i );
        }
        println!("{}", count_by_line(&content, length));
    }
}

pub mod aoc14_2 {
    use crate::AOC14::aoc14_1::{count_by_line, sort_by_line};
    use crate::python_builtins::builtins::read;

    pub fn main14_2() {
        let content = read("AOC14.txt");
        let length = content.lines().nth(0).unwrap().len();
        let line_size = content.lines().count();
        let mut content = {
            let mut tmp = vec![];
            for line in content.lines(){
                let line: Vec<_> = line.chars().collect();
                tmp.push(line)
            }
            tmp
        };
        let mut in_stored = vec![];
        let (mut initial, mut final_) = (0, 0);
        let mut is_contiguous = false;
        loop {
            //north
            for i in 0..line_size{
                sort_by_line(&mut content, true, false, i );
            }
            //west
            for i in 0..length{
                sort_by_line(&mut content, false, false, i );
            }
            // south
            for i in 0..line_size{
                sort_by_line(&mut content, true, true, i );
            }
            //east
            for i in 0..length{
                sort_by_line(&mut content, false, true, i );
            }
            let amt = count_by_line(&content, length);
            if in_stored.contains(&amt) {
                if is_contiguous {
                    println!("{:?}", in_stored);
                    let idx = in_stored.iter().position(|x| x == &amt).unwrap().clone();
                    (initial, final_) = (idx + 1, in_stored.len() - idx);
                    println!("{initial}, {final_}");
                    in_stored = in_stored[idx..].to_owned();
                    break
                } else {
                    in_stored.push(amt);
                }
            } else {
                is_contiguous = !is_contiguous;
                in_stored.push(amt);
            }
        }

        let result = 1000000000 - (initial + ((1000000000 - initial) / final_) * final_);
        println!("Result: {:?}, {result}, {}", in_stored[result], in_stored.len())
    }
}
