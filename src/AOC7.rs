pub mod aoc7_1 {
    use std::cmp::Ordering;
    use std::iter::zip;
    use crate::{int, str};
    use crate::python_builtins::builtins::read;

    pub fn rank_hands(hands: &str) -> u8 {

        // check for 5, 4, 3 a kind, or full hse
        let mut tmp = vec![];
        for item in hands.chars(){
            tmp.push(item);
        }
        tmp.sort();
        //println!("{:?}", tmp);

        // full house
        if (tmp[0] == tmp[2] && tmp[3] == tmp[4] && tmp[0] != tmp[3]) ||
            (tmp[0] == tmp[1] && tmp[2] == tmp[4] && tmp[0] != tmp[2]){
            return 5;
        }

        // 5, 4, 3 a kind
        let mut counter = 0;
        let mut first = tmp[0];
        for &item in tmp.iter(){
            if item != first && counter < 3 {
                counter = 1;
                first = item;
            } else if item != first && counter >= 3 {
                break;
            } else {
                counter += 1;
            }
        }
        match counter{
            5 => {return 7},
            4 => {return 6},
            3 => {return 4},
            _ => {}
        }

        // find pairs

        let mut counter = 0u8;
        let mut i = 0usize;
        loop{
            if i >= 4 {
                break
            }
            if tmp[i] == tmp[i+1]{
                counter += 1;
                i += 2;
            } else {
                i += 1;
            }
        }

        return match counter {
            2 => { 3 }
            1 => { 2 }
            _ => { 1 }
        }
    }

    // if card1 == card2, returns true
    pub fn card1_is_larger (card1: &str, card2: &str, rank: &[char]) -> bool{
        for (i, j) in zip(card1.chars(), card2.chars()){
            match rank.iter().position(|x| x == &i).unwrap().cmp(&rank.iter().position(|x| x == &j).unwrap()) {
                Ordering::Less => {return true}
                Ordering::Equal => {}
                Ordering::Greater => {return false}
            }
        }
        true
    }

    pub fn tmp_mapping(input: String) -> Vec<(String, u32)> {
        let input: Vec<(String, u32)> = input.lines().map(|x| {
            let tmp: Vec<_> = x.split(' ').collect();
            (str!(tmp[0]), int!(tmp[1], u32))
        }).collect();
        input
    }

    pub fn insertion_sort(x: &mut Vec<(String, u32)>, rank: &[char]){
        for mut i in 1..x.len(){
            while i > 0 && rank_hands(&*x[i].0) == rank_hands(&*x[i-1].0) && card1_is_larger(&*x[i-1].0, &*x[i].0, &rank) {
                x.swap(i, i-1);
                i -= 1;
            }
        }
    }

    pub fn main_7_1() {
        let rank = ['A', 'K', 'Q', 'J', 'T', '9',
            '8', '7', '6', '5', '4', '3', '2',];
        let contents = read("AOC7.txt");
        let mut contents: Vec<(String, u32)> = tmp_mapping(contents);
        contents.sort_by_key(|x| rank_hands(&*x.0));

        insertion_sort(&mut contents, &rank);

        // now we give a rank, and add them up
        let mut total = 0u32;
        for (rank, &ref i) in contents.iter().enumerate(){
            total += (rank as u32+ 1) * i.1
        }

        println!("{total}");
    }

}

#[cfg(test)]
pub mod tests_aoc7_1 {
    use crate::AOC7::aoc7_1::rank_hands;
    use crate::AOC7::aoc7_1::card1_is_larger;
    #[test]
    fn rank_hands_works() {
        assert_eq!(rank_hands("32T3K"), 2);
        assert_eq!(rank_hands("T55J5"), 4);
        assert_eq!(rank_hands("KK677"), 3);
        assert_eq!(rank_hands("KTJJT"), 3);
        assert_eq!(rank_hands("QQQJA"), 4);

        // given test cases
        assert_eq!(rank_hands("AAAAA"), 7);
        assert_eq!(rank_hands("AA8AA"), 6);
        assert_eq!(rank_hands("23332"), 5);
        assert_eq!(rank_hands("TTT98"), 4);
        assert_eq!(rank_hands("23432"), 3);
        assert_eq!(rank_hands("A23A4"), 2);
        assert_eq!(rank_hands("23456"), 1);
    }

    #[test]
    fn card1_is_larger_works() {
        let rank = ['A', 'K', 'Q', 'J', 'T', '9',
            '8', '7', '6', '5', '4', '3', '2',];
        assert_eq!(card1_is_larger("KTJJT", "KK677", &rank), false);
        assert_eq!(card1_is_larger("T55J5", "QQQJA", &rank), false);
    }

}

pub mod aoc7_2 {
    use crate::AOC7::aoc7_1::{insertion_sort, rank_hands, tmp_mapping};
    use crate::python_builtins::builtins::read;

    pub fn rank_hands_2(hands: &str) -> u8 {

        let mut tmp = vec![];
        for item in hands.chars(){
            tmp.push(item);
        }
        tmp.sort();
        if !tmp.contains(&'J') {
            return rank_hands(hands)
        }
        return match tmp.iter().filter(|&x| x == &'J').count() {
            5 => { 7 }
            4 => { 7 }
            3 => {
                let rank = rank_hands(hands);
                if rank == 5 {
                    // there will be 3 x cards and 2 y cards
                    return 7
                }
                if rank == 4 {
                    // there will be 3 x cards, 1 y card and 1 z card
                    return 6
                }
                7
            }
            2 => {
                let rank = rank_hands(hands);
                if rank == 5 {
                    // there will be 3 x cards and 2 y cards
                    return 7
                }
                if rank == 3 {
                    // there will be 2 x cards, 2 y card and 1 z card. W L O G, let 2 x be the 'J'
                    return 6
                }
                if rank == 2 {
                    // there will be 2 x cards, 1 y card, 1 z card, and 1 t card. W L O G, let 2 x be 'J'
                    return 4
                }
                0
            }
            1 => {
                let rank = rank_hands(hands);
                if rank == 6 {
                    return 7
                }
                if rank == 4 {
                    // there will be 3 x cards, 1 y card and 1 z card
                    return 6
                }
                if rank == 3 {
                    // there will be 2 x cards, 2 y card and 1 z card. W L O G, let 1 z be the 'J'
                    return 5
                }
                if rank == 2 {
                    // there will be 2 x cards, 1 y card, 1 z card, and 1 t card. W L O G, let 1 t be 'J'
                    return 4
                }
                2
            }
            _ => {1}
        }
    }

    pub fn main_7_2() {
        let rank = ['A', 'K', 'Q', 'T', '9',
            '8', '7', '6', '5', '4', '3', '2', 'J'];
        let contents = read("AOC7.txt");
        let mut contents: Vec<(String, u32)>= tmp_mapping(contents);
        contents.sort_by_key(|x| rank_hands_2(&*x.0));

        insertion_sort(&mut contents, &rank);

        // now we give a rank, and add them up
        let mut total = 0u32;
        for (rank, &ref i) in contents.iter().enumerate(){
            total += (rank as u32+ 1) * i.1
        }

        println!("{total}");
    }

}

#[cfg(test)]
pub mod tests_aoc7_2 {
    use crate::AOC7::aoc7_2::rank_hands_2;

    #[test]
    fn rank_hands_works() {
        assert_eq!(rank_hands_2("32T3K"), 2);
        assert_eq!(rank_hands_2("T55J5"), 6);
        assert_eq!(rank_hands_2("KK677"), 3);
        assert_eq!(rank_hands_2("KTJJT"), 6);
        assert_eq!(rank_hands_2("QQQJA"), 6);

        // given test cases
        assert_eq!(rank_hands_2("8JJA9"), 4);
        assert_eq!(rank_hands_2("AA8AA"), 6);
        assert_eq!(rank_hands_2("23332"), 5);
        assert_eq!(rank_hands_2("TTT98"), 4);
        assert_eq!(rank_hands_2("23432"), 3);
        assert_eq!(rank_hands_2("A23A4"), 2);
        assert_eq!(rank_hands_2("23456"), 1);
        assert_eq!(rank_hands_2("JTTJT"), 7);
        assert_eq!(rank_hands_2("J2J28"), 6);
    }

}