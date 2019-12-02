use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Iterator;

fn main() {
    let input = first_line(file_to_vec("input.txt".to_string()).unwrap());
    println!("part 1 answer: {}", determine_floor(&input));
    println!("part 2 answer: {}", determine_basement_hit(&input));
}

fn first_line(lines: Vec<String>) -> String {
    return match lines.first() {
        Some(x) => x.to_string(),
        None => "".to_string(),
    };
}

fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn determine_floor(input: &String) -> i32 {
    return input
        .chars()
        .map(determine_increment)
        .fold(0, |acc, x| acc + x);
}

fn determine_basement_hit(input: &String) -> i32 {
    let mut counter = 1;
    let mut floor = 0;
    for c in input.chars() {
        floor += determine_increment(c);
        if floor < 0 {
            return counter;
        }
        counter = counter + 1;
    }
    return 0;
}

fn determine_increment(input: char) -> i32 {
    return match input {
        '(' => 1,
        ')' => -1,
        _ => 0,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn determine_floor_works() {
        assert_eq!(0, determine_floor(&"(())".to_string()));
        assert_eq!(0, determine_floor(&"()()".to_string()));
        assert_eq!(3, determine_floor(&"(((".to_string()));
        assert_eq!(3, determine_floor(&"(()(()(".to_string()));
        assert_eq!(3, determine_floor(&"))(((((".to_string()));
        assert_eq!(-1, determine_floor(&"())".to_string()));
        assert_eq!(-1, determine_floor(&"))(".to_string()));
        assert_eq!(-3, determine_floor(&")))".to_string()));
        assert_eq!(-3, determine_floor(&")())())".to_string()));
    }

    #[test]
    fn determine_basement_hit_works() {
        assert_eq!(1, determine_basement_hit(&")".to_string()));
        assert_eq!(5, determine_basement_hit(&"()())".to_string()));
    }
}
