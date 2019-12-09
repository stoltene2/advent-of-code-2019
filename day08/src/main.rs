use itertools::Itertools;

fn main() {
    let r = input_to_layer_vec(&image_input(), 25, 6);

    // Print the "secret" message. Use ' ' for white and * for black
    for j in 0..6 {
        for i in 0..25 {
            if r[i+(j*25)] == 0 {
                print!("{}", " ");
            } else {
                print!("{}", "*");
            }
        }
        println!("");
    }
}

fn verify_image(image: &Vec<u8>, x: usize, y: usize) -> u32 {
    let mut rows: Vec<_> = Vec::new();
    let mut layers: Vec<_> = Vec::new();

    for row in &image.into_iter().chunks(x) {
        let vec = row.collect_vec();
        rows.push(vec);
    }

    for layer in &rows.into_iter().chunks(y) {
        layers.push(layer.collect_vec());
    }

    let min = layers.into_iter()
        .map(|l| {
            let s = layer_summary(l);
            println!("{:?}", &s);
            s
        })
        .filter(|(a, b, c)| *a != 0 && *b!=0 && *c!=0)
        .min_by(|a, b| (a.0).cmp(&b.0))
        .unwrap();

    min.1 * min.2
}

fn layer_summary(layer: Vec<Vec<&u8>>) -> (u32, u32, u32) {
    layer.into_iter().fold((0,0,0), |(a, b, c), row| {
        let (a1, b1, c1) = count_0_1_2(row.clone());
        (a + a1, b + b1, c + c1)
    })
}

// This receives an array of bytes. We need to match on bytes not numbers
fn count_0_1_2(row: Vec<&u8>) -> (u32, u32, u32) {
    row.iter().fold((0, 0, 0), |(a, b, c), n| {
        match n {
            b'0' => (a+1, b, c),
            b'1' => (a, b+1, c),
            b'2' => (a, b, c+1),
            _ => (a, b, c)
        }
    })
}

// Takes input and makes a Vec of Vecs where each row is the flattened layer
fn input_to_layer_vec(input: &Vec<u8>, row_size: usize, column_size: usize) -> Vec<u8> {
    let image_size = row_size * column_size;
    let mut layers: Vec<_> = Vec::new();

    for layer in &input.into_iter().chunks(image_size) {
        let vec = layer.collect_vec();
        layers.push(vec);
    }

    let mut output = Vec::new();

    for col in 0..image_size {
        let mut nth_element_from_each_layer = Vec::new();

        for layer_num in 0..layers.len() {
            nth_element_from_each_layer.push(*layers[layer_num][col]);
        }

        output.push(first_non_transparent(nth_element_from_each_layer).unwrap_or(2));
    }

    println!("output size: {}", &output.len());
    output
}

fn first_non_transparent(nums: Vec<u8>) -> Option<u8> {
    let ns: Vec<u8> = nums.into_iter().skip_while(|&x| x == 2).collect();

    match ns.get(0) {
        Some(val) => Some(*val),
        None => None,
    }
}

// Returns image_input so that it is 0, 1, 2. Instead of byte order of 48, 49, 50
fn image_input() -> Vec<u8> {
    use std::fs;
    let mut input = fs::read("input.txt").unwrap();
    input.pop();
    input.into_iter().map(|n| n-48).collect()
}
