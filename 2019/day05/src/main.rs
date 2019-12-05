use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Iterator;

fn main() {
    let program_spec = first_line(file_to_vec("input.txt".to_string()).unwrap());
    /* let (result, output) = run_program_simple(program_spec.to_string(), "1".to_string());
    println!("PART 1 OUTPUT: {}", output);
    let (result, output) = run_program_simple(program_spec.to_string(), "5".to_string());
    println!("PART 2 OUTPUT: {}", output); */
    let (result, output) = run_program_new(program_spec.to_string(), "1".to_string());
    println!("PART 1 OUTPUT: {}", output);
    let (result, output) = run_program_new(program_spec.to_string(), "5".to_string());
    println!("PART 2 OUTPUT: {}", output);
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

fn run_program_new(program_spec: String, input_spec: String) -> (String, String) {
    let mut program_in = comma_separated_ints_to_vec(program_spec);
    let mut inputs = comma_separated_ints_to_vec(input_spec);

    let mut emulator = Emulator::new(program_in, inputs, true);
    emulator.run_program();
    
    let ret1 = vec_to_comma_separated_ints(emulator.program);
    let ret2 = vec_to_comma_separated_ints(emulator.outputs);
    return (ret1, ret2);
}

struct Emulator {
    pc: usize,
    debug: bool,
    program: Vec<i32>,
    inputs: Vec<i32>,
    outputs: Vec<i32>
}

impl Emulator {
    fn new(program: Vec<i32>, inputs: Vec<i32>, debug: bool) -> Emulator {
        Emulator {
            pc: 0,
            debug,
            program,
            inputs,
            outputs: vec![]
        }
    }

    fn print_debug(&self, statement: String) {
        if self.debug {
            println!("{}", statement);
        }
    }

    /*
        ABCDE
        DE = opcode
        C = p1 mode
        B = p2 mode
        A = p3 mode
        Parameters that an instruction writes to will never be in immediate mode.
    */
    fn get_opcode(&self) -> i32{
        self.program[self.pc] % 100
    }

    fn is_immediate_parameter(&self, index: usize) -> bool {
        let instruction = self.program[self.pc];
        match index {
            1 => ((instruction / 100) % 10) == 1,
            2 => ((instruction / 1000) % 10) == 1,
            //unlikely this'll get hit, since
            //currently all the 3-parameter opcodes write to the 3rd parameter
            3 => ((instruction / 10000) % 10) == 1,
            _ => panic!("UNEXPECTED PARAMETER '{}'", index),
        }
    }

    fn parameter(&self, index: usize) -> i32 {
        match self.is_immediate_parameter(index) {
            true => self.get_immediate(index),
            false => self.get_positional(index)
        }
    }

    fn get_immediate(&self, index: usize) -> i32 {
        self.program[self.pc + index]
    }

    fn get_positional(&self, index: usize) -> i32 {
        let x = self.get_immediate(index) as usize;
        self.program[self.pc + x]
    }

    fn set_positional(&mut self, index: usize, value: i32) {
        let x = self.get_immediate(index) as usize;
        self.program[self.pc + x] = value;
    }

    fn run_program(&mut self) {
        let mut input_index: usize = 0;
        loop {
            let opcode = self.get_opcode();
            match opcode {
                1 => self.add(),
                2 => self.multiply(),
                3 => self.input(),
                4 => self.output(),
                5 => self.jump_if_true(),
                6 => self.jump_if_false(),
                7 => self.less_than(),
                8 => self.equals(),
                99 => break, //HALT!
                _ => panic!("UNEXPECTED OPCODE '{}'", opcode)
            }
        }
    }

    fn add(&mut self) {
        let val1 = self.parameter(1);
        let val2 = self.parameter(2);
        let res = val1 + val2;
        self.print_debug(format!("{} + {} = {}", val1, val2, res));
        self.set_positional(3, res);
        self.pc += 4;
    }

    fn multiply(&mut self) {
        self.pc += 4;
    }

    fn input(&mut self) {
        self.pc += 2;
    }

    fn output(&mut self) {
        self.pc += 2;
    }

    fn jump_if_true(&mut self) {
        self.pc += 3;
    }

    fn jump_if_false(&mut self) {
        self.pc += 3;
    }

    fn less_than(&mut self) {
        self.pc += 4;
    }

    fn equals(&mut self) {
        self.pc += 4;
    }
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

        //println!("opcode {} ({}, {}, {})", opcode, is_immediate_1, is_immediate_2, is_immediate_3);

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
            5 => {
                let val1: i32 = if is_immediate_1 { program[pc+1] } else { program[(program[pc+1] as usize)]};
                let val2: i32 = if is_immediate_2 { program[pc+2] } else { program[(program[pc+2] as usize)]};
                if val1 != 0 {
                    //println!("{} <> 0, jump to {}", val1, val2);
                    pc = val2 as usize;
                } else {
                    pc += 3;
                }
            },
            6 => {
                let val1: i32 = if is_immediate_1 { program[pc+1] } else { program[(program[pc+1] as usize)]};
                let val2: i32 = if is_immediate_2 { program[pc+2] } else { program[(program[pc+2] as usize)]};
                if val1 == 0 {
                    //println!("{} == 0, jump to {}", val1, val2);
                    pc = val2 as usize;
                } else {
                    pc += 3;
                }
            },
            7 => {
                let val1: i32 = if is_immediate_1 { program[pc+1] } else { program[(program[pc+1] as usize)]};
                let val2: i32 = if is_immediate_2 { program[pc+2] } else { program[(program[pc+2] as usize)]};
                let pos3: usize = program[pc + 3] as usize;

                if val1 < val2 {
                    //println!("{} < {}, 1 -> {}", val1, val2, pos3);
                    program[pos3] = 1;
                } else {
                    //println!("{} !< {}, 0 -> {}", val1, val2, pos3);
                    program[pos3] = 0;
                }

                pc += 4;
            },
            8 => {
                let val1: i32 = if is_immediate_1 { program[pc+1] } else { program[(program[pc+1] as usize)]};
                let val2: i32 = if is_immediate_2 { program[pc+2] } else { program[(program[pc+2] as usize)]};
                let pos3: usize = program[pc + 3] as usize;

                if val1 == val2 {
                    //println!("{} == {}, 1 -> {}", val1, val2, pos3);
                    program[pos3] = 1;
                } else {
                    //println!("{} != {}, 0 -> {}", val1, val2, pos3);
                    program[pos3] = 0;
                }

                pc += 4;
            }
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
        let (prog, output) = run_program_new(program_spec, input_spec);
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
