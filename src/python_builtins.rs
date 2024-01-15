pub mod builtins {
    #[allow(unused_imports)]
    use std::cmp::Ordering;
    #[allow(unused_imports)]
    use std::fmt::Display;
    #[allow(unused_imports)]
    use std::fs::{remove_file, File, OpenOptions};
    #[allow(unused_imports)]
    use std::io::{Read, Write};

    #[macro_export]
    macro_rules! range {
        ($x: expr) => {
            (0..$x)
        };
        ($x: expr, $y: expr) => {
            ($x..$y)
        };
        ($x: expr, $y: expr, $z: expr) => {
            ($x..$y).step_by($z as usize)
        };
        ($x: expr, $y: expr, $z: expr, $sign: expr ) => {
            ($x..$y).step_by($z as usize).rev()
        };
    }

    #[macro_export]
    macro_rules! input {
        () => {{
            print!("\n");
            let mut new = String::new();
            std::io::stdin().read_line(&mut new).unwrap();
            new.trim().to_string()
        }};
        ($($arg:tt)*) => {{
            println!("{}", format_args!($($arg)*));
            let mut new = String::new();
            std::io::stdin().read_line(&mut new).unwrap();
            new.trim().to_string()
        }};
    }

    #[allow(dead_code)]
    pub fn write(filename: &str, content: &str, modes: &str) {
        match modes {
            "w" => {
                let mut file;
                match File::open(filename) {
                    Ok(..) => {
                        remove_file(filename).expect("could not remove file");
                        file = File::create(filename).unwrap();
                    }
                    Err(..) => {
                        file = File::create(filename).unwrap();
                    }
                }
                file.write(content.as_bytes()).expect("write failed");
            }
            "a" => {
                let mut file = OpenOptions::new()
                    .append(true)
                    .open(filename)
                    .expect("cannot open file");

                file.write(content.as_bytes()).expect("write failed");
            }
            _ => {}
        }
    }

    #[allow(dead_code)]
    pub fn read(filename: &str) -> String {
        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }

    #[macro_export]
    macro_rules! to_char {
        ($x: expr, $y: expr) => {
            $x.as_bytes()[$y] as char
        };
    }

    #[macro_export]
    macro_rules! len {
        ($x: expr) => {
            $x.len()
        };
    }

    #[macro_export]
    macro_rules! int {
        ($x: expr) => {
            $x.parse::<i64>().unwrap_or_else(|x| panic!("{x} is returned"))
        };
        ($x: expr, $y: ident) => {
            $x.parse::<$y>().unwrap_or_else(|x| panic!("{x} is returned"))
        };
    }

    #[macro_export]
    macro_rules! str {
        () => {
            String::new()
        };
        ($x: expr) => {
            $x.to_string()
        };
        ($x: expr, $y: expr) => {
            $x.chars().enumerate().filter(|&(i, _)| i < $y ).map(|(_, e)| e).collect::<String>()
        };
        ($x: expr, $y: expr, $z: expr) => {
            if $y > $ z {
                $x.chars().enumerate().filter(|&(i, _)| {i >= $z && i < $y} ).map(|(_, e)| e).collect::<String>().chars().rev().collect::<String>()
            } else {
                $x.chars().enumerate().filter(|&(i, _)| {i >= $y && i < $z} ).map(|(_, e)| e).collect::<String>()
            }
        };
    }
    #[allow(dead_code)]
    pub fn ord(some_chr: char) -> u32 {
        let something = some_chr.clone();
        something as u32
    }
    #[allow(dead_code)]
    pub fn chr(some_num: &u32) -> char {
        let something = some_num.clone();
        char::from_u32(something).unwrap()
    }
    #[allow(dead_code)]
    pub fn search_arr<T: PartialOrd>(lst: &[T], item: &T) -> isize {
        let mut l = 0 as usize;
        let mut r = len!(lst) - 1;
        while l <= r {
            let mid = (l + r) / 2;
            match lst[mid].partial_cmp(item) {
                Some(Ordering::Greater) => r = mid - 1,
                Some(Ordering::Equal) => return mid as isize,
                Some(Ordering::Less) => l = mid + 1,
                _ => {}
            }
        }
        -1
    }
    #[allow(dead_code)]
    pub fn clear_line() {
        print!("\x1B[2J\x1B[1;1H");
    }
}
