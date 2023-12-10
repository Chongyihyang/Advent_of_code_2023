pub mod aoc9_1{
    use crate::int;
    use crate::python_builtins::builtins::read;

    pub fn check_all_0(input: &Vec<i32>) -> bool {
        input.iter().all(|x| *x == 0)
    }

    pub fn generate_differences(input: &Vec<i32>) -> (Vec<i32>, bool) {
        let mut tmp = Vec::new();
        let mut is_zero = true;
        for counter in (0..input.len() - 1){
            let difference = input[counter + 1] - input[counter];
            if difference != 0 {
                is_zero = false
            }
            tmp.push(difference)
        }
        (tmp, is_zero)
    }

    pub fn abstraction(reverse: bool){
        let contents = read("AOC9.txt");
        let contents: i32 = contents
            .lines()
            .map(|x| {
                let mut counter = 0;
                let mut pos = true;
                let mut line = x
                    .split(' ')
                    .map(|y| int!(y.trim(), i32))
                    .collect::<Vec<_>>();

                loop{
                    let mut is_zero = true;
                    if !reverse {
                        counter += line[line.len() - 1];
                    } else {
                        if pos{
                            counter += line[0];
                            pos = false;
                        } else {
                            counter -= line[0];
                            pos = true;
                        }
                    }
                    (line, is_zero) = generate_differences(&line);
                    if is_zero{
                        break
                    }
                }
                counter})
            .sum();
        println!("{contents}")
    }

    pub fn main9_1() {
        abstraction(false)
    }
}

pub mod aoc9_2{
    use crate::AOC9::aoc9_1::abstraction;
    pub fn main9_2() {
        abstraction(true)
    }
}