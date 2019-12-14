use std::u32;
use std::usize;

type Layer = Vec<Vec<u32>>;
type LayerVec = Vec<Layer>;

pub fn start(input: &str) {
    let processed = process_input(input);
    let layers = into_layers(&processed, 25, 6);

    validate_image(&layers);
    print_image(&layers);
}

fn process_input(input: &str) -> Vec<u32> {
    let out: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).expect("Invalid digit."))
        .collect();
    return out;
}

fn into_layers(raw: &Vec<u32>, row_len: usize, column_len: usize) -> LayerVec {
    let mut layers: LayerVec = Vec::new();
    let mut idx = 0;

    while idx < raw.len() {
        let mut layer: Layer = Vec::new();
        for _ in 0..column_len {
            let mut row: Vec<u32> = Vec::new();
            for _ in 0..row_len {
                row.push(raw[idx]);
                idx += 1;
            }
            layer.push(row);
        }
        layers.push(layer);
    }

    return layers;
}

fn validate_image(layers: &LayerVec) {
    let mut lowest_zeroes = u32::MAX;
    let mut validation_number = 0;

    for layer in layers.iter() {
        let mut zero_count = 0;
        let mut one_count = 0;
        let mut two_count = 0;
        for row in layer.iter() {
            for digit in row.iter() {
                match *digit {
                    0 => zero_count += 1,
                    1 => one_count += 1,
                    2 => two_count += 1,
                    _ => panic!("Invalid digit: {}", *digit),
                }
            }
        }

        if zero_count < lowest_zeroes {
            lowest_zeroes = zero_count;
            validation_number = one_count * two_count;
        }
    }

    println!("Validation number: {}", validation_number);
}

fn print_image(layers: &LayerVec) {
    let mut image = layers.first().unwrap().clone();

    for layer in layers.iter() {
        for (i, row) in layer.iter().enumerate() {
            for (j, digit) in row.iter().enumerate() {
                if image[i][j] == 2 {
                    image[i][j] = *digit;
                }
            }
        }
    }

    println!("Image: ");
    for row in image.iter() {
        println!("{:?}", row);
    }
}
