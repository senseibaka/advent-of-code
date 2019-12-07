use common::*;
use intcode::intcode::*;

fn main() {
    let program_spec = first_line(file_to_vec("input.txt".to_string()).unwrap());
    let (_, output) = run_emulator(program_spec.to_string(), "1".to_string(), false);
    println!("PART 1 OUTPUT: {}", output);
    let (_, output) = run_emulator(program_spec.to_string(), "5".to_string(), false);
    println!("PART 2 OUTPUT: {}", output);
}

fn run_emulator(program_spec: String, input_spec: String, debug: bool) -> (String, String) {
    let mut emulator = prepare_emulator(program_spec, input_spec, debug);
    emulator.run_program();
    return deconstruct_output(emulator);
}
