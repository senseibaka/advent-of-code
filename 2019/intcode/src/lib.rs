pub mod intcode {
    pub fn prepare_emulator(program_spec: String, input_spec: String, debug: bool) -> Emulator {
        Emulator::new(
            common::comma_separated_ints_to_vec(&program_spec),
            common::comma_separated_ints_to_vec(&input_spec),
            debug,
        )
    }
    
    pub fn deconstruct_output(emulator: Emulator) -> (String, String) {
        (
            common::vec_to_comma_separated_ints(emulator.program),
            common::vec_to_comma_separated_ints(emulator.outputs)
        )
    }

    /*
        ABCDE
        DE = opcode
        C = p1 mode
        B = p2 mode
        A = p3 mode
        Parameters that an instruction writes to will never be in immediate mode.
    */
    // have get_opcode and is_immediate_parameter outside Emulator for easy testing
    pub fn get_opcode(instruction: i32) -> i32 {
        instruction % 100
    }

    pub fn is_immediate_parameter(instruction: i32, index: usize) -> bool {
        match index {
            1 => ((instruction / 100) % 10) == 1,
            2 => ((instruction / 1000) % 10) == 1,
            //unlikely this'll get hit, since
            //currently all the 3-parameter opcodes write to the 3rd parameter
            3 => ((instruction / 10000) % 10) == 1,
            _ => panic!("UNEXPECTED PARAMETER '{}'", index),
        }
    }

    pub struct Emulator {
        pc: usize,
        debug: bool,
        pub program: Vec<i32>,
        pub inputs: Vec<i32>, //note: treat this like a stack
        pub outputs: Vec<i32>,
    }

    impl Emulator {
        pub fn new(program: Vec<i32>, inputs: Vec<i32>, debug: bool) -> Emulator {
            Emulator {
                pc: 0,
                debug,
                program,
                inputs,
                outputs: vec![],
            }
        }

        fn print_debug(&self, statement: String) {
            if self.debug {
                println!("{}", statement);
            }
        }

        fn get_opcode(&self) -> i32 {
            get_opcode(self.program[self.pc])
        }

        fn is_immediate_parameter(&self, index: usize) -> bool {
            is_immediate_parameter(self.program[self.pc], index)
        }

        fn parameter(&self, index: usize) -> i32 {
            match self.is_immediate_parameter(index) {
                true => self.get_immediate(index),
                false => self.get_positional(index),
            }
        }

        fn get_immediate(&self, index: usize) -> i32 {
            self.print_debug(format!("get_immediate {}", index));
            
            self.program[self.pc + index]
        }

        fn get_positional(&self, index: usize) -> i32 {
            let x = self.get_immediate(index) as usize;
            self.print_debug(format!("get_positional {} -> {}", index, x));
            
            self.program[x]
        }

        pub fn run_program(&mut self) {
            loop {
                let opcode = self.get_opcode();
                //self.print_debug(format!("OPCODE {}", opcode));
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
                    _ => panic!("UNEXPECTED OPCODE '{}'", opcode),
                }
            }
        }

        fn add(&mut self) {
            let val1 = self.parameter(1);
            let val2 = self.parameter(2);
            let dest = self.program[self.pc + 3] as usize;
            let res = val1 + val2;
            self.print_debug(format!("ADD {} + {} = {} -> {}", val1, val2, res, dest));
            self.program[dest] = res;
            self.pc += 4;
        }

        fn multiply(&mut self) {
            let val1 = self.parameter(1);
            let val2 = self.parameter(2);
            let dest = self.program[self.pc + 3] as usize;
            let res = val1 * val2;
            self.print_debug(format!("MUL {} * {} = {} -> {}", val1, val2, res, dest));
            self.program[dest] = res;
            self.pc += 4;
        }

        fn input(&mut self) {
            if self.inputs.len() == 0 {
                panic!("TRY TO READ INPUT BUT THERE IS NONE. Instruction {}", self.program[self.pc]);
            }
            let val: i32 = self.inputs.remove(0);
            let dest = self.program[self.pc + 1] as usize;
            self.print_debug(format!("INPUT {} -> {}", val, dest));
            self.program[dest] = val;
            self.pc += 2;
        }

        fn output(&mut self) {
            let val = self.parameter(1);
            self.print_debug(format!("OUTPUT {}", val));
            self.outputs.push(val);
            self.pc += 2;
        }

        fn jump_if_true(&mut self) {
            let val1 = self.parameter(1);
            let val2 = self.parameter(2);
            self.print_debug(format!("JIT {} != 0 ? jump to {}", val1, val2));
            if val1 != 0 {
                self.pc = val2 as usize;
            } else {
                self.pc += 3;
            }
        }

        fn jump_if_false(&mut self) {
            let val1 = self.parameter(1);
            let val2 = self.parameter(2);
            self.print_debug(format!("JIF {} == 0 ? jump to {}", val1, val2));
            if val1 == 0 {
                self.pc = val2 as usize;
            } else {
                self.pc += 3;
            }
        }

        fn less_than(&mut self) {
            let val1 = self.parameter(1);
            let val2 = self.parameter(2);
            let dest = self.program[self.pc + 3] as usize;
            self.print_debug(format!(
                "LT {} < {} ? 1 -> {} : 0 -> {}",
                val1, val2, dest, dest
            ));
            if val1 < val2 {
                self.program[dest] = 1;
            } else {
                self.program[dest] = 0;
            }
            self.pc += 4;
        }

        fn equals(&mut self) {
            let val1 = self.parameter(1);
            let val2 = self.parameter(2);
            let dest = self.program[self.pc + 3] as usize;
            self.print_debug(format!(
                "EQ {} == {} ? 1 -> {} : 0 -> {}",
                val1, val2, dest, dest
            ));
            if val1 == val2 {
                self.program[dest] = 1;
            } else {
                self.program[dest] = 0;
            }
            self.pc += 4;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::intcode::*;

    #[test]
    fn run_program_works_1() {
        run_program_case_no_io("1,0,0,0,99".to_string(), "2,0,0,0,99".to_string());
    }
    #[test]
    fn run_program_works_2() {
        run_program_case_no_io("2,3,0,3,99".to_string(), "2,3,0,6,99".to_string());
    }
    #[test]
    fn run_program_works_3() {
        run_program_case_no_io("2,4,4,5,99,0".to_string(), "2,4,4,5,99,9801".to_string());
    }
    #[test]
    fn run_program_works_4() {
        run_program_case_no_io(
            "1,1,1,4,99,5,6,0,99".to_string(),
            "30,1,1,4,2,5,6,0,99".to_string(),
        );
    }
    #[test]
    fn run_program_works_5() {
        run_program_case_no_io("1002,4,3,4,33".to_string(), "1002,4,3,4,99".to_string());
    }

    #[test]
    fn run_program_with_io_works() {
        run_program_case(
            "3,0,4,0,99".to_string(),
            "1".to_string(),
            "1,0,4,0,99".to_string(),
            "1".to_string(),
        )
    }

    fn run_program_case_no_io(program_spec: String, expected_program: String) {
        run_program_case(
            program_spec,
            "".to_string(),
            expected_program,
            "".to_string(),
        );
    }

    fn run_program_case(
        program_spec: String,
        input_spec: String,
        expected_program: String,
        expected_output: String,
    ) {
        let mut emulator = prepare_emulator(program_spec, input_spec, true);
        emulator.run_program();
        let (prog, output) = deconstruct_output(emulator);
        assert_eq!(expected_program, prog);
        assert_eq!(expected_output, output);
    }

    #[test]
    fn get_opcode_works() {
        assert_eq!(1, get_opcode(1));
        assert_eq!(2, get_opcode(102));
        assert_eq!(2, get_opcode(1002));
        assert_eq!(2, get_opcode(10002));
        assert_eq!(2, get_opcode(11102));
    }

    #[test]
    fn is_immediate_parameter_works() {
        assert_eq!(false, is_immediate_parameter(1, 1));
        assert_eq!(false, is_immediate_parameter(1, 2));
        assert_eq!(false, is_immediate_parameter(1, 3));

        assert_eq!(true, is_immediate_parameter(102, 1));
        assert_eq!(false, is_immediate_parameter(102, 2));
        assert_eq!(false, is_immediate_parameter(102, 3));

        assert_eq!(false, is_immediate_parameter(1002, 1));
        assert_eq!(true, is_immediate_parameter(1002, 2));
        assert_eq!(false, is_immediate_parameter(1002, 3));

        assert_eq!(false, is_immediate_parameter(10002, 1));
        assert_eq!(false, is_immediate_parameter(10002, 2));
        assert_eq!(true, is_immediate_parameter(10002, 3));

        assert_eq!(true, is_immediate_parameter(11102, 1));
        assert_eq!(true, is_immediate_parameter(11102, 2));
        assert_eq!(true, is_immediate_parameter(11102, 3));
    }
}
