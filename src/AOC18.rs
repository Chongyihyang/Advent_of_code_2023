pub mod aoc18 {
    use std::fs::read_to_string;

    pub fn main18() {
        let content = read_to_string("AOC18.txt").unwrap();
        let (mut vertices, mut vertices1) = (vec![], vec![]);
        let (mut curr, mut curr1) = ([0, 0], [0, 0]);
        vertices.push((curr[0], curr[1]));
        vertices1.push((curr1[0], curr1[1]));
        let (mut total, mut total1) = (0, 0);
        for line in content.lines() {
            let code = line.split(" ")
                                 .nth(2)
                                 .unwrap()
                                 .trim()
                                 .trim_start_matches("(#")
                                 .trim_end_matches(")");
            let (magnitude1, direction) = (i64::from_str_radix(&code[0..code.len()-1], 16).unwrap(),
                                                        &code[code.len()-1..code.len()]);
            total1 += magnitude1;
            let magnitude: i64 = line.split(" ").nth(1).unwrap().parse().unwrap();
            total += magnitude;
            match line.chars().nth(0).unwrap() {
                'R' => curr[0] = curr[0] + magnitude,
                'L' => curr[0] = curr[0] - magnitude,
                'U' => curr[1] = curr[1] + magnitude,
                'D' => curr[1] = curr[1] - magnitude,
                _ => {}
            }
            vertices.push((curr[0], curr[1]));
            match direction {
                "0" => curr1[0] = curr1[0] + magnitude1,
                "2" => curr1[0] = curr1[0] - magnitude1,
                "3" => curr1[1] = curr1[1] + magnitude1,
                "1" => curr1[1] = curr1[1] - magnitude1,
                _ => {}
            }
            vertices1.push((curr1[0], curr1[1]));
        }
        let mut total2 = 0;
        for x in 0..vertices.len()-1 {
            let (x0, y0) = vertices.iter().nth(x).unwrap();
            let (x1, y1) = vertices.iter().nth(x+1).unwrap();
            total2 += (x1 + x0) * (y1 - y0);
        }
        total2 = total2.abs();
        total = (total + total2) / 2 + 1;
        let mut total2 = 0;
        for x in 0..vertices1.len()-1 {
            let (x0, y0) = vertices1.iter().nth(x).unwrap();
            let (x1, y1) = vertices1.iter().nth(x+1).unwrap();
            total2 += (x1 + x0) * (y1 - y0);
        }
        total2 = total2.abs();
        total1 = (total1 + total2) / 2 + 1;    
        println!("part 1:{total}, part 2: {total1}");
    }
}
