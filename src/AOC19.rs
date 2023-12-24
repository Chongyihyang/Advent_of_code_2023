pub mod aoc19_1 {
    use std::collections::HashMap;
    use crate::python_builtins::builtins::read;
    use crate::{int, str, to_char};

    #[derive(Debug)]
    pub struct Limits {
        name: char,
        sign: char,
        num: u32,
        destination: String
    }

    impl Limits {
        pub fn new(name: char, sign: char, num:u32, destination: String) -> Self {
            Self {name, sign, num, destination}
        }
    }
    #[derive(Debug)]
    pub enum LimitTypes {
        Limits(Limits),
        Single(String)
    }


    pub fn main19_1() {
        let mut total = 0u32;
        let content = read("AOC19.txt");
        let  content: Vec<_> = content.split("\r\n\r\n").collect();
        let mut workflow_hashmap = HashMap::new();
        let mut has_rej = vec![];
        let (_, parts) = (content[0].clone().to_string()
                              .lines()
                              .map(|x| {
                                  let mut limits_vec = vec![];
                                  let (workstation, limits) =
                                      (x.split('{').nth(0).unwrap(),
                                       x.split('{').nth(1).unwrap()
                                           .split('}').nth(0).unwrap());
                                  for limit in limits.split(',') {
                                      if limit.len() <= 3{
                                          limits_vec.push(LimitTypes::Single(str!(limit)))
                                      } else {
                                          let tmp= str!(limit, 2);
                                          let tmp: Vec<_>  = tmp.split(':').collect();
                                          // println!("{:?}", tmp);
                                          let (num, destination) = (int!(tmp[0], u32), str!(tmp[1]));
                                          limits_vec.push(LimitTypes::Limits(
                                              Limits::new(
                                                  to_char!(limit, 0),
                                                  to_char!(limit, 1),
                                                  num, destination)
                                          ))
                                      }
                                  }
                                  workflow_hashmap.insert(workstation.to_string(), limits_vec);
                              })
                              .collect::<Vec<_>>(),
                          content[1]);
        for part in parts.lines() {
            let part: Vec<_> = part.strip_prefix('{').unwrap()
                            .strip_suffix('}').unwrap().split(",")
                            .map(|x| int!(str!(x, 2), u32)).collect();
            let count: u32 = part.iter().sum();
            let mut range = [1u32, 4000, 1, 4000, 1, 4000, 1, 4000];
            let mut current = str!("in");
            loop {
                let instructions = workflow_hashmap.get(&current).unwrap();
                let mut loop_break = false;
                let loc_map = ['x', 'm', 'a', 's'];
                for instruction in instructions {
                    match instruction {
                        LimitTypes::Limits(T) => {
                            let loc = loc_map.iter().position(|x| x == &T.name).unwrap();
                            let loc = part[loc];
                            if T.sign == '<' && loc < T.num {
                                if T.destination == str!("A") {
                                    total += count;
                                    loop_break = true;
                                    break;
                                } else if T.destination == str!("R") {
                                    loop_break = true;
                                    break;
                                } else {
                                    current = T.destination.clone();
                                    break;
                                }
                            } else if T.sign == '>' && loc > T.num {
                                if T.destination == str!("A") {
                                    total += count;
                                    loop_break = true;
                                    break;
                                } else if T.destination == str!("R") {
                                    loop_break = true;
                                    break;
                                } else {
                                    current = T.destination.clone();
                                    break;
                                }
                            }
                        }
                        LimitTypes::Single(T) => {
                            if T == &str!("A") {
                                total += count;
                                loop_break = true;
                                break;
                            } else if T == &str!("R") {
                                loop_break = true;
                                break;
                            } else {
                                current = T.clone();
                                break;
                            }
                        }
                    }
                }
                if loop_break {
                    //println!("{current}");
                    break
                }
            }
        }
        println!("{total}");
        println!("{:?}", has_rej.len());
    }
}