use common::*;
use std::f64;
use std::cmp::Eq;
//use std::cmp::Ordering;
use std::hash::Hash;
use std::hash::Hasher;
use std::collections::HashSet;
use std::collections::HashMap;
//use std::collections::BTreeMap;

fn main() {
    let lines = file_to_vec("input.txt".to_string()).unwrap();
    let (station, p1sol, visible) = part_1_solution(&lines);
    println!("Place the station at {:?}, it can see {} asteroids, which are:", station, p1sol);
    //let mut ordered_visible: BTreeMap<Double, (i32, i32)> = BTreeMap::new();
    for asteroid in visible {
        let angle = (asteroid.0).0;
        //let mut adjusted = angle + f64::consts::FRAC_PI_2;
        let mut adjusted = f64::consts::FRAC_PI_2 - angle;
        if adjusted < 0.0 {
            adjusted = adjusted + f64::consts::PI + f64::consts::PI;
        }
        //let adjusted = angle;
        //println!("{} => {:?} ", (asteroid.0).0, asteroid.1);
        println!("{} => {:?}", adjusted, asteroid.1);
    }
    //cargo run > output.txt
    //sort the lines in your text editor, 200th line is the point :)
    //TODO: sort the stuff in code and print the result
}

fn part_1_solution(lines: &Vec<String>) -> ((i32, i32), usize, HashMap<Double, (i32, i32)>) {
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
    let mut best_visible: HashMap<Double, (i32, i32)> = HashMap::new();
    let mut best: (i32, i32) = (0,0);
    for asteroid in &asteroids {
        let mut angles: HashMap<Double, (i32, i32)> = HashMap::new();
        for other_asteroid in &asteroids {
            if asteroid == other_asteroid {
                continue;
            }
            let a = ((asteroid.1 - other_asteroid.1) as f64).atan2(
                (other_asteroid.0 - asteroid.0) as f64
            );
            //check which is closest to the asteroid, and store that
            let angle = Double(a);
            match angles.get(&angle) {
                Some(existing) => {
                    let existing_distance = (existing.0 - asteroid.0).abs() + (existing.1 - asteroid.1).abs();
                    let candidate_distance = (other_asteroid.0 - asteroid.0).abs() + (other_asteroid.1 - asteroid.1).abs();
                    //println!("{:?} vs {:?}", existing, other_asteroid);
                    if candidate_distance < existing_distance {
                        angles.insert(Double(a), (other_asteroid.0, other_asteroid.1));
                    }
                },
                None => {
                    angles.insert(Double(a), (other_asteroid.0, other_asteroid.1));
                }
            };
        }

        let count = angles.len();
        if count > best_count {
            best_count = count;
            best_visible = angles.clone();
            best = (asteroid.0, asteroid.1);
        }
    }

    (best, best_count, best_visible)
}

#[derive(Debug, Clone, Copy)]
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

/* impl Ord for Double {
    fn cmp(&self, other: &Self) -> Ordering {

    }
} */
