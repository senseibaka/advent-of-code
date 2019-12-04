use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Iterator;
use std::collections::HashSet;
use std::collections::HashMap;
use rusttype;

fn main() {
    let lines = file_to_vec("input.txt".to_string()).unwrap();
    let points1 = path_to_points((&lines[0]).to_string());
    let points2 = path_to_points((&lines[1]).to_string());
    println!("part1 answer: {}", solution_1(&points1, &points2));
    println!("part2 answer: {}", solution_2(&points1, &points2));
}

fn solution_1(points1: &HashMap<rusttype::Point<i32>, i32>, points2: &HashMap<rusttype::Point<i32>, i32>) -> i32 {
    //equivalent: let keys1: HashSet<rusttype::Point<i32>> = HashSet::from_iter(points1.keys().cloned());
    let keys1: HashSet<_> = points1.keys().cloned().collect();
    let keys2: HashSet<_> = points2.keys().cloned().collect();
    return keys1.intersection(&keys2)
        .map(|p| p.x.abs() + p.y.abs())
        .filter(|d| *d != 0)
        .fold(std::i32::MAX, |acc, d| if acc > d {d} else {acc});
}

fn solution_2(points1: &HashMap<rusttype::Point<i32>, i32>, points2: &HashMap<rusttype::Point<i32>, i32>) -> i32 {
    return points1.iter()
        .filter(|&(k, v)| *v != 0)
        .filter(|&(k, v)| points2.contains_key(&*k)) 
        .map(|(k, v)| *v + points2.get(k).unwrap())
        .fold(std::i32::MAX, |acc, d| if acc > d {d} else {acc});
}

fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn path_to_points(path: String) -> HashMap<rusttype::Point<i32>, i32> {
    let mut points: HashMap<rusttype::Point<i32>, i32> = HashMap::new();
    let mut steps = 0;
    let mut x = 0;
    let mut y = 0;
    for command in path.split(",") {
        let distance: i32 = command[1..].parse::<i32>().unwrap();
        let range = 1..distance+1;
        match command {
            c if c.starts_with("L") => {
                for i in range {
                    steps += 1;
                    x -= 1;
                    points.insert(rusttype::Point {
                        x: x,
                        y: y
                    }, steps);
                }
            },
            c if c.starts_with("R") => {
                for i in range {
                    steps += 1;
                    x += 1;
                    points.insert(rusttype::Point {
                        x: x,
                        y: y
                    }, steps);
                }
            },
            c if c.starts_with("U") => {
                for i in range {
                    steps += 1;
                    y += 1;
                    points.insert(rusttype::Point {
                        x: x,
                        y: y
                    }, steps);
                }
            },
            c if c.starts_with("D") => {
                for i in range {
                    steps += 1;
                    y -= 1;
                    points.insert(rusttype::Point {
                        x: x,
                        y: y
                    }, steps);
                }
            },
            _ => println!("noop")
        }
    }
    println!("final: {}, {}", x, y);
    return points;
}