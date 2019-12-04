use std::iter::Iterator;

fn main() {
    let range1 = 284639..=748759;
    let valid1 = range1
        .filter(|x| has_same_adjacent(*x))
        .filter(|x| never_deacreases(*x))
        .fold(0, |acc, x| acc + 1);
    println!("part 1: {}", valid1);
    
    let range2 = 284639..=748759;
    let valid2 = range2
        //.filter(|x| has_same_adjacent(*x))
        .filter(|x| contains_an_exact_pair(*x))
        .filter(|x| never_deacreases(*x))
        .fold(0, |acc, x| acc + 1);
    println!("part 2: {}", valid2);
}

fn has_same_adjacent(candidate: i32) -> bool {
    let chars: Vec<char> = candidate.to_string().chars().collect();
    for i in 1..chars.len() {
        let prev = chars[i-1];
        let cur = chars[i];
        if prev == cur {
            return true;
        }
    }

    return false;
}

fn contains_an_exact_pair (candidate: i32) -> bool {
    let s: String = candidate.to_string();
    let chars: Vec<char> = s.chars().collect();
    for c in chars {
        if s.matches(c).count() == 2 {
            return true;
        }
    }
    return false;
}

fn never_deacreases(candidate: i32) -> bool {
    let chars: Vec<char> = candidate.to_string().chars().collect();
    for i in 1..chars.len() {
        let prev = chars[i-1];
        let cur = chars[i];
        
        if prev > cur {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn has_same_adjacent_works() {
        assert_eq!(true, has_same_adjacent(111111));
        assert_eq!(true, has_same_adjacent(223450));

        assert_eq!(false, has_same_adjacent(123789));
    }
    
    #[test]
    fn never_deacreases_works() {
        assert_eq!(true, never_deacreases(111111));
        assert_eq!(true, never_deacreases(123789));

        assert_eq!(false, never_deacreases(223450));
    }

    #[test]
    fn contains_an_exact_pair_works() {
        assert_eq!(true, contains_an_exact_pair(112233));
        assert_eq!(true, contains_an_exact_pair(111122));

        assert_eq!(false, contains_an_exact_pair(123444));
    }
}
