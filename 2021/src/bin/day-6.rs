fn main() {
    println!("Hello there");

    let sample_input: Vec<u8> = vec![3, 4, 3, 1, 2];

    let mut result: Vec<u8> = sample_input.clone();

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
    let mut result = input();

    for i in 1..=256 {
        if i % 10 == 0 {
            println!("Iteration: {}", i);
        }

        result = next_generation(result);
    }

    println!("Solution 2: {}", result.len());
    //       assert_eq!(380612, result.len());
}

fn next_generation(current: Vec<u8>) -> Vec<u8> {
    let mut next_generation: Vec<u8>;
    let mut new_lanternfish: Vec<u8> = Vec::with_capacity(current.len());

    next_generation = current
        .iter()
        .map(|fish| {
            if *fish != 0 {
                *fish - 1
            } else {
                new_lanternfish.push(8);
                6
            }
        })
        .collect();

    next_generation.append(&mut new_lanternfish);
    next_generation
}

fn input() -> Vec<u8> {
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
