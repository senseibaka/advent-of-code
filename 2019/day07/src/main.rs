use common::*;
use intcode::intcode::*;
use itertools::Itertools;

fn main() {
    let program_spec = first_line(file_to_vec("input.txt".to_string()).unwrap());
    let mut max_signal = std::i32::MIN;
    let mut max_combo = vec![0,0,0,0,0];
    
    for combo in (0..=4).permutations(5) {
        let signal = run_amp_sequence(&program_spec, &combo, false);
        if signal > max_signal {
            max_signal = signal;
            max_combo = combo;
            //println!("New max found: {:?} -> {}", max_combo, &max_signal);
        }
    }
    println!("PART 1 MAX: {:?} -> {}", max_combo, &max_signal);

    max_signal = std::i32::MIN;
    for combo in (5..=9).permutations(5) {
        let signal = run_amp_feedback_sequence(&program_spec, &combo, false);
        if signal > max_signal {
            max_signal = signal;
            max_combo = combo;
            //println!("New max found: {:?} -> {}", max_combo, &max_signal);
        }
    }
    println!("PART 2 MAX: {:?} -> {}", max_combo, &max_signal);
}

fn run_amp_feedback_sequence(program_spec: &String, phases: &Vec<i32>, debug: bool) -> i32 {
    //setup
    let mut emulators: Vec<Emulator> = vec![];
    let mut first = true;
    for phase in phases {
        let mut emulator = Emulator::new(
            common::comma_separated_ints_to_vec(program_spec),
            vec![*phase],
            debug,
        );
        if first {
            emulator.inputs.push(0); // seeding signal
            first = false;
        }
        emulators.push(emulator);
    }
    
    let mut current_emulator = 0;
    loop {
        match emulators[current_emulator].run_program() {
            RunSignal::Halt => {
                if current_emulator == emulators.len() - 1 {
                    return emulators[current_emulator].outputs.pop().unwrap();
                }
            },
            RunSignal::Output(value) => {
                let next_emulator = ( current_emulator + 1) % 5;
                emulators[next_emulator].inputs.push(value);
            }
            //RunSignal::NoInput =>
            _ => continue // if input is requested, then we need to move on to the next emulator. eventually we'll loop around and output should feed to input, and it'll resolve
        }
        current_emulator = ( current_emulator + 1) % emulators.len();
    }
}

fn run_amp_sequence(program_spec: &String, phases: &Vec<i32>, debug: bool) -> i32 {
    //println!("### RUN AMP SEQUENCE {:?}", phases);
    let mut signal = 0;

    for phase in phases {
        signal = run_amp(&program_spec, *phase, signal, debug);
    }

    //println!("### RUN AMP SEQUENCE {:?} -> {}", &phases, signal);
    return signal;
}

fn run_amp(program_spec: &String, phase: i32, signal: i32, debug: bool) -> i32 {
    //println!("### RUN AMP p:{}, s:{}", phase, signal);

    let mut emulator = Emulator::new(
        common::comma_separated_ints_to_vec(program_spec),
        vec![],
        debug,
    );
    
    emulator.inputs.append(&mut vec![phase, signal]);

    loop {
        match emulator.run_program() {
            RunSignal::Halt => break,
            _ => continue
        }
    }
    
    let result = emulator.outputs.pop().unwrap();

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_amp_runs() {
        let signal = run_amp(
            &"3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0".to_string(),
            4,
            0,
            true,
        );
        println!("SIGNAL: {}", signal);
        assert_eq!(4, signal);
    }

    #[test]
    fn run_amp_sequence_works_1() {
        //IN([15])
        //IN([16])
        //MUL([16], 10, [16])
        //ADD([16], [15], [15])
        //OUT([15])
        //HALT
        //(0 * 10) + 4 => 4
        //(4 * 10) + 3 => 43
        //(43 * 10) + 2 => 432
        //(432 * 10) + 1 => 4321
        //(4321 * 10) + 0 => 43210
        let signal = run_amp_sequence(
            &"3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0".to_string(),
            vec![4, 3, 2, 1, 0],
            true,
        );
        assert_eq!(43210, signal);
    }

    #[test]
    fn run_amp_sequence_works_2() {
        let signal = run_amp_sequence(
            &"3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0".to_string(),
            vec![0, 1, 2, 3, 4],
            false,
        );
        assert_eq!(54321, signal);
    }

    #[test]
    fn run_amp_sequence_works_3() {
        let signal = run_amp_sequence(&"3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0".to_string(), vec![1,0,4,3,2], false);
        assert_eq!(65210, signal);
    }
}
