use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;
use std::iter::Iterator;

fn main() {
    let lines = file_to_vec("input.txt".to_string()).unwrap();
    let result_part1 = lines.iter().map(|x| x.parse::<i32>().unwrap()).map(fuel_cost).fold(0, |acc, x| acc + x);
    println!("part 1 answer: {}", result_part1);
    let result_part2 = lines.iter().map(|x| x.parse::<i32>().unwrap()).map(recursive_fuel_cost).fold(0, |acc, x| acc + x);
    println!("part 2 answer: {}", result_part2);
}

fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn fuel_cost(mass: i32) -> i32 {
    let cost = (mass / 3) - 2;

    return match cost {
        c if c <= 0 => 0,
        _ => cost
    }
}

fn recursive_fuel_cost(mass: i32) -> i32 {
    let cost = fuel_cost(mass);

    return match cost {
        c if c <= 0 => 0,
        _ => cost + recursive_fuel_cost(cost)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fuel_cost_works() {
        assert_eq!(2, fuel_cost(12));
        assert_eq!(2, fuel_cost(14));
        assert_eq!(654, fuel_cost(1969));
        assert_eq!(33583, fuel_cost(100756));
    }

    #[test]
    fn recursive_fuel_cost_works() {
        assert_eq!(2, recursive_fuel_cost(14));
        assert_eq!(966, recursive_fuel_cost(1969));
        assert_eq!(50346, recursive_fuel_cost(100756));
    }
}
