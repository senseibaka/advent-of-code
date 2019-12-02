use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Iterator;

fn main() {
    let input = first_line(file_to_vec("input.txt".to_string()).unwrap());
    run_program_verbs(input.to_string(), 12, 2);
    run_program_verbs(input.to_string(), 45, 59, 19690720);
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

fn run_program_simple(input: String) -> String {
    println!("input: {}", input);
    let mut program: Vec<i32> = input
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    program = run_program(program);
    let stringified: Vec<String> = program.into_iter().map(|i| i.to_string()).collect();
    let result = stringified.join(",");
    println!("result: {}", result);
    return result;
}

fn run_program_verbs(input: String, noun: i32, verb: i32, expected: i32) {
    let mut program: Vec<i32> = input
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    program[1] = noun;
    program[2] = verb;
    program = run_program(program);
    let result = program[0];
    println!(
        "{}, {} -> {} (matches {}? {}, difference is {})",
        noun,
        verb,
        result,
        expected,
        result == expected,
        (expected - result)
    );

    if result == expected {
        let answer = 100 * noun + verb;
        println!("answer: {}", answer);
    }
}

fn run_program(mut program: Vec<i32>) -> Vec<i32> {
    let mut index: usize = 0;

    loop {
        let opcode: i32 = program[index];

        if opcode == 99 {
            //println!("opcode 99! quit!");
            return program;
        }

        let pos1: usize = program[index + 1] as usize;
        let pos2: usize = program[index + 2] as usize;
        let pos3: usize = program[index + 3] as usize;
        //println!("opcode {} {} {} {}", opcode, pos1, pos2, pos3);

        if opcode == 1 {
            let val1: i32 = program[pos1];
            let val2: i32 = program[pos2];
            let res: i32 = val1 + val2;
            //println!("{} + {} = {} -> {}", val1, val2, res, pos3);
            program[pos3] = res;
        }

        if opcode == 2 {
            let val1: i32 = program[pos1];
            let val2: i32 = program[pos2];
            let res: i32 = val1 * val2;
            //println!("{} * {} = {} -> {}", val1, val2, res, pos3);
            program[pos3] = res;
        }

        index = index + 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_program_works() {
        assert_eq!(
            "2,0,0,0,99".to_string(),
            run_program_simple("1,0,0,0,99".to_string())
        );
        assert_eq!(
            "2,3,0,6,99".to_string(),
            run_program_simple("2,3,0,3,99".to_string())
        );
        assert_eq!(
            "2,4,4,5,99,9801".to_string(),
            run_program_simple("2,4,4,5,99,0".to_string())
        );
        assert_eq!(
            "30,1,1,4,2,5,6,0,99".to_string(),
            run_program_simple("1,1,1,4,99,5,6,0,99".to_string())
        );
    }
}
