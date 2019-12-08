use itertools::Itertools;

fn main() {
    println!("{:?}", verify_image(&image_input(), 25, 6));
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

fn image_input() -> Vec<u8> {
    use std::fs;
    fs::read("input.txt").unwrap()
}
