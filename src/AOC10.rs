pub mod aoc10_2 {
    use crate::python_builtins::builtins::{chr, ord, read, write};



    pub fn main10_2(){
        let contents = read("AOC10_2.txt");
        let mut contents: Vec<_> = contents
            .lines()
            .map(|x| {
                x.chars().map(|mut x| {
                    if x == '.'{
                        x = '0'
                    } else if ord(x) >= 25000 {
                        x = ' '
                    } else {
                        x = '*'
                    }
                    x
                }).collect::<Vec<char>>()
            })
            .collect();
        let mut contents = contents
            .iter()
            .map(|x| {
                let mut x = x.iter().collect::<String>();
                x.push_str("\r\n");
                //println!("{x}");
                x
            })
            .collect::<String>();
        write("AOC10_2.txt", &*contents, "w");
    }



    pub fn change_char(input: String, x: u32, y: u32, count: u32) -> String {
        // let og = input.len();
        let (x, y) = (x as usize, y as usize);
        let mut input: Vec<_> = input
            .lines()
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect();
        input[y][x] = chr(&count);
        // println!("{}, {count}", chr(&count));
        let mut input = input
            .iter()
            .map(|x| {
                let mut x = x.iter().collect::<String>();
                x.push_str("\r\n");
                //println!("{x}");
                x
            })
            .collect::<String>();
        input.trim().to_string();
        // println!("{input}");
        // assert_eq!(og, input.len());
        input
    }

    pub fn get_char(input: &String, x: usize, y: usize) -> char {
        // let og = input.len();
        let mut input: Vec<_> = input
            .lines()
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect();
        input[y][x]
    }

    #[derive(Clone)]
    pub struct Frame {
        top: Option<char>,
        bottom: Option<char>,
        left: Option<char>,
        right: Option<char>,
        coord: (u32, u32),
        pub count: u32,
        character: char,
    }

    impl Frame {
        pub fn new(input: String, x: usize, y: usize, count: u32, character: char) -> Self {
            // println!("{x}, {y}");
            // settle the left and right
            let curr = input.lines().nth(y).unwrap();
            let mut left = curr.chars().nth(x - 1);
            let mut right = curr.chars().nth(x + 1);

            // settle the top
            let mut top = None;
            if y > 0{
                let curr = input.lines().nth(y - 1);
                if let Some(i) = curr {
                    top = curr.unwrap().chars().nth(x);
                }
            }


            // settle the bottom
            let curr = input.lines().nth(y + 1);
            let mut bottom = None;
            if let Some(i) = curr {
                bottom = curr.unwrap().chars().nth(x);
            }

            Self {
                top,
                bottom,
                left,
                right,
                coord: (x as u32, y as u32),
                count,
                character,
            }
        }



        pub fn change_tabs(&self, mut input: String) -> (Vec<Frame>, String) {
            let mut oth = vec![];
            let (x, y) = self.coord;
            input = change_char(input.clone(), x, y, self.count);
            let (x , y) = (x as usize, y as usize);
            match self.character{
                '|' => {
                    if ord(get_char(&input, x, y-1)) < 25000 {oth.push(Frame::new(input.clone(), x, y - 1, self.count + 1, self.top.unwrap()))};
                    if ord(get_char(&input, x, y+1)) < 25000 {oth.push(Frame::new(input.clone(), x, y + 1,self.count + 1, self.bottom.unwrap()))};
                },
                '-' => {
                    if ord(get_char(&input, x-1, y)) < 25000 {oth.push(Frame::new(input.clone(), x - 1, y, self.count + 1, self.left.unwrap()))};
                    if ord(get_char(&input, x+1, y)) < 25000 {oth.push(Frame::new(input.clone(), x + 1, y,self.count + 1, self.right.unwrap()))};
                },
                'L' => {
                    if ord(get_char(&input, x, y-1)) < 25000 {oth.push(Frame::new(input.clone(), x, y - 1, self.count + 1, self.top.unwrap()))};
                    if ord(get_char(&input, x+1, y)) < 1000 {oth.push(Frame::new(input.clone(), x + 1, y,self.count + 1, self.right.unwrap()))};
                },
                'J' => {
                    if ord(get_char(&input, x, y-1)) < 25000 {oth.push(Frame::new(input.clone(), x, y - 1, self.count + 1, self.top.unwrap()))};
                    if ord(get_char(&input, x-1, y)) < 1000 {oth.push(Frame::new(input.clone(), x - 1, y,self.count + 1, self.left.unwrap()))};
                },
                '+' => {
                    if ord(get_char(&input, x-1, y)) < 25000 {oth.push(Frame::new(input.clone(), x - 1, y, self.count + 1, self.left.unwrap()))};
                    if ord(get_char(&input, x, y+1)) < 1000 {oth.push(Frame::new(input.clone(), x, y + 1,self.count + 1, self.bottom.unwrap()))};
                },
                'F' => {
                    if ord(get_char(&input, x+1, y)) < 25000 {oth.push(Frame::new(input.clone(), x + 1, y, self.count + 1, self.right.unwrap()))};
                    if ord(get_char(&input, x, y+1)) < 25000 {oth.push(Frame::new(input.clone(), x, y + 1,self.count + 1, self.bottom.unwrap()))};
                }
                _ => {}
            }
            (oth, input)
        }

        pub fn coord(&self) -> (u32, u32) {
            self.coord
        }
    }
}

pub mod aoc10_1 {
    use crate::python_builtins::builtins::read;
    use crate::AOC10::aoc10_2::Frame;

    pub fn main10_1() {
        let mut contents = read("AOC10.txt");
        // current character = 'L'
        let mut frame = Frame::new(contents.clone(), 74, 95, 25000, 'L');
        let mut queue = vec![];
        let mut high = 0u32;
        loop {
            // let (x, y) = frame.coord();
            // println!("{x}, {y}");
            let tmp = frame.change_tabs(contents.clone());
            let mut new_frames = tmp.0;
            contents = tmp.1;
            queue.append(&mut new_frames);
            frame = queue.remove(0);
            if frame.count > high{
                high = frame.count
            }
            if queue.len() == 0 {
                break;
            }
            //println!("{contents}");
        }
        println!("{}", high - 25000);
        println!("{contents}");
    }
}
