use common::*;
use std::iter::Iterator;

fn main() {
    let lines: Vec<i32> = file_to_vec("input.txt".to_string())
        .unwrap()
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    println!(
        "part 1 answer: {}",
        lines
            .clone()
            .iter()
            .map(|x| fuel_cost(*x))
            .fold(0, |acc, x| acc + x)
    );
    println!(
        "part 2 answer: {}",
        lines
            .clone()
            .iter()
            .map(|x| recursive_fuel_cost(*x))
            .fold(0, |acc, x| acc + x)
    );
}

fn fuel_cost(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn recursive_fuel_cost(mass: i32) -> i32 {
    return match fuel_cost(mass) {
        c if c <= 0 => 0,
        c => c + recursive_fuel_cost(c),
    };
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
