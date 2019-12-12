use common::*;
use intcode::intcode::*;
use std::collections::HashMap;

fn main() {
    let program_spec = first_line(file_to_vec("input.txt".to_string()).unwrap());
    let canvas = run_emulator(program_spec.clone(), Colour::Black, false);
    let unique = canvas.keys().count();
    //println!("{:?}", &canvas);
    print_canvas(&canvas);
    println!("Painted {} squares at least once", unique);

    let canvas = run_emulator(program_spec.clone(), Colour::White, false);
    //println!("{:?}", &canvas);
    print_canvas(&canvas);
}

#[derive(Debug)]
enum Colour {
    Black,
    White,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug)]
enum NextAction {
    Paint,
    Move,
}

fn run_emulator(
    program_spec: String,
    starting_colour: Colour,
    debug: bool,
) -> HashMap<(i32, i32), Colour> {
    let mut canvas: HashMap<(i32, i32), Colour> = HashMap::new();
    let mut position = (0, 0);
    canvas.insert(position, starting_colour);
    let mut facing = Direction::Up;
    let mut emulator = prepare_emulator(program_spec, "".to_string(), debug);
    let mut next = NextAction::Paint;

    loop {
        match emulator.run_program() {
            RunSignal::Halt => break,
            RunSignal::NoInput => {
                //println!("Read");
                match canvas.get(&position) {
                    Some(c) => match c {
                        Colour::White => emulator.inputs.push(1),
                        Colour::Black => emulator.inputs.push(0),
                    },
                    None => emulator.inputs.push(0),
                }
            } //give it what the robot sees
            RunSignal::Output(value) => match next {
                NextAction::Paint => {
                    match value {
                        1 => {
                            //println!("Paint {:?} {:?}", position, Colour::White);
                            canvas.insert(position, Colour::White);
                        }
                        _ => {
                            //println!("Paint {:?} {:?}", position, Colour::Black);
                            canvas.insert(position, Colour::Black);
                        }
                    };
                    next = NextAction::Move;
                }
                NextAction::Move => {
                    match value {
                        0 => {
                            let (p, f) = calculate_robot_state(position, facing, Turn::Left);
                            position = p;
                            facing = f;
                        }
                        _ => {
                            let (p, f) = calculate_robot_state(position, facing, Turn::Right);
                            position = p;
                            facing = f;
                        }
                    }
                    //println!("Moved to {:?}, facing {:?}", position, facing);
                    next = NextAction::Paint;
                }
            }, //paint!
        }
    }
    //emulator.outputs.pop().unwrap()

    canvas
}

fn print_canvas(canvas: &HashMap<(i32, i32), Colour>) {
    let mut min_x = 1000;
    let mut max_x = -1000;
    let mut min_y = 1000;
    let mut max_y = -1000;

    for key in canvas.keys() {
        max_x = i32::max(max_x, key.0);
        min_x = i32::min(min_x, key.0);
        max_y = i32::max(max_y, key.1);
        min_y = i32::min(min_y, key.1);
    }

    //println!("{:?} -> {:?}",(min_x, min_y), (max_x, max_y));

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            match &canvas.get(&(x, y)) {
                Some(c) => match c {
                    Colour::White => print!("#"),
                    _ => print!(" "),
                },
                None => print!(" "),
            }
        }
        println!("");
    }
}

fn calculate_robot_state(
    position: (i32, i32),
    facing: Direction,
    turn: Turn,
) -> ((i32, i32), Direction) {
    let new_facing: Direction = match turn {
        Turn::Left => match facing {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        },
        Turn::Right => match facing {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        },
    };

    let new_position = match new_facing {
        Direction::Up => (position.0, position.1 + 1),
        Direction::Right => (position.0 + 1, position.1),
        Direction::Down => (position.0, position.1 - 1),
        Direction::Left => (position.0 - 1, position.1),
    };

    (new_position, new_facing)
}
