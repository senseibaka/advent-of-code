use common::*;
use std::f64;
use std::cmp::Eq;
use std::hash::Hash;
use std::hash::Hasher;
use std::collections::HashSet;

fn main() {
    let lines = file_to_vec("input.txt".to_string()).unwrap();
    let (station, p1sol) = part_1_solution(&lines);
    println!("Place the station at {:?}, it can see {} asteroids", station, p1sol);
}

fn part_1_solution(lines: &Vec<String>) -> ((i32, i32), usize) {
let mut x = 0;
    let mut y = 0;
    let mut asteroids: HashSet<(i32, i32)> = HashSet::new();
    
    for line in lines {
        for c in line.chars() {
            if c == '#' {
                asteroids.insert((x, y));
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }

    let mut best_count: usize = 0;
    let mut best: (i32, i32) = (0,0);
    for asteroid in &asteroids {
        let mut angles: HashSet<Double> = HashSet::new();
        for other_asteroid in &asteroids {
            if asteroid == other_asteroid {
                continue;
            }
            let a = ((other_asteroid.1 - asteroid.1) as f64).atan2(
                (other_asteroid.0 - asteroid.0) as f64
            );
            angles.insert(Double(a));
        }

        let count = angles.len();
        if count > best_count {
            best_count = count;
            best = (asteroid.0, asteroid.1);
        }
    }

    (best, best_count)
}

#[derive(Debug)]
struct Double(f64);

impl Double {
    fn canonicalize(&self) -> i64 {
        (self.0 * 1024.0 * 1024.0).round() as i64
    }
}

impl PartialEq for Double {
    fn eq(&self, other: &Double) -> bool {
        self.canonicalize() == other.canonicalize()
    }
}

impl Eq for Double {}

impl Hash for Double {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.canonicalize().hash(state);
    }
}