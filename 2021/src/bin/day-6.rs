fn main() {
    println!("Hello there");

    let sample_input: Vec<u64> = vec![3, 4, 3, 1, 2];

    let mut result: Vec<u64> = sample_input.clone();

    for _i in 1..=80 {
        result = next_generation(result);
    }

    assert_eq!(5934, result.len());

    // Problem 1
    let mut result = input();

    for _i in 1..=80 {
        result = next_generation(result);
    }

    println!("Solution 1: {}", result.len());
    assert_eq!(380612, result.len());

    //Problem 2
    let mut result = build_model(input());

    for _i in 1..=256 {
        result = next_generation_p2(result);
    }

    let answer: u64 = result.iter().fold(0, |acc, val| acc + val);
    println!("Solution 2: {:?}", answer);
    assert_eq!(1710166656900, answer);
}

fn next_generation(mut current: Vec<u64>) -> Vec<u64> {
    for i in 0..current.len() {
        if current[i] != 0 {
            current[i] = current[i] - 1;
        } else {
            current.push(8);
            current[i] = 6;
        }
    }
    current
}

fn next_generation_p2(mut current: [u64; 9]) -> [u64; 9] {
    let spawns = current[0];

    for i in 0..8 {
        current[i] = current[i + 1];
    }

    current[6] += spawns;
    current[8] = spawns;
    current
}

fn build_model(data: Vec<u64>) -> [u64; 9] {
    data.iter()
        .fold([0, 0, 0, 0, 0, 0, 0, 0, 0], |mut res, days| {
            res[*days as usize] += 1;
            res
        })
}

fn input() -> Vec<u64> {
    vec![
        3, 1, 4, 2, 1, 1, 1, 1, 1, 1, 1, 4, 1, 4, 1, 2, 1, 1, 2, 1, 3, 4, 5, 1, 1, 4, 1, 3, 3, 1,
        1, 1, 1, 3, 3, 1, 3, 3, 1, 5, 5, 1, 1, 3, 1, 1, 2, 1, 1, 1, 3, 1, 4, 3, 2, 1, 4, 3, 3, 1,
        1, 1, 1, 5, 1, 4, 1, 1, 1, 4, 1, 4, 4, 1, 5, 1, 1, 4, 5, 1, 1, 2, 1, 1, 1, 4, 1, 2, 1, 1,
        1, 1, 1, 1, 5, 1, 3, 1, 1, 4, 4, 1, 1, 5, 1, 2, 1, 1, 1, 1, 5, 1, 3, 1, 1, 1, 2, 2, 1, 4,
        1, 3, 1, 4, 1, 2, 1, 1, 1, 1, 1, 3, 2, 5, 4, 4, 1, 3, 2, 1, 4, 1, 3, 1, 1, 1, 2, 1, 1, 5,
        1, 2, 1, 1, 1, 2, 1, 4, 3, 1, 1, 1, 4, 1, 1, 1, 1, 1, 2, 2, 1, 1, 5, 1, 1, 3, 1, 2, 5, 5,
        1, 4, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 4, 5, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 4, 4, 1, 1, 4, 1,
        3, 4, 1, 5, 4, 2, 5, 1, 2, 1, 1, 1, 1, 1, 1, 4, 3, 2, 1, 1, 3, 2, 5, 2, 5, 5, 1, 3, 1, 2,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 1, 1, 1, 3, 1, 4, 1, 4, 2, 1, 3, 4, 1, 1, 1, 2, 3, 1, 1, 1,
        4, 1, 2, 5, 1, 2, 1, 5, 1, 1, 2, 1, 2, 1, 1, 1, 1, 4, 3, 4, 1, 5, 5, 4, 1, 1, 5, 2, 1, 3,
    ]
}
