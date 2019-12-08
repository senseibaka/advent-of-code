use common::*;
use std::iter::Iterator;

fn main() {
    const WIDTH: usize = 25;
    const HEIGHT: usize = 6;
    const CHUNK_SIZE: usize = WIDTH * HEIGHT;
    let input = first_line(file_to_vec("input.txt".to_string()).unwrap());
    let mut fewest_zeroes = std::u32::MAX;
    let mut fewest_result = 0;
    let mut fewest_layer: Vec<u32> = vec![];
    let layers = layerize(&input, CHUNK_SIZE);

    for layer in &layers {
        let (zeroes, result) = part_1_process_layer(&layer);
        if zeroes < fewest_zeroes {
            fewest_zeroes = zeroes;
            fewest_result = result;
            fewest_layer = layer.clone();
        }
    }
    println!("PART 1: {} zeroes -> {}", fewest_zeroes, fewest_result);
    print_layer(&fewest_layer, WIDTH);

    let flattened = flatten_layers(&layers, CHUNK_SIZE);
    println!("PART 2:");
    print_layer(&flattened, WIDTH);
}

fn layerize(input: &String, chunk_size: usize) -> Vec<Vec<u32>> {
    input
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
        .chunks(chunk_size)
        .map(|x| x.to_vec())
        .collect()
}

fn flatten_layers(layers: &Vec<Vec<u32>>, chunk_size: usize) -> Vec<u32> {
    let mut canvas: Vec<u32> = vec![];
    for l in (0..layers.len()).rev() {
        let layer = layers.get(l).unwrap();
        if canvas.len() == 0 {
            for pixel in layer {
                canvas.push(*pixel);
            }
            continue;
        }
        for p in 0..chunk_size {
            match layer[p] {
                2 => continue, //transparent, do nothing
                pixel => canvas[p] = pixel,
            }
        }
    }
    canvas
}

fn print_layer(layer: &Vec<u32>, width: usize) {
    let mut x = 0;

    for pixel in layer {
        if x == width {
            x = 0;
            println!("");
        }

        match pixel {
            1 => print!("#"),
            _ => print!(" "),
        }

        x += 1;
    }
    println!("");
}

fn part_1_process_layer(layer: &Vec<u32>) -> (u32, u32) {
    let mut zeroes = 0u32;
    let mut ones = 0u32;
    let mut twos = 0u32;

    for x in layer {
        match x {
            0 => zeroes += 1,
            1 => ones += 1,
            2 => twos += 1,
            _ => continue,
        }
    }

    (zeroes, ones * twos)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layerize_works() {
        let layers = layerize(&"123456789012".to_string(), 6);
        for layer in &layers {
            println!("Layer:");
            print_layer(&layer, 3);
        }

        assert_eq!(vec![vec![1, 2, 3, 4, 5, 6], vec![7, 8, 9, 0, 1, 2]], layers);
    }

    #[test]
    fn flatten_layers_works() {
        let layers = layerize(&"0222112222120000".to_string(), 4);
        for layer in &layers {
            println!("Layer:");
            print_layer(&layer, 2);
        }
        let flattened = flatten_layers(&layers, 4);

        assert_eq!(vec![0, 1, 1, 0], flattened)
    }
}
