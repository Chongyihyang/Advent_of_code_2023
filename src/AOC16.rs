pub mod aoc16_1 {
    use itertools::Itertools;
    use crate::python_builtins::builtins::read;

    #[derive(Clone, PartialEq, Debug)]
    pub enum Direction {
        Up,
        Down,
        Left,
        Right
    }
    #[derive(Debug)]
    pub struct Location {
        pub x: usize,
        pub y: usize,
        pub direction: Direction,
        pub limit: (usize, usize),
    }

    impl Location {
        pub fn new(x: usize, y: usize, direction: Direction, limit: (usize, usize)) -> Self {
            Self{x, y, direction, limit}
        }
        pub fn next_location(&self, direction: &Direction) -> (Option<usize>, Option<usize>) {
            let mut return_value = (Some(self.x), Some(self.y));
            match direction {
                Direction::Up => {
                    if self.y > 0 {
                        return_value.1 = Some(self.y - 1);
                    } else {
                        return_value.1 = None
                    }
                }
                Direction::Down => {
                    if self.y < self.limit.1  {
                        return_value.1 = Some(self.y + 1);
                    } else {
                        return_value.1 = None
                    }
                }
                Direction::Left => {
                    if self.x > 0 {
                        return_value.0 = Some(self.x - 1);
                    } else {
                        return_value.0 = None
                    }
                }
                Direction::Right => {
                    if self.x < self.limit.0  {
                        return_value.0 = Some(self.x + 1);
                    }else {
                        return_value.0 = None
                    }
                }
            }
            return_value
        }

        pub fn identify_direction(&self, input: &Vec<Vec<char>>) -> Vec<Direction> {
            let mut directions = vec![];
            //println!("{}", input[self.y][self.x]);
            match input[self.y][self.x]{
                '.' => {
                    directions.push(self.direction.clone());
                },
                '|' => {
                    if self.direction == Direction::Left ||
                        self.direction == Direction::Right {
                        directions.push(Direction::Up);
                        directions.push(Direction::Down);
                    } else {
                        directions.push(self.direction.clone());
                    }
                },
                '\\' => {
                    match self.direction{
                        Direction::Up => {directions.push(Direction::Left)}
                        Direction::Down => {directions.push(Direction::Right)}
                        Direction::Left => {directions.push(Direction::Up)}
                        Direction::Right => {directions.push(Direction::Down)}
                    }
                },
                '/' => {
                    match self.direction{
                        Direction::Up => {directions.push(Direction::Right)}
                        Direction::Down => {directions.push(Direction::Left)}
                        Direction::Left => {directions.push(Direction::Down)}
                        Direction::Right => {directions.push(Direction::Up)}
                    }
                },
                '-' => {
                    if self.direction == Direction::Up ||
                        self.direction == Direction::Down {
                        directions.push(Direction::Left);
                        directions.push(Direction::Right);
                    } else {
                        directions.push(self.direction.clone());
                    }
                },
                _ => {}
            }
            directions
        }
    }

    pub fn start(contents: &mut Vec<Vec<char>>, limit: (usize, usize), initial: Location) -> usize{
        let mut coords = vec![];
        let mut coords_with_direction = vec![];
        // contents[y][x]
        let mut queue: Vec<Location> = Vec::new();
        queue.push(initial);
        while queue.len() > 0{
            let tmp = queue.remove(0);
            if !coords.iter().contains(&(tmp.x, tmp.y)){
                coords.push((tmp.x, tmp.y));
            }
            if !coords_with_direction.iter().contains(&(tmp.x, tmp.y, tmp.direction.clone())) {
                let directions = tmp.identify_direction(&contents);
                coords_with_direction.push((tmp.x, tmp.y, tmp.direction.clone()));
                for direction in directions {
                    let (x, y) = tmp.next_location(&direction);
                    //println!("{}, {:?}, {:?}, {:?}", queue.len(), direction, x, y);
                    if x != None && y != None {
                        queue.push(Location::new(x.unwrap(),y.unwrap(), direction.clone(), limit))
                    }
                }
            } else {
                continue
            }
        }
        // for (x, y) in &coords {
        //     contents[*y][*x] = '#'
        // }
        // println!("{}", coords.len());
        // for line in contents{
        //     let line: String = line.clone().iter().collect();
        //     println!("{line}", );
        // }
        coords.len()
    }

    pub fn main16_1() {
        let contents = read("AOC16.txt");
        let mut contents: Vec<_> = contents.lines()
            .map(|x| {x.chars().collect::<Vec<_>>()})
            .collect();
        let line_width = contents.iter().nth(0).unwrap().len() - 1;
        let line_height = contents.iter().count() - 1;
        let limit = (line_width, line_height);
        start(&mut contents, limit, Location::new(line_width - 1, 19, Direction::Left, limit));
    }
}

pub mod aoc16_2 {
    use crate::AOC16::aoc16_1::{Direction, Location, start};
    use crate::python_builtins::builtins::read;

    pub fn main16_2() {
        let contents = read("AOC16.txt");
        let mut contents: Vec<_> = contents.lines()
            .map(|x| {x.chars().collect::<Vec<_>>()})
            .collect();
        let line_width = contents.iter().nth(0).unwrap().len() - 1;
        let line_height = contents.iter().count() - 1;
        let limit = (line_width, line_height);
        let mut max = 0;
        // for top
        for i in 0..=line_width {
            let tmp = start(&mut contents, limit, Location::new(i, 0, Direction::Down, limit));
            if tmp > max {max = tmp}
        }
        // for bottom
        for i in 0..=line_width {
            let tmp = start(&mut contents, limit, Location::new(i, line_height, Direction::Up, limit));
            if tmp > max {max = tmp}
        }
        // for left
        for i in 0..=line_height {
            let tmp = start(&mut contents, limit, Location::new(0, i, Direction::Right, limit));
            if tmp > max {max = tmp}
        }
        // for right
        for i in 0..=line_height {
            let tmp = start(&mut contents, limit, Location::new(line_width, i, Direction::Left, limit));
            if tmp > max {max = tmp}
        }
        println!("max: {max}");
    }
}