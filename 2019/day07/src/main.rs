use common::*;
use intcode::intcode::*;
use itertools::Itertools;
use std::sync::mpsc::*;

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
    println!("PART 1 MAX: {:?} -> {}", max_combo, &max_signal)
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

    let (input_send, input_recv) = channel();
    let (output_send, output_recv) = channel();

    let mut emulator = Emulator::new(
        common::comma_separated_ints_to_vec(program_spec),
        input_recv,
        output_send,
        debug,
    );
    
    input_send.send(phase).unwrap();
    input_send.send(signal).unwrap();

    emulator.run_program();

    let result = output_recv.recv().unwrap();

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
