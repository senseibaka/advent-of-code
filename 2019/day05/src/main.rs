use common::*;
use intcode::intcode::*;

fn main() {
    let program_spec = first_line(file_to_vec("input.txt".to_string()).unwrap());
    println!(
        "PART 1 OUTPUT: {}",
        run_emulator(program_spec.to_string(), "1".to_string(), false)
    );
    println!(
        "PART 2 OUTPUT: {}",
        run_emulator(program_spec.to_string(), "5".to_string(), false)
    );
}

fn run_emulator(program_spec: String, input_spec: String, debug: bool) -> i32 {
    let mut emulator = prepare_emulator(program_spec, input_spec, debug);
    loop {
        match emulator.run_program() {
            RunSignal::Halt => break,
            _ => continue,
        }
    }
    return emulator.outputs.pop().unwrap();
}
