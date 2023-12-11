pub mod aoc11_1{
    use num::abs;
    use crate::python_builtins::builtins::read;

    pub fn expand_rows(input: &String) -> Vec<u32>{
        let mut rows = vec![];
        for (i, line) in input.lines().rev().enumerate(){
            if line.chars().all(|x| x == '.'){
                rows.push(i as u32);
            }
        }
        rows
    }

    pub fn expand_columns(input: &String) -> Vec<u32>{
        let mut cols = vec![];
        let mut counter = 0usize;
        loop{
            if input.lines().map(|x| x.chars().nth(counter).unwrap()).all(|x| x == '.'){
                cols.push(counter as u32);
            }
            if counter == input.lines().nth(0).unwrap().len() - 1{
                break;
            }
            counter += 1;
        }
        cols
    }

    pub fn find_galaxies(input: &str, cols: &Vec<u32>, rows: &Vec<u32>) -> Vec<Coordinates> {
        let mut coords = vec![];
        for (mut y, line) in input.lines().rev().enumerate(){
            for (mut x, char) in line.chars().enumerate(){
                if char == '#' {
                    let (mut x, mut y) = (x as u32, y as u32);
                    x += cols.iter().filter(|&q| q < &x).count() as u32 * (1000000 - 1);
                    y += rows.iter().filter(|&q| q < &y).count() as u32 * (1000000 - 1);
                    coords.push(Coordinates::new(x, y))
                }
            }
        }
        coords
    }

    pub fn find_permutations(x: u32, y: u32, vecs: Vec<Coordinates>) -> u64{
        let mut counter = 0u64;
        for i in (x..=y){
            for j in (i+1..=y){
                let (i, j) = (i as usize- 1, j as usize - 1);
                let (i, j) = (vecs[i].clone(), vecs[j].clone());
                counter += i.find_shortest_distance(j) as u64;
            }
        }
        counter
    }

    #[derive(Clone)]
    pub struct Coordinates{
        x: u32,
        y: u32,
    }

    impl Coordinates{
        pub fn new(x: u32, y: u32) -> Self {
            Self {x, y}
        }

        pub fn find_shortest_distance(&self, coord: Coordinates) -> u32 {
            let (x, y) = (self.x as i32 - coord.x as i32, self.y as i32 - coord.y as i32);
            let total = abs(x) + abs(y);
            //  println!("{total}, ({},{}), ({},{})", self.x, self.y, coord.x, coord.y);
            total as u32
        }
    }

    pub fn main11_1(){
        let mut contents = read("AOC11.txt");
        let (cols, rows) = (expand_columns(&contents), expand_rows(&contents));
        let galaxies = find_galaxies(&*contents, &cols, &rows);
        let steps = find_permutations(1, galaxies.len() as u32, galaxies);
        println!("{steps}");
    }
}