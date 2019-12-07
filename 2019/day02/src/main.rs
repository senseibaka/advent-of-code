use common::*;
use intcode::intcode::*;

fn main() {
    let program_spec = first_line(file_to_vec("input.txt".to_string()).unwrap());
    let part_1_result = run_emulator_verbs(program_spec.to_string(), 12, 2, false);
    println!("PART 1 OUTPUT: {}", part_1_result);
    let (noun, verb) = find_part_2(program_spec.to_string(), false);
    println!("PART 2 OUTPUT: {}", 100 * noun + verb);
}

fn find_part_2(program_spec: String, _debug: bool) -> (i32, i32) {
    for noun in 1..99 {
        for verb in 1..99 {
            if 19690720 == run_emulator_verbs(program_spec.to_string(), noun, verb, _debug) {
                return (noun, verb);
            }
        }
    }
    (-1, -1)
}

fn run_emulator_verbs(program_spec: String, noun: i32, verb: i32, _debug: bool) -> i32 {
    let mut emulator = Emulator::new(comma_separated_ints_to_vec(program_spec), vec![], _debug);
    emulator.program[1] = noun;
    emulator.program[2] = verb;
    emulator.run_program();
    let result = emulator.program[0];
    return result;
}
