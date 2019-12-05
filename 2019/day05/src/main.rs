use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Iterator;

fn main() {
    let program_spec = first_line(file_to_vec("input.txt".to_string()).unwrap());
    let (result, output) = run_program_simple(program_spec, "1".to_string());
    println!("PART 1 OUTPUT: {}", output);
    //println!("RESULT: {}", result);
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

fn comma_separated_ints_to_vec(line: String) -> Vec<i32> {
    line.split(",")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn vec_to_comma_separated_ints(vec: Vec<i32>) -> String {
    let list: Vec<_> = vec.into_iter().map(|i| i.to_string()).collect();
    return list.join(",");
}

fn run_program_simple(program_spec: String, input_spec: String) -> (String, String) {
    //println!("program_spec: {}", program_spec);
    //println!("input_spec: {}", program_spec);

    let mut program_in = comma_separated_ints_to_vec(program_spec);
    let mut inputs = comma_separated_ints_to_vec(input_spec);

    let (program_out, outputs) = run_program(program_in, inputs);

    let ret1 = vec_to_comma_separated_ints(program_out);
    let ret2 = vec_to_comma_separated_ints(outputs);
    //println!("ret1 {}", ret1);
    //println!("ret2 {}", ret2);
    return (ret1, ret2);
}

fn decode(instruction: i32) -> (i32, bool, bool, bool) {
    /*
        ABCDE
        DE = opcode
        C = p1 mode
        B = p2 mode
        A = p3 mode
        Parameters that an instruction writes to will never be in immediate mode.
    */
    let opcode = instruction % 100;
    let p1_mode = (instruction / 100) % 10;
    let p2_mode = (instruction / 1000) % 10;
    let p3_mode = (instruction / 10000) % 10;
    //println!("{} ({}, {}, {})", opcode, p1_mode, p2_mode, p3_mode);
    return (opcode, p1_mode == 1, p2_mode == 1, p3_mode == 1);
}

fn run_program(mut program: Vec<i32>, inputs: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut pc: usize = 0;
    let mut input_index: usize = 0;
    let mut outputs: Vec<i32> = Vec::new();

    loop {
        let (opcode, is_immediate_1, is_immediate_2, is_immediate_3) = decode(program[pc]);

        //println!("opcode {}", opcode);

        if opcode == 99 {
            return (program, outputs);
        }

        match opcode {
            1 => {
                let val1: i32 = if is_immediate_1 { program[pc+1] } else { program[(program[pc+1] as usize)]};
                let val2: i32 = if is_immediate_2 { program[pc+2] } else { program[(program[pc+2] as usize)]};
                //let pos1: usize = program[pc + 1] as usize;
                //let pos2: usize = program[pc + 2] as usize;
                let pos3: usize = program[pc + 3] as usize;
                //let val1: i32 = program[pos1];
                //let val2: i32 = program[pos2];
                let res: i32 = val1 + val2;
                //println!("{} + {} = {} -> {}", val1, val2, res, pos3);
                program[pos3] = res;
                pc += 4;
            },
            2 => {
                let val1: i32 = if is_immediate_1 { program[pc+1] } else { program[(program[pc+1] as usize)]};
                let val2: i32 = if is_immediate_2 { program[pc+2] } else { program[(program[pc+2] as usize)]};
                /* let pos1: usize = program[pc + 1] as usize;
                let pos2: usize = program[pc + 2] as usize; */
                let pos3: usize = program[pc + 3] as usize;
                /* let val1: i32 = program[pos1];
                let val2: i32 = program[pos2]; */
                let res: i32 = val1 * val2;
                //println!("{} * {} = {} -> {}", val1, val2, res, pos3);
                program[pos3] = res;
                pc += 4;
            },
            3 => {
                let pos1: usize = program[pc + 1] as usize;
                let val: i32 = inputs[input_index];
                //println!("(in){} -> {}", val, pos1);
                program[pos1] = val;
                input_index += 1;
                pc += 2;
            },
            4 => {
                //let pos1: usize = program[pc + 1] as usize;
                //let val: i32 = program[pos1];
                let val: i32 = if is_immediate_1 { program[pc+1] } else { program[(program[pc+1] as usize)]};
                //let val2: i32 = if is_immediate_2 { program[pc+2] } else { program[(program[pc+2] as usize)]};
                //println!("(out){}", val);
                outputs.push(val);
                pc += 2;
            },
            _ => {
                panic!("Encountered unexpected opcode {}", opcode);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_program_works() {
        run_program_case_no_io("1,0,0,0,99".to_string(), "2,0,0,0,99".to_string());
        run_program_case_no_io("2,3,0,3,99".to_string(), "2,3,0,6,99".to_string());
        run_program_case_no_io("2,4,4,5,99,0".to_string(), "2,4,4,5,99,9801".to_string());
        run_program_case_no_io("1,1,1,4,99,5,6,0,99".to_string(), "30,1,1,4,2,5,6,0,99".to_string());
        run_program_case_no_io("1002,4,3,4,33".to_string(), "1002,4,3,4,99".to_string());
    }

    #[test]
    fn run_program_with_io_works(){ 
        run_program_case("3,0,4,0,99".to_string(), "1".to_string(), "1,0,4,0,99".to_string(), "1".to_string())
    }

    fn run_program_case_no_io(program_spec: String, expected_program: String) {
        run_program_case(program_spec, "".to_string(), expected_program, "".to_string());
    }

    fn run_program_case(program_spec: String, input_spec: String, expected_program: String, expected_output: String) {
        let (prog, output) = run_program_simple(program_spec, input_spec);
        assert_eq!(expected_program, prog);
        assert_eq!(expected_output, output);
    }

    #[test]
    fn decode_works() {
        assert_eq!((1, false, false, false), decode(1));
        assert_eq!((2, true, false, false), decode(102));
        assert_eq!((2, false, true, false), decode(1002));
        assert_eq!((2, false, false, true), decode(10002));
        assert_eq!((2, true, true, true), decode(11102));
    }
}
