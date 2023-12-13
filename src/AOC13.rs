pub mod aoc13_1 {
    use crate::python_builtins::builtins::read;

    pub fn scan(input: &Vec<String>, num: usize, size: usize) -> u32 {
        let (mut left, mut right) = (num - 1, num);
        let mut counter = 0u32;
        loop {
            if input[left] == input[right]{
                counter += 1;
                if left == 0 || right == size - 1 {
                    break
                }
                left -= 1;
                right += 1;
            } else {
                counter = 0;
                break;
            }
        }
        counter
    }

    pub enum Option {
        Rows,
        Columns,
        None
    }

    pub fn each_pattern(contents: &String) -> (u32, Option) {
        let rows: Vec<_> = contents.lines().map(|x| x.to_string()).collect();
        let mut columns = vec![];
        for i in (0..rows[0].len()){
            let tmp: String = contents.lines().map(|x| x[i..i+1].to_string()).collect();
            columns.push(tmp);
        }
        let (rows_len, col_len) = (rows.len(), columns.len());
        let (mut rows_counter, mut cols_counter) = (0, 0);
        let (mut cols_, mut rows_) = (0, 0);

        // we start the procedure from right to left
        for i in (1..rows[0].len()).rev(){
            //println!("{}, cols", i);
            cols_counter = scan(&columns, i, col_len);
            if cols_counter != 0 {
                cols_ = i;
                break
            }
        }

        // we start the procedure from bottom to top
        for i in (1..rows.len()).rev(){
            //println!("{}, rows", i);
            rows_counter = scan(&rows, i, rows_len);
            if rows_counter != 0 {
                rows_ = i;
                break
            }
        }

        return if rows_counter != 0 {
            ((rows_ * 100) as u32, Option::Rows)
        } else if cols_counter != 0{
            (cols_ as u32, Option::Columns)
        } else {
            (0, Option::None)
        }
    }

    pub fn main13_1() {
        let contents = read("AOC13.txt");
        let patterns: Vec<_> = contents.split("\r\n\r\n")
            .map(|x| x.to_string()).collect();
        let mut counter = 0u32;
        for pattern in patterns{
            counter += each_pattern(&pattern).0;
        }
        println!("{counter}");
    }
}

pub mod aoc13_2 {
    use crate::python_builtins::builtins::read;

    fn count_diff(s1: &Vec<char>, s2: &Vec<char>) -> u32 {
        let mut count = 0;
        for (ch1, ch2) in s1.iter().zip(s2.iter()) {
            if ch1 != ch2 {
                count += 1;
            }
        }
        count
    }

    fn find_reflection(pattern: &Vec<Vec<char>>) -> u32 {
        let num_rows = pattern.len();
        let num_cols = pattern[0].len();
        for split_col in 1..num_cols {
            let left_range = (0..split_col).rev();
            let right_range = split_col..num_cols;
            let mut diff_count = 0;
            for column_pair in left_range.zip(right_range) {
                let left = pattern
                    .iter()
                    .map(|row| row[column_pair.0])
                    .collect::<Vec<char>>();
                let right = pattern
                    .iter()
                    .map(|row| row[column_pair.1])
                    .collect::<Vec<char>>();
                diff_count += count_diff(&left, &right);
            }
            if diff_count == 1 {
                return split_col as u32;
            }
        }
        for split_row in 1..num_rows {
            let top_range = (0..split_row).rev();
            let bottom_range = split_row..num_rows;
            let mut diff_count = 0;
            for row_pair in top_range.zip(bottom_range) {
                let top = &pattern[row_pair.0];
                let bottom = &pattern[row_pair.1];
                diff_count += count_diff(&top, &bottom);
            }
            if diff_count == 1 {
                return (split_row as u32) * 100;
            }
        }
        0
    }

    fn split_patterns(contents: &str) -> Vec<Vec<Vec<char>>> {
        let mut patterns: Vec<Vec<Vec<char>>> = vec![];
        let mut current_pattern: Vec<Vec<char>> = vec![];
        for line in contents.lines() {
            if line.is_empty() {
                patterns.push(current_pattern);
                current_pattern = vec![];
            } else {
                let row: Vec<char> = line.chars().collect();
                current_pattern.push(row);
            }
        }
        patterns.push(current_pattern);
        patterns
    }

    fn process(contents: &str) -> u32 {
        let mut sum: u32 = 0;
        let patterns = split_patterns(contents);
        for pattern in patterns {
            sum += find_reflection(&pattern);
        }
        sum
    }

    pub fn main13_2() {
        let contents = read("AOC13.txt");
        let result = process(&contents);
        println!("result = {result}");
    }
}