pub mod aoc3_1 {
    use crate::int;
    use crate::python_builtins::builtins::read;

    pub fn is_sign(input: &char) -> bool {
        return if input != &'.' && !input.is_digit(10){
            true
        } else {
            false
        }
    }

    pub fn find_sign(content: &Vec<Vec<char>>, line: usize, start: usize, end: usize) -> bool {
        let line_height = content.len();
        let line_width = content.iter().nth(0).unwrap().len();

        let (mut x, mut y) = (start, end);
        // found right
        if end < line_width - 1 {
            y += 1;
            if is_sign(&content[line][y]){return true}
        }
        // found left
        if start > 0 {
            x -= 1;
            if is_sign(&content[line][x]){return true}
        }
        // found top
        if line > 0 {
            let l = line - 1;
            for q in (x..=y){
                if is_sign(&content[l][q]){return true}
            }
        }
        // found bottom
        if line < line_height - 1 {
            let l = line + 1;
            for q in (x..=y){
                if is_sign(&content[l][q]){return true}
            }
        }

        false
    }

    pub fn main3_1() {
        let mut counter = 0u32;
        let contents = read("AOC3.txt");
        let contents: Vec<_> = contents.lines()
            .map(|x| x.chars().collect::<Vec<_>>()).collect();
        let line_height = contents.len();
        let line_width = contents.iter().nth(0).unwrap().len();
        let (mut x, mut y) = (0, 0);
        while x < line_height {
            let (mut start, mut end, line) = (0, 0, x);
            while y < line_width {
                if contents[x][y].is_digit(10){
                    start = y;
                    while y < line_width && contents[x][y].is_digit(10){
                        y += 1;
                    }
                    end = y - 1;
                    if find_sign(&contents, line,start, end){
                        let num = int!(contents[x][start..=end]
                            .iter().collect::<String>(), u32);
                        counter += num;
                    }
                } else {
                    y += 1;
                }
            }
            y = 0;
            x += 1
        }
        println!("{counter}");
    }
}

pub mod aoc3_2 {
    use crate::python_builtins::builtins::read;

    pub fn find_num(content: &Vec<Vec<char>>, line: usize, start: usize, stop: usize, from_right: bool) -> u32 {
        let mut line: Vec<_> = content.iter().nth(line).unwrap()[start..=stop].iter().collect(); // [4,8,.,]
        if from_right{
            let mut x = line.len() - 1;
            loop {
                let num: String = line[0..=x].iter().map(|&x| x.clone()).collect();
                match num.parse::<u32>() {
                    Ok(y) => {
                        return y;
                    }
                    Err(_) => {}
                }
                if x > 0{
                    x -= 1
                } else {
                    break
                }
            }
        } else {
            line.reverse();
            let mut x = line.len() - 1;
            loop {
                let num: String = line[0..=x].iter().rev().map(|&x| x.clone()).collect();
                match num.parse::<u32>() {
                    Ok(y) => {
                        println!("{y}");
                        return y;
                    }
                    Err(_) => {}
                }
                if x > 0{
                    x -= 1
                } else {
                    break
                }
            }
        }
        println!("0");
        0
    }

    pub fn find_num_vert(content: &Vec<Vec<char>>, line: usize, idx: usize) -> u8 {
        0
    }

    pub fn find_star(content: &Vec<Vec<char>>, line: usize, loc: usize) -> u32 {
        let line_height = content.len();
        let line_width = content.iter().nth(0).unwrap().len();
        let mut number_of_items = 0u8;
        let mut total = 1u32;

        let (mut x, mut y) = ((loc - 3) % line_width, (loc + 3) % line_width);
        if x > loc {x = 0}
        if y < loc {y = line_width - 1}

        // found right
        let tmp = find_num(content, line, loc + 1, y, true);
        if tmp != 0 {
            number_of_items += 1;
            total *= tmp;
        }

        // found left
        let tmp = find_num(content, line, x, loc-1, false);
        if tmp != 0 {
            number_of_items += 1;
            total *= tmp;
        }

        // found top
        if line > 0 && find_num_vert(content, line - 1, loc) != 0 {
            number_of_items += 1;
        }
        // found bottom
        if line < line_height - 1 && find_num_vert(content, line + 1, loc) != 0 {
            number_of_items += 1;
        }

        return if number_of_items == 2{
            total
        } else {
            0
        }

    }

    pub fn main3_2() {
        let mut counter = 0u32;
        let contents = read("AOC3.txt");
        let contents: Vec<_> = contents.lines()
            .map(|x| x.chars().collect::<Vec<_>>()).collect();
        let line_height = contents.len();
        let line_width = contents.iter().nth(0).unwrap().len();
        let (mut x, mut y) = (0, 0);
        while x < line_height {
            while y < line_width {
                if contents[x][y] == '*'{
                    counter += find_star(&contents, x, y);
                }
                y += 1
            }
            y = 0;
            x += 1
        }
        println!("{counter}");
    }
}