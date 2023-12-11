pub mod aoc10_2 {
    use std::collections::VecDeque;
    use std::str::FromStr;
    use crate::python_builtins::builtins::read;

    const GRID_SIZE: usize = 140;

    #[derive(Clone, Copy, Debug, PartialEq)]
    enum Direction {
        North,
        East,
        South,
        West,
    }

    const COMPASS: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Pipe(Direction, Direction);

    impl Pipe {
        fn traverse(self, edge: Direction) -> Option<Direction> {
            if edge == self.0 {
                Some(self.1)
            } else if edge == self.1 {
                Some(self.0)
            } else {
                None
            }
        }

        fn read_from_char(ch: char) -> Option<Self> {
            match ch {
                '|' => Some(Self(Direction::North, Direction::South)),
                '-' => Some(Self(Direction::East, Direction::West)),
                'L' => Some(Self(Direction::North, Direction::East)),
                'J' => Some(Self(Direction::North, Direction::West)),
                '7' => Some(Self(Direction::South, Direction::West)),
                'F' => Some(Self(Direction::East, Direction::South)),
                _ => None,
            }
        }
    }

    #[derive(Debug, PartialEq)]
    struct EdgePosition {
        pos: usize,
        edge: Direction,
        steps: u32,
    }

    impl EdgePosition {
        fn cross_edge(&self) -> Self {
            match self.edge {
                Direction::North => Self {
                    pos: self.pos - GRID_SIZE,
                    edge: Direction::South,
                    steps: self.steps,
                },
                Direction::East => Self {
                    pos: self.pos + 1,
                    edge: Direction::West,
                    steps: self.steps,
                },
                Direction::South => Self {
                    pos: self.pos + GRID_SIZE,
                    edge: Direction::North,
                    steps: self.steps,
                },
                Direction::West => Self {
                    pos: self.pos - 1,
                    edge: Direction::East,
                    steps: self.steps,
                },
            }
        }

        fn traverse_pipe(&self, pipe: Pipe) -> Option<Self> {
            pipe.traverse(self.edge).map(|edge| Self {
                pos: self.pos,
                edge,
                steps: self.steps + 1,
            })
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    enum Corner {
        TopLeft,
        TopRight,
        BottomLeft,
        BottomRight,
    }

    impl Corner {
        fn move_crosses(self, direction: Direction) -> Option<Direction> {
            match (self, direction) {
                (Corner::TopLeft, Direction::East) | (Corner::TopRight, Direction::West) => {
                    Some(Direction::North)
                }
                (Corner::TopLeft, Direction::South) | (Corner::BottomLeft, Direction::North) => {
                    Some(Direction::West)
                }
                (Corner::TopRight, Direction::South) | (Corner::BottomRight, Direction::North) => {
                    Some(Direction::East)
                }
                (Corner::BottomLeft, Direction::East) | (Corner::BottomRight, Direction::West) => {
                    Some(Direction::South)
                }
                _ => None,
            }
        }
    }

    #[derive(Debug, PartialEq)]
    struct CornerPosition {
        pos: usize,
        corner: Corner,
    }

    impl CornerPosition {
        fn adjacent_positions<'a>(&'a self, pipe: &'a Option<Pipe>) -> impl Iterator<Item = Self> + 'a {
            // calculate adjacent locations, accounting for OOB
            let north = self.pos.checked_sub(GRID_SIZE);
            let east = if (self.pos % GRID_SIZE) == (GRID_SIZE - 1) {
                None
            } else {
                Some(self.pos + 1)
            };
            let west = self.pos.checked_sub(1);
            let south = if (self.pos / GRID_SIZE) == (GRID_SIZE - 1) {
                None
            } else {
                Some(self.pos + GRID_SIZE)
            };

            COMPASS.iter().filter_map(move |direction| {
                // check for move blocked by part of pipe
                if let Some(cross) = self.corner.move_crosses(*direction) {
                    if let Some(pipe) = pipe {
                        if pipe.0 == cross || pipe.1 == cross {
                            return None;
                        }
                    }
                }

                // calculate destination
                match (direction, self.corner) {
                    (Direction::North, Corner::TopLeft) => north.map(|pos| Self {
                        pos,
                        corner: Corner::BottomLeft,
                    }),
                    (Direction::North, Corner::TopRight) => north.map(|pos| Self {
                        pos,
                        corner: Corner::BottomRight,
                    }),
                    (Direction::North, Corner::BottomLeft) | (Direction::West, Corner::TopRight) => {
                        Some(Self {
                            pos: self.pos,
                            corner: Corner::TopLeft,
                        })
                    }
                    (Direction::North, Corner::BottomRight) | (Direction::East, Corner::TopLeft) => {
                        Some(Self {
                            pos: self.pos,
                            corner: Corner::TopRight,
                        })
                    }
                    (Direction::East, Corner::TopRight) => east.map(|pos| Self {
                        pos,
                        corner: Corner::TopLeft,
                    }),
                    (Direction::East, Corner::BottomLeft) | (Direction::South, Corner::TopRight) => {
                        Some(Self {
                            pos: self.pos,
                            corner: Corner::BottomRight,
                        })
                    }
                    (Direction::East, Corner::BottomRight) => east.map(|pos| Self {
                        pos,
                        corner: Corner::BottomLeft,
                    }),
                    (Direction::South, Corner::TopLeft) | (Direction::West, Corner::BottomRight) => {
                        Some(Self {
                            pos: self.pos,
                            corner: Corner::BottomLeft,
                        })
                    }
                    (Direction::South, Corner::BottomLeft) => south.map(|pos| Self {
                        pos,
                        corner: Corner::TopLeft,
                    }),
                    (Direction::South, Corner::BottomRight) => south.map(|pos| Self {
                        pos,
                        corner: Corner::TopRight,
                    }),
                    (Direction::West, Corner::TopLeft) => west.map(|pos| Self {
                        pos,
                        corner: Corner::TopRight,
                    }),
                    (Direction::West, Corner::BottomLeft) => west.map(|pos| Self {
                        pos,
                        corner: Corner::BottomRight,
                    }),
                }
            })
        }
    }

    #[derive(Debug, PartialEq)]
    struct CornerVisitTracker([u8; GRID_SIZE * GRID_SIZE]);

    impl CornerVisitTracker {
        fn new() -> Self {
            Self([0; GRID_SIZE * GRID_SIZE])
        }

        fn count_unvisited(&self) -> u32 {
            self.0
                .iter()
                .fold(0, |empty, val| empty + u32::from(val == &0))
        }

        fn visit(&mut self, pos: &CornerPosition) -> bool {
            let value = match pos.corner {
                Corner::TopLeft => 1,
                Corner::TopRight => 2,
                Corner::BottomLeft => 4,
                Corner::BottomRight => 8,
            };
            let visited = self.0[pos.pos] & value == value;
            self.0[pos.pos] |= value;
            visited
        }
    }

    struct Maze {
        start: usize,
        grid: [Option<Pipe>; GRID_SIZE * GRID_SIZE],
    }

    impl Maze {
        fn furthest_point_in_loop(&self) -> Option<u32> {
            let mut visited = [false; GRID_SIZE * GRID_SIZE];
            let mut queue = VecDeque::new();

            for edge in COMPASS {
                queue.push_back(
                    EdgePosition {
                        pos: self.start,
                        edge,
                        steps: 0,
                    }
                        .cross_edge(),
                );
            }

            while let Some(pos) = queue.pop_front() {
                if let Some(pipe) = self.grid[pos.pos] {
                    if let Some(next) = pos.traverse_pipe(pipe).map(|p| p.cross_edge()) {
                        if visited[next.pos] {
                            return Some(next.steps + 1);
                        }
                        visited[next.pos] = true;
                        queue.push_back(next);
                    }
                }
            }

            None
        }

        fn replacement_start_pipe(&self) -> Option<Pipe> {
            let pos = self.start;
            let mut edges = COMPASS.iter().filter_map(|edge| {
                let cross = EdgePosition {
                    pos,
                    edge: *edge,
                    steps: 0,
                }
                    .cross_edge();
                let Some(pipe) = self.grid[cross.pos] else {
                    return None;
                };
                cross.traverse_pipe(pipe).map(|_| edge)
            });

            let first = edges.next();
            let second = edges.next();
            match (first, second) {
                (Some(first), Some(second)) => Some(Pipe(*first, *second)),
                _ => None,
            }
        }

        fn spaces_enclosed_by_loop(&self) -> u32 {
            let mut visited = CornerVisitTracker::new();
            let mut queue = VecDeque::new();

            let start_pipe = self.replacement_start_pipe();

            queue.push_back(CornerPosition {
                pos: 0,
                corner: Corner::TopLeft,
            });

            while let Some(pos) = queue.pop_front() {
                if !visited.visit(&pos) {
                    let pipe = if pos.pos == self.start {
                        start_pipe
                    } else {
                        self.grid[pos.pos]
                    };
                    queue.extend(pos.adjacent_positions(&pipe));
                }
            }

            visited.count_unvisited()
        }
    }

    #[derive(Debug, PartialEq)]
    struct ParseMazeError;

    impl FromStr for Maze {
        type Err = ParseMazeError;

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            let mut start = 0;
            let mut grid = [None; GRID_SIZE * GRID_SIZE];

            for (row, line) in input.lines().enumerate() {
                let row_start = row * GRID_SIZE;
                for (col, ch) in line.chars().enumerate() {
                    let pos = row_start + col;
                    grid[pos] = Pipe::read_from_char(ch);
                    if ch == 'S' {
                        start = pos;
                    }
                }
            }

            Ok(Self { start, grid })
        }
    }

    #[must_use]
    pub fn main10_2(){
        let input = read("AOC10.txt");
        let input = &*input;
        if let Ok(maze) = Maze::from_str(input) {
            println!("{}", maze.spaces_enclosed_by_loop());
        }
    }
}

pub mod aoc10_1 {
    use crate::python_builtins::builtins::{chr, ord, read};
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
        pub top: Option<char>,
        pub bottom: Option<char>,
        pub left: Option<char>,
        pub right: Option<char>,
        pub coord: (u32, u32),
        pub count: u32,
        pub character: char,
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
            if y >= 1{
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
            input = change_char(input.clone(), x, y, 48);
            let (x , y) = (x as usize, y as usize);
            match self.character{
                '|' => {
                    if get_char(&input, x, y-1) != '0' {oth.push(Frame::new(input.clone(), x, y - 1, self.count + 1, self.top.unwrap()))};
                    if get_char(&input, x, y+1) != '0' {oth.push(Frame::new(input.clone(), x, y + 1, self.count + 1, self.bottom.unwrap()))};
                },
                '-' => {
                    if get_char(&input, x-1, y) != '0' {oth.push(Frame::new(input.clone(), x - 1, y, self.count + 1, self.left.unwrap()))};
                    if get_char(&input, x+1, y) != '0' {oth.push(Frame::new(input.clone(), x + 1, y, self.count + 1, self.right.unwrap()))};
                },
                'L' => {
                    if get_char(&input, x, y-1) != '0' {oth.push(Frame::new(input.clone(), x, y - 1, self.count + 1, self.top.unwrap()))};
                    if get_char(&input, x+1, y) != '0' {oth.push(Frame::new(input.clone(), x + 1, y, self.count + 1, self.right.unwrap()))};
                },
                'J' => {
                    if get_char(&input, x, y-1) !='0' {oth.push(Frame::new(input.clone(), x, y - 1, self.count + 1, self.top.unwrap()))};
                    if get_char(&input, x-1, y) !='0' {oth.push(Frame::new(input.clone(), x - 1, y, self.count + 1, self.left.unwrap()))};
                },
                '7' => {
                    if get_char(&input, x-1, y) != '0' {oth.push(Frame::new(input.clone(), x - 1, y, self.count + 1, self.left.unwrap()))};
                    if get_char(&input, x, y+1) != '0' {oth.push(Frame::new(input.clone(), x, y + 1, self.count + 1, self.bottom.unwrap()))};
                },
                'F' => {
                    if get_char(&input, x+1, y) != '0' {oth.push(Frame::new(input.clone(), x + 1, y, self.count + 1, self.right.unwrap()))};
                    if get_char(&input, x, y+1) != '0' {oth.push(Frame::new(input.clone(), x, y + 1, self.count + 1, self.bottom.unwrap()))};
                }
                _ => {}
            }
            (oth, input)
        }

        pub fn coord(&self) -> (u32, u32) {
            self.coord
        }
    }

    pub fn main10_1() {
        let mut contents = read("AOC10.txt");
        let mut frame = Frame::new(contents.clone(), 74, 95, 25000, 'L');
        let mut queue = vec![];
        let mut high = 0u32;
        loop {
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
        }
        println!("{}", high - 25000);
        println!("{contents}");
    }
}
