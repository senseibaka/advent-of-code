pub mod intcode {
    pub fn prepare_emulator(program_spec: String, input_spec: String, debug: bool) -> Emulator {
        Emulator::new(
            common::comma_separated_i64_to_vec(&program_spec),
            common::comma_separated_i64_to_vec(&input_spec),
            debug,
        )
    }

    pub fn deconstruct_output(emulator: Emulator) -> (String, String) {
        (
            common::vec_to_comma_separated_i64(emulator.program),
            common::vec_to_comma_separated_i64(emulator.outputs),
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
    pub fn get_opcode(instruction: i64) -> i64 {
        instruction % 100
    }

    pub fn decode_parameter(instruction: i64, index: usize) -> Mode {
        match index {
            1 => decode_mode((instruction / 100) % 10),
            2 => decode_mode((instruction / 1000) % 10),
            //unlikely this'll get hit, since
            //currently all the 3-parameter opcodes write to the 3rd parameter
            3 => decode_mode((instruction / 10000) % 10),
            _ => panic!("UNEXPECTED PARAMETER '{}'", index),
        }
    }

    pub fn decode_mode(mode: i64) -> Mode {
        match mode {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => panic!("INVALID MODE {}", mode),
        }
    }

    pub fn add_i64_to_usize(a: i64, b: usize) -> usize {
        if a == 0 {
            return b;
        }
        match a.is_negative() {
            true => b.checked_sub(a.abs() as usize).unwrap(),
            false => b.checked_add(a as usize).unwrap(),
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub enum Mode {
        // 0
        Position,
        // 1
        Immediate,
        // 2
        Relative,
    }

    pub struct Emulator {
        pc: usize,
        relative_base: usize,
        debug: bool,
        pub program: Vec<i64>,
        pub inputs: Vec<i64>,
        pub outputs: Vec<i64>,
        is_halted: bool,
    }

    #[derive(Debug)]
    pub enum RunSignal {
        Halt,
        NoInput,
        Output(i64),
    }

    impl Emulator {
        pub fn new(program: Vec<i64>, inputs: Vec<i64>, debug: bool) -> Emulator {
            let mut memory = program.clone();
            memory.append(&mut vec![0; 10000]);
            Emulator {
                pc: 0,
                relative_base: 0,
                debug,
                program: memory,
                inputs,
                outputs: vec![],
                is_halted: false,
            }
        }

        fn print_debug(&self, statement: String) {
            if self.debug {
                println!("{}", statement);
            }
        }

        fn get_opcode(&self) -> i64 {
            get_opcode(self.program[self.pc])
        }

        fn decode_parameter(&self, index: usize) -> Mode {
            decode_parameter(self.program[self.pc], index)
        }

        fn get_parameter(&self, index: usize) -> i64 {
            match self.decode_parameter(index) {
                Mode::Position => self.get_positional(index),
                Mode::Immediate => self.get_immediate(index),
                Mode::Relative => self.get_relative(index),
            }
        }

        fn set_parameter(&mut self, index: usize, value: i64) {
            match self.decode_parameter(index) {
                Mode::Position => self.set_positional(index, value),
                Mode::Immediate => panic!("IMMEDIATE MODE NOT SUPPORTED FOR WRITING VALUES"),
                Mode::Relative => self.set_relative(index, value)
            }
        }

        fn get_immediate(&self, index: usize) -> i64 {
            self.print_debug(format!("get_immediate {}", index));
            self.program[self.pc + index]
        }

        fn get_positional(&self, index: usize) -> i64 {
            let x = self.program[self.pc + index] as usize;
            self.print_debug(format!("get_positional [{}] -> {}", index, x));
            self.program[x]
        }

        fn get_relative(&self, index: usize) -> i64 {
            let x = self.program[self.pc + index];
            let oldrel = self.relative_base.clone();
            let relative_index = add_i64_to_usize(x, self.relative_base);
            self.print_debug(format!(
                "get_relative [{} + {}] -> {}",
                oldrel, x, relative_index
            ));
            self.program[relative_index]
        }

        fn set_positional(&mut self, index: usize, value: i64) {
            self.print_debug(format!("set_positional [{}] <- {}", index, value));
            let x = self.program[self.pc + index] as usize;
            self.program[x] = value;
        }

        fn set_relative(&mut self, index: usize, value: i64) {
            let x = self.program[self.pc + index];
            let oldrel = self.relative_base.clone();
            let relative_index = add_i64_to_usize(x, self.relative_base);
            self.print_debug(format!(
                "set_relative [{} + {} = {}] <- {}",
                oldrel, x, relative_index, value
            ));
            self.program[relative_index] = value;
        }

        pub fn run_program(&mut self) -> RunSignal {
            if self.is_halted {
                return RunSignal::Halt;
            }
            loop {
                let opcode = self.get_opcode();
                /* self.print_debug(format!(
                    "OPCODE {}",
                    opcode
                )); */
                match opcode {
                    1 => self.add(),
                    2 => self.multiply(),
                    3 => {
                        if !self.input() {
                            return RunSignal::NoInput;
                        }
                    }
                    4 => {
                        self.output();
                        let last_output = self.outputs[self.outputs.len() - 1];
                        return RunSignal::Output(last_output);
                    }
                    5 => self.jump_if_true(),
                    6 => self.jump_if_false(),
                    7 => self.less_than(),
                    8 => self.equals(),
                    9 => self.adjust_relative_base(),
                    99 => {
                        self.is_halted = true;
                        return RunSignal::Halt; //HALT!
                    }
                    _ => panic!("UNEXPECTED OPCODE '{}'", opcode),
                }
            }
        }

        fn add(&mut self) {
            let val1 = self.get_parameter(1);
            let val2 = self.get_parameter(2);
            let res = val1 + val2;
            self.print_debug(format!("ADD {} + {} = {}", val1, val2, res));
            self.set_parameter(3, res);
            self.pc += 4;
        }

        fn multiply(&mut self) {
            let val1 = self.get_parameter(1);
            let val2 = self.get_parameter(2);
            let res = val1 * val2;
            self.print_debug(format!("MUL {} * {} = {}", val1, val2, res));
            self.set_parameter(3, res);
            self.pc += 4;
        }

        fn input(&mut self) -> bool {
            if self.inputs.len() == 0 {
                return false; //signal we need more input!
            }
            let val: i64 = self.inputs.remove(0);
            self.print_debug(format!("INPUT {}", val));
            self.set_parameter(1, val);
            self.pc += 2;
            return true;
        }

        fn output(&mut self) {
            let val = self.get_parameter(1);
            self.print_debug(format!("OUTPUT {}", val));
            self.outputs.push(val);
            self.pc += 2;
        }

        fn jump_if_true(&mut self) {
            let val1 = self.get_parameter(1);
            let val2 = self.get_parameter(2);
            self.print_debug(format!("JIT {} != 0 ? jump to {}", val1, val2));
            if val1 != 0 {
                self.pc = val2 as usize;
            } else {
                self.pc += 3;
            }
        }

        fn jump_if_false(&mut self) {
            let val1 = self.get_parameter(1);
            let val2 = self.get_parameter(2);
            self.print_debug(format!("JIF {} == 0 ? jump to {}", val1, val2));
            if val1 == 0 {
                self.pc = val2 as usize;
            } else {
                self.pc += 3;
            }
        }

        fn less_than(&mut self) {
            let val1 = self.get_parameter(1);
            let val2 = self.get_parameter(2);
            self.print_debug(format!(
                "LT {} < {} ?",
                val1, val2
            ));
            if val1 < val2 {
                self.set_parameter(3, 1);
            } else {
                self.set_parameter(3, 0);
            }
            self.pc += 4;
        }

        fn equals(&mut self) {
            let val1 = self.get_parameter(1);
            let val2 = self.get_parameter(2);
            self.print_debug(format!(
                "EQ {} == {}",
                val1, val2
            ));
            if val1 == val2 {
                self.set_parameter(3, 1);
            } else {
                self.set_parameter(3, 0);
            }
            self.pc += 4;
        }

        fn adjust_relative_base(&mut self) {
            let val1 = self.get_parameter(1);
            let oldrel = self.relative_base.clone();
            self.relative_base = add_i64_to_usize(val1, self.relative_base);
            self.print_debug(format!(
                "ADJUST REL {} + {} = {}",
                oldrel, val1, self.relative_base
            ));
            self.pc += 2;
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
    fn run_program_works_6() {
        run_program_case(
            "3,0,4,0,99".to_string(),
            "1".to_string(),
            "1,0,4,0,99".to_string(),
            "1".to_string(),
        )
    }

    #[test]
    fn run_program_works_7() {
        run_program_case(
            "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".to_string(),
            "".to_string(),
            "".to_string(),
            "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".to_string(),
        )
    }

    #[test]
    fn run_program_works_8() {
        run_program_case(
            "1102,34915192,34915192,7,4,7,99,0".to_string(),
            "".to_string(),
            "".to_string(),
            "1219070632396864".to_string(),
        )
    }

    #[test]
    fn run_program_works_9() {
        run_program_case(
            "104,1125899906842624,99".to_string(),
            "".to_string(),
            "".to_string(),
            "1125899906842624".to_string(),
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
        loop {
            match emulator.run_program() {
                RunSignal::Halt => break,
                _ => continue,
            }
        }
        let (prog, output) = deconstruct_output(emulator);

        assert!(prog.starts_with(&expected_program));
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
    fn decode_parameter_works() {
        assert_eq!(Mode::Position, decode_parameter(1, 1));
        assert_eq!(Mode::Position, decode_parameter(1, 2));
        assert_eq!(Mode::Position, decode_parameter(1, 3));
        assert_eq!(Mode::Position, decode_parameter(2, 1));
        assert_eq!(Mode::Position, decode_parameter(2, 2));
        assert_eq!(Mode::Position, decode_parameter(2, 3));
        assert_eq!(Mode::Position, decode_parameter(10002, 1));
        assert_eq!(Mode::Position, decode_parameter(10002, 2));
        assert_eq!(Mode::Immediate, decode_parameter(102, 1));
        assert_eq!(Mode::Immediate, decode_parameter(1002, 2));
        assert_eq!(Mode::Immediate, decode_parameter(10002, 3));
        assert_eq!(Mode::Relative, decode_parameter(202, 1));
        assert_eq!(Mode::Relative, decode_parameter(2002, 2));
        assert_eq!(Mode::Relative, decode_parameter(20002, 3));
        assert_eq!(Mode::Immediate, decode_parameter(11102, 1));
        assert_eq!(Mode::Immediate, decode_parameter(11102, 2));
        assert_eq!(Mode::Immediate, decode_parameter(11102, 3));
        assert_eq!(Mode::Immediate, decode_parameter(1102, 1));
        assert_eq!(Mode::Immediate, decode_parameter(1102, 2));
        assert_eq!(Mode::Position, decode_parameter(1102, 3));
    }

    #[test]
    fn add_i64_to_usize_works() {
        assert_eq!(0, add_i64_to_usize(-1, 1));
        assert_eq!(1, add_i64_to_usize(1, 0));
    }
}
